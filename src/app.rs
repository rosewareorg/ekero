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

    fn handle(&self, mut ctx: Context) {
        let req = match ctx.request() {
            Ok(req) => req,
            Err(e) => {
                println!(
                    "Error parsing the request from {}: {e}",
                    ctx.request_address()
                );
                return;
            }
        };

        for (path, handler) in self.handlers.iter() {
            let handler = *handler;

            if *path == req.path {
                self.pool.execute(move || {
                    if let Err(res) = handler(ctx) {
                        log::error!("Cannot process a request: {res}")
                    }
                });
                break;
            }
        }
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

    pub fn add_handler(&mut self, path: impl Into<String>, handler: Handler) {
        self.handlers.insert(path.into(), handler);
    }
}
