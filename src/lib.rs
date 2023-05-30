use pyo3::prelude::*; 
use std::{
    io::{Error, ErrorKind},
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

#[derive(Debug)]
enum ListenerResult {
    RequestFound(Stream),
    RequestNotFound,
    KeyboardInterrupt 
}

enum RouteResult<T> {
    SubRoute(T),
    Failed
}

fn error(e: PyErr) -> String {
    let err: Error = e.into();
    println!("{:?}", err);
    panic!();
}

fn extract_path_from_url(url: &str) -> &str {
    match url.find('?') {
        Some(index) => &url[..index],
        None => url,
    }
}

fn is_valid_json(json: &RouteResult<&PyAny>) -> bool {
    let final_json = match json {
        RouteResult::Failed => return false,
        RouteResult::SubRoute(resp) => {
            if resp.hasattr("__granola__").unwrap() {
                resp.call_method0("__granola__").unwrap()
            } else {
                resp
            }
        }
    };
    match final_json.get_type().name() {
        Ok("str") => false,
        _ => true,
    }
}

fn process_response(call: String, request: Request, app: &PyAny) -> (RouteResult<&PyAny>, Request) {
    (
        match app.call_method1(call.as_str(), request.clone()) {
            Ok (e) => RouteResult::SubRoute(e),
            Err(e) => {
                println!("{}", e);
                if Error::from(e).to_string() == format!("TypeError: {}.{} missing 1 required argument", app.get_type().name().unwrap(), call) { 
                    match app.call_method0(call.as_str()) {
                        Ok (e) => RouteResult::SubRoute(e),
                        Err(e) => {
                            println!("{}", e);
                            RouteResult::Failed
                        }
                    }
                } else {
                    RouteResult::Failed
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

fn handle_connection(stream: Stream, app: &PyAny) {
    println!("handle_connection: {:?}", stream);
    let request_or_none = Request::new(&stream);
    let mut response: RouteResult<&PyAny> = RouteResult::SubRoute(app);
    let response = match request_or_none {
        Some(mut request) => {
            println!("{}", request.to_string());
            for call in request.clone().route.split('/').skip_while(|route| route.is_empty()).take_while(|route| !route.is_empty() ) {
                (response, request) = process_request(extract_path_from_url(call).to_string(), request, response);
            };

            let result = if let RouteResult::SubRoute(resp) = response {
                match resp.hasattr("__granola__") {
                    Ok(true) => {
                        match resp.call_method0("__granola__") {
                            Ok(e) => e.str().unwrap().to_string(),
                            Err(e) => error(e)
                        }
                    }
                    Ok(false) => resp.str().unwrap().to_string(),
                    Err(e) => error(e)

                } 
            } else {
                "INTERNAL SERVER ERROR".to_string()
            };

            let content_type = match is_valid_json(&response) {
                true => "application/json".to_string(),
                false => "text/html".to_string()
            };

            Response {
                status_code: 200,
                reason: "OK".to_string(),
                content_type,
                body: result
            }.to_string()
        },
        None => Response {
            status_code: 500,
            reason: "Resource not found".to_string(),
            content_type: "text/plain".to_string(),
            body: "RESOURCE NOT FOUND".to_string()
        }.to_string()
    };


    stream.respond(response).unwrap();
}

fn check_for_request(listener: &Listener) -> ListenerResult {
    match listener.check_for_requests() {
        Ok(stream) => {
            let _ = listener.accept();
            println!("{:?}", Request::new(&stream));
            sleep(Duration::from_millis(100));
            ListenerResult::RequestFound(stream)
        },
        Err(e) if e.kind() == ErrorKind::WouldBlock => {
            sleep(Duration::from_millis(100));
            ListenerResult::RequestNotFound
        },
        Err(e) => {
            println!("{}", e);
            ListenerResult::RequestNotFound
        }
    }
}

fn run_server(listener: &Listener, running: &Arc<AtomicBool>) -> ListenerResult {
    if running.load(Ordering::SeqCst) {
        let req = check_for_request(listener);
        println!("{:?}", req);
        req
    } else {
        ListenerResult::KeyboardInterrupt
    }
}

#[pyfunction]
fn serve(port: u32, app: &PyAny) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
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
