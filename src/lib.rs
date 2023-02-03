use pyo3::prelude::*;
use std::{
    io::{ErrorKind, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
}; 
mod http_request; use http_request::HttpRequest;



fn handle_connection(stream: TcpStream) {
    let mut a = HttpRequest::from(stream);
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    a.stream.write_all(response.as_bytes()).unwrap();
    println!("{:#?}", a);
}


fn run_server(listener: TcpListener, running: &Arc<AtomicBool>) -> Option<TcpListener> {
    if running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let _ = listener.accept();
                handle_connection(stream);
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    std::thread::sleep(Duration::from_millis(100));
                }; 
            }
        };
        Some(listener)
    } else {
        None
    }
}


#[pyfunction]
fn serve(port: u32) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    println!("TCP server started on port {}", port);
    loop {
        match run_server(listener, &running) {
            Some(Listener) => listener = Listener,
            None => break
        };
    };

}


#[pymodule]
fn beserk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
