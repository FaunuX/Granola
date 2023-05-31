use pyo3::{
    prelude::*,
    types::*
};

use std::{
    io::{prelude::*, BufReader},
};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub host: String,
    pub route: String,
    pub version: f32,
}

impl Request {
    // pub fn new(mut stream: &Stream) -> Option<Self> {
    //     let mut buf = String::new();
    //     stream.read_to_string(&mut buf);
    //     println!("buf: {buf}");
    //     let buf_reader = BufReader::new(&mut stream);
    //     let http_request_headers: Vec<_> = buf_reader
    //         .lines()
    //         .map_while(|line_result| match line_result {
    //                 Ok(line) => if line.is_empty() {
    //                     Some(line)
    //                 } else {
    //                     None
    //                 }
    //                 Err(e) => {
    //                     println!("{}", e);
    //                     None
    //                 }
    //             }
    //         )
    //         .collect();

    //     if http_request_headers.len() == 0 {
    //         return None;
    //     }

    //     let method = http_request_headers[0]
    //         .split_whitespace().collect::<Vec<_>>()[0].to_string();
    //     let host =  http_request_headers[1]
    //         .split_whitespace().collect::<Vec<_>>()[1].to_string();
    //     let route =  http_request_headers[0]
    //         .split_whitespace().collect::<Vec<_>>()[1].to_string();
    //     let version = http_request_headers[0] 
    //         .split('/').collect::<Vec<_>>().last().expect("1.1").parse().unwrap();

    //     let mut http_request_iter = http_request_headers.iter();

    //     let content_length_str = loop {
    //         let row = match http_request_iter.next() {
    //             Some(val) => val,
    //             None => break None,
    //         };
    //         if row.starts_with(&String::from("Content-Length")) {
    //             let content_length = row.split("Content-Length: ").collect::<Vec<_>>()[1];
    //             break Some(content_length)
    //         }
    //     };

    //     let content_length = if content_length_str.is_some() {
    //         match content_length_str.expect("Congrats! You broke the universe!").parse::<u8>() {
    //             Ok(content_length) => content_length,
    //             Err(err) => panic!("Error parsing content length")
    //         }
    //     } else {
    //         0
    //     };

    //     // println!("{:#?}", stream.read(&mut [content_length]));

    //     Some(
    //         Self {
    //             method,
    //             host,
    //             route,
    //             version,
    //         }
    //     )
    // }
}

impl IntoPy<Py<PyTuple>> for Request {
    fn into_py(self, py: Python<'_>) -> Py<PyTuple> {
        let dict = PyDict::new(py);
        dict.set_item("method" , &self.method ).unwrap();
        dict.set_item("host"   , &self.host   ).unwrap();
        dict.set_item("route"  , &self.route  ).unwrap();
        dict.set_item("version", &self.version).unwrap();
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
