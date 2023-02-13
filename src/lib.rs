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

mod http; 
use http::Request;
use http::Response;

enum ListenerResult {
    RequestFound(TcpStream),
    RequestNotFound,
    KeyboardInterrupt
}

fn handle_connection(stream: TcpStream) {
    let mut a = Request::from(stream);
    let response = Response {
        status_code: 200,
        reason: "OK".to_string(),
        content_type: "text/html".to_string(),
        body: "<div>Aight cool</div>".to_string()
    }.to_string();
    a.stream.write_all(response.as_bytes()).unwrap();
    println!("{:#?}", a);
}

fn run_server(listener: &TcpListener, running: &Arc<AtomicBool>) -> ListenerResult {
    if running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let _ = listener.accept();
                std::thread::sleep(Duration::from_millis(100));
                ListenerResult::RequestFound(stream)
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                ListenerResult::RequestNotFound
            },
            _ => {
                ListenerResult::RequestNotFound
            }
        }
    } else {
        ListenerResult::KeyboardInterrupt
    }
}

#[pyfunction]
fn serve(port: u32, app: &PyAny) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    println!("{:?}", app.hasattr("api"));
    println!("TCP server started on port {}", port);
    loop {
        match run_server(&listener, &running) {
            ListenerResult::RequestFound(stream) => handle_connection(stream),
            ListenerResult::RequestNotFound => continue,
            ListenerResult::KeyboardInterrupt => break
        };
    };
}


#[pymodule]
fn granola(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
