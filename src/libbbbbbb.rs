use pyo3::prelude::*;

use std::net::TcpListener;
use std::io::{Error, Read, Write};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use ctrlc;

mod http_request;
use http_request::HttpRequest;

fn run_server(exit: Arc<AtomicBool>) -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        if exit.load(Ordering::SeqCst) {
            break;
        }
        let stream = stream?;
        let exit = exit.clone();
        thread::spawn(move || {
            handle_client(stream, &exit);
        });
    }
    Ok(())
}

fn handle_client(mut stream: std::net::TcpStream, exit: &AtomicBool) {
    let mut data = [0 as u8; 50];
    while !exit.load(Ordering::SeqCst) {
        match stream.read(&mut data) {
            Ok(size) => {
                let message = &data[0..size];
                println!("Received message: {}", String::from_utf8_lossy(message));
                stream.write(message).unwrap();
            },
            Err(_) => break
        }
    }
}

#[pyfunction]
fn serve() {
    let exit = Arc::new(AtomicBool::new(false));
    let exit_clone = exit.clone();
    ctrlc::set_handler(move || {
        exit_clone.store(true, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    if let Err(e) = run_server(exit) {
        println!("Error: {}", e);
    } else {
        println!("Success!")
    }
}


#[pymodule]
fn beserk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
