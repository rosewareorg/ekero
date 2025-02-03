/* TODO */

use crate::{context::Context, handler::Handler, threadpool::ThreadPool};
use std::{
    collections::HashMap,
    io,
    net::{TcpListener, ToSocketAddrs},
};

pub struct App {
    listener: TcpListener,
    pool: ThreadPool,
    handlers: HashMap<String, Handler>,
}

impl App {
    pub fn new<P>(path: P, threads: usize) -> Self
    where
        P: ToSocketAddrs,
    {
        let listener = TcpListener::bind(path).expect("Cannot bind the listener");
        let pool = ThreadPool::new(threads);
        let handlers = HashMap::new();

        Self {
            listener,
            pool,
            handlers,
        }
    }

    fn receive_next(&self) -> io::Result<Context> {
        let next = self.listener.accept()?;
        let ctx = Context::new(next.0, next.1.ip());
        Ok(ctx)
    }

    fn handle(&self, ctx: Context) {
        todo!()
    }

    pub fn poll_once(&self) {
        let next = self.receive_next();
        match next {
            Ok(data) => self.handle(data),
            Err(e) => {
                log::error!("Cannot receive the next client: {e}");
                return;
            }
        }
    }

    pub fn poll_forever(&self) -> ! {
        loop {
            self.poll_once();
        }
    }
}
