use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    host: String,
    route: String,
    version: f32
}

impl From<TcpStream> for HttpRequest {
    fn from(stream: TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let http_request_data: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        Self {
            method: http_request_data[0].split_whitespace().collect::<Vec<_>>()[0].to_string(),
            host: http_request_data[1].split_whitespace().collect::<Vec<_>>()[1].to_string(),
            route: http_request_data[0].split_whitespace().collect::<Vec<_>>()[1].to_string(),
            version: http_request_data[0].split("/").collect::<Vec<_>>().last().expect("1.1").parse().unwrap()
        }

    }
