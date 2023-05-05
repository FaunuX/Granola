use pyo3::prelude::*;
use std::{
    io::{Error, ErrorKind},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
    thread::sleep,
};

mod http;
use http::{Request, Response};

mod net;
use net::{Listener, Stream, Requesting, Serving};

enum ListenerResult {
    RequestFound(Stream),
    RequestNotFound,
    KeyboardInterrupt,
}

enum RouteResult<T> {
    SubRoute(T),
    Failed,
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
            if resp.hasattr("__granola__").unwrap_or_else(|_| false) {
                match resp.call_method0("__granola__") {
                    Ok(json) => json,
                    Err(_) => return false,
                }
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

fn process_response(
    call: String,
    request: Request,
    app: &PyAny,
    ) -> (Result<RouteResult<&PyAny>, PyErr>, Request) {
    (
        app.call_method1(call.as_str(), request.clone()).map_or_else(
            |e| {
                println!("{}", e);
                if Error::from(e).to_string()
                    == format!("TypeError: {}.{} missing 1 required argument", app.get_type().name().unwrap(), call)
                    {
                        app.call_method0(call.as_str()).map_or_else(
                            |e| {
                                println!("{}", e);
                                Err(e.into())
                            },
                            |e| Ok(RouteResult::SubRoute(e)),
                            )
                    } else {
                        Err(e.into())
                    }
            },
            |e| Ok(RouteResult::SubRoute(e)),
            ),
            request,
            )
}

