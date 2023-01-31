use pyo3::prelude::*;
use std::net::{TcpListener, TcpStream};

mod http_request;
use http_request::HttpRequest;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn serve(port: u32) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port.to_string())).unwrap();
    println!("TCP server started on port {}", port.to_string());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let a = HttpRequest::from(stream);

        println!("{:?}", a);
    };
}

/// A Python module implemented in Rust.
#[pymodule]
fn beserk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(serve, m)?)?;
    Ok(())
}
