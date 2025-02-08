use crate::prelude::Response;
use std::{
    io::{self, Write},
    net::TcpStream,
};

pub trait Resource {
    fn add_headers(&self, source: Response) -> Response;
    fn write_to_stream(&self, stream: &mut TcpStream) -> io::Result<()>;
}

impl Resource for &'static str {
    fn add_headers(&self, source: Response) -> Response {
        source.header("Content-Length", self.len())
    }

    fn write_to_stream(&self, stream: &mut TcpStream) -> io::Result<()> {
        write!(stream, "{self}")
    }
}

impl Resource for String {
    fn add_headers(&self, source: Response) -> Response {
        source.header("Content-Length", self.len())
    }

    fn write_to_stream(&self, stream: &mut TcpStream) -> io::Result<()> {
        write!(stream, "{self}")
    }
}

impl<'a> Resource for &'a [u8] {
    #[inline]
    fn add_headers(&self, source: Response) -> Response {
        source
    }

    fn write_to_stream(&self, stream: &mut TcpStream) -> io::Result<()> {
        let _size = stream.write(self)?;
        Ok(())
    }
}
