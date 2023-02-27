use pyo3::prelude::*;
use std::{
    io::{ErrorKind, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
    thread::sleep
}; 

mod http; 
use http::Request;
use http::Response;

enum ListenerResult {
    RequestFound(TcpStream),
    RequestNotFound,
    KeyboardInterrupt
}

enum RouteResult<T> {
    SubRoute(T),
    Failed
}

fn handle_connection(stream: TcpStream, app: &PyAny) {
    let mut request = Request::from(stream);
    let mut response: RouteResult<&PyAny> = RouteResult::SubRoute(app);
    println!("{:#?}", request.route.split('/').collect::<Vec<&str>>());
    for call in request.route.split('/').skip_while(|route| route.is_empty()).take_while(|route| !route.is_empty() ) {
        match response {
            RouteResult::SubRoute(app) => {
                response = match app.call_method0(call) {
                    Ok(e) => {
                        println!("{:?}", e);
                        RouteResult::SubRoute(e)
                    },
                    Err(_) => {
                        RouteResult::Failed 
                    }

                };
            },
            RouteResult::Failed => {
                panic!();
            }
        }
    };
    let result = if let RouteResult::SubRoute(resp) = response {
        resp.str().unwrap().to_string()
    } else {
        "INTERNAL SERVER ERROR".to_string()
    };

    let response = Response {
        status_code: 200,
        reason: "OK".to_string(),
        content_type: "application/json".to_string(),
        body: result
    }.to_string();

    request.stream.write_all(response.as_bytes()).unwrap();
    println!("{:#?}", response);
}

fn run_server(listener: &TcpListener, running: &Arc<AtomicBool>) -> ListenerResult {
    if running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                let _ = listener.accept();
                sleep(Duration::from_millis(100));
                ListenerResult::RequestFound(stream)
            },
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                sleep(Duration::from_millis(100));
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
    println!("TCP server started on port {}, Ctrl+C to quit", port);
    loop {
        match run_server(&listener, &running) {
            ListenerResult::RequestFound(stream) => handle_connection(stream, app),
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
