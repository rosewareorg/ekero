use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpStream},
    sync::{Arc, Mutex, MutexGuard},
};

use crate::{errors::PoisonError, request::Request};

pub struct Context<T> {
    stream: TcpStream,
    addr: IpAddr,
    request: Option<Request>,

    pub state: Arc<Mutex<T>>,
}

impl<T> Context<T> {
    pub const fn new(stream: TcpStream, addr: IpAddr, state: Arc<Mutex<T>>) -> Self {
        Self {
            stream,
            addr,
            request: None,
            state,
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
        let mut buffer = [0; 30 * 1024];
        let ptr = self.read(&mut buffer)?;

        Ok(buffer[..ptr].to_vec())
    }

    pub fn request(&mut self) -> Result<Request, Box<dyn std::error::Error>> {
        match &self.request {
            Some(req) => Ok(req.clone()),
            None => {
                let bytes = self.read_bytes()?;
                let req = Request::parse_from_bytes(bytes)?;
                self.request = Some(req.clone());
                Ok(req)
            }
        }
    }

    pub fn state_lock(&self) -> Result<MutexGuard<'_, T>, PoisonError> {
        self.state.lock().map_err(|_e| PoisonError)
    }
}

impl<T> Read for Context<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<T> Write for Context<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}
