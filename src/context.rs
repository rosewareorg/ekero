use std::{
    io::{Read, Write},
    net::{IpAddr, TcpStream},
};

pub struct Context {
    stream: TcpStream,
    addr: IpAddr,
}

impl Context {
    pub const fn new(stream: TcpStream, addr: IpAddr) -> Self {
        Self { stream, addr }
    }

    #[inline]
    pub fn request_address(&self) -> IpAddr {
        self.addr
    }

    #[inline]
    pub fn is_request_local(&self) -> bool {
        self.addr.is_loopback()
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
