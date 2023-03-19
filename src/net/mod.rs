use std::io::Write;
use std::{
    io,
    net::{
        TcpStream,
        TcpListener
    },
};

pub trait Serving {
    type Stream;
    fn connect(address: String) -> Self;
    fn check_for_requests(&self) -> io::Result<Self::Stream>;
}

pub trait Requesting {
    fn respond(self, response: String) -> io::Result<()>;
}

pub type Listener = TcpListener;
pub type Stream = TcpStream;

impl Serving for Listener {
    type Stream = Stream;
    fn connect(address: String) -> Self {
        let return_value = Self::bind(address).unwrap();
        return_value.set_nonblocking(true);
        return_value
    }
    fn check_for_requests(&self) -> io::Result<Self::Stream> {
        match self.accept() {
            Ok((stream, _)) => Ok(stream),
            Err(e) => Err(e)
        }
    }
}

impl Requesting for Stream {
    fn respond(mut self, response: String) -> io::Result<()> {
        self.write_all(response.as_bytes())
    }
}
