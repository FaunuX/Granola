use pyo3::prelude::*;
use std::{
    io::{ErrorKind},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
    thread::sleep
}; 

mod http; 
use http::{
    Request,
    Response
};

mod net; 
use net::{
    Listener,
    Stream,
    Requesting,
    Serving
};

enum ListenerResult {
    RequestFound(Stream),
    RequestNotFound,
    KeyboardInterrupt 
}

enum RouteResult<T> {
    SubRoute(T),
    Failed
}

fn process_response(call: String, request: Request, app: &PyAny) -> (RouteResult<&PyAny>, Request) {
    (
        match app.call_method1(call.as_str(), request.clone()) {
            Ok (e) => RouteResult::SubRoute(e),
            Err(e) => {
                match app.call_method0(call.as_str()) {
                    Ok (e) => RouteResult::SubRoute(e),
                    Err(e) => {
                        println!("{}", e);
                        RouteResult::Failed
                    }
                }
            }
        },
        request
    )
}

fn process_request(call: String, request: Request, response: RouteResult<&PyAny>) -> (RouteResult<&PyAny>, Request) {
    match response {
        RouteResult::SubRoute(app) => {
            return process_response(call, request, app);
        },
        RouteResult::Failed => {
            panic!();
        }
    }
}

fn handle_connection(mut stream: Stream, app: &PyAny) {
    let mut request = Request::new(&stream);
    let mut response: RouteResult<&PyAny> = RouteResult::SubRoute(app);
    println!("{}", request.to_string());
    for call in request.clone().route.split('/').skip_while(|route| route.is_empty()).take_while(|route| !route.is_empty() ) {
        (response, request) = process_request(call.to_string(), request, response);
    };

    let result = if let RouteResult::SubRoute(resp) = response {
        if resp.hasattr("__granola__").unwrap() {
            resp.call_method0("__granola__").unwrap().str().unwrap().to_string()
        } else {
            resp.str().unwrap().to_string()
        }
    } else {
        "INTERNAL SERVER ERROR".to_string()
    };

    let response = Response {
        status_code: 200,
        reason: "OK".to_string(),
        content_type: "application/json".to_string(),
        body: result
    }.to_string();

    stream.respond(response).unwrap();
}

fn check_for_request(listener: &Listener) -> ListenerResult {
    match listener.check_for_requests() {
        Ok(stream) => {
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
}

fn run_server(listener: &Listener, running: &Arc<AtomicBool>) -> ListenerResult {
    if running.load(Ordering::SeqCst) {
        check_for_request(listener)
    } else {
        ListenerResult::KeyboardInterrupt
    }
}

#[pyfunction]
fn serve(port: u32, app: &PyAny) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let listener = Listener::connect(format!("127.0.0.1:{}", port));
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
#[pyo3(name="granola")]
fn granola(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
