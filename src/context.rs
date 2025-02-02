use super::{request::Request, response::Response};
use std::{io::Write, net::TcpStream};

pub struct Context<'a> {
    stream: &'a TcpStream,
}

impl<'a> Context<'a> {
    pub fn new(stream: &'a TcpStream) -> Self {
        Self { stream }
    }

    pub fn request(&self) -> Request {
        todo!()
    }

    pub fn send_response(&mut self, response: Response) {
        let bytes = response.to_bytes();
        let _res = self.stream.write(bytes);
        todo!()
    }
}
