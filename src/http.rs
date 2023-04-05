use pyo3::{
    prelude::*,
    types::*
};

use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub host: String,
    pub route: String,
    pub version: f32,
    pub body: String,
}

impl Request {
    pub fn new(mut stream: &TcpStream) -> Self {
        let buf_reader = BufReader::new(&mut stream);
        let http_request_data: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let method = http_request_data[0]
            .split_whitespace().collect::<Vec<_>>()[0].to_string();
        let host =  http_request_data[1]
            .split_whitespace().collect::<Vec<_>>()[1].to_string();
        let route =  http_request_data[0]
            .split_whitespace().collect::<Vec<_>>()[1].to_string();
        let version = http_request_data[0] 
            .split('/').collect::<Vec<_>>().last().expect("1.1").parse().unwrap();
        let body = http_request_data.last().unwrap().to_string();

        Self {
            method,
            host,
            route,
            version,
            body
        }
    }
}

impl IntoPy<Py<PyTuple>> for Request {
    fn into_py(self, py: Python<'_>) -> Py<PyTuple> {
        let dict = PyDict::new(py);
        dict.set_item("method" , &self.method ).unwrap();
        dict.set_item("host"   , &self.host   ).unwrap();
        dict.set_item("route"  , &self.route  ).unwrap();
        dict.set_item("version", &self.version).unwrap();
        dict.set_item("body", &self.body).unwrap();
        PyTuple::new(py, [dict]).into_py(py)
    }
}

impl ToString for Request {
    fn to_string(&self) -> String {
        format!("{}: {} {} | {}", self.method, self.host, self.route, self.version)
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub reason: String,
    pub content_type: String,
    pub body: String
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status_code, 
            self.reason, 
            self.content_type, 
            self.body
            )
    }
}
