use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpStream},
};

use crate::request::Request;

pub struct Context {
    stream: TcpStream,
    addr: IpAddr,
    request: Option<Request>,
}

impl Context {
    pub const fn new(stream: TcpStream, addr: IpAddr) -> Self {
        Self {
            stream,
            addr,
            request: None,
        }
    }

    #[inline]
    pub fn request_address(&self) -> IpAddr {
        self.addr
    }

    #[inline]
    pub fn is_request_local(&self) -> bool {
        self.addr.is_loopback()
    }

    fn read_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = [0; 8 * 1024];
        let ptr = self.read(&mut buffer)?;

        Ok(buffer[..ptr].to_vec())
    }

    pub fn request(&mut self) -> Result<Request, Box<dyn std::error::Error>> {
        match &self.request {
            Some(req) => Ok(req.clone()),
            None => {
                let bytes = self.read_bytes()?;
                Request::parse_from_bytes(bytes)
            }
        }
    }
}

impl Read for Context {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for Context {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
