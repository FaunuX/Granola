use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub host: String,
    pub route: String,
    pub version: f32,
    pub stream: TcpStream,
}

impl From<TcpStream> for Request {
    fn from(stream: TcpStream) -> Self {
        let buf_reader = BufReader::new(&stream);
        let http_request_data: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        Self {
            method: http_request_data[0].split_whitespace().collect::<Vec<_>>()[0].to_string(),
            host: http_request_data[1].split_whitespace().collect::<Vec<_>>()[1].to_string(),
            route: http_request_data[0].split_whitespace().collect::<Vec<_>>()[1].to_string(),
            version: http_request_data[0]
                .split('/')
                .collect::<Vec<_>>()
                .last()
                .expect("1.1")
                .parse()
                .unwrap(),
            stream,
        }
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
