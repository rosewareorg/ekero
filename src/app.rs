/* TODO */

use crate::{context::Context, handler::Handler, request::Method, threadpool::ThreadPool};
use std::{
    collections::HashMap,
    io,
    net::{TcpListener, ToSocketAddrs},
};

pub struct App {
    listener: TcpListener,
    pool: ThreadPool,
    handlers: HashMap<(String, Method), Handler>,
    default_handler: Option<Handler>,
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
            default_handler: None,
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

        let req = (req.path, req.method);

        if let Some(handler) = self.handlers.get(&req) {
            self.send_to_thread_pool(ctx, handler);
        } else if let Some(ref handler) = self.default_handler {
            self.send_to_thread_pool(ctx, handler);
        } else {
            log::error!("No handler found for {:?} {}", req.1, req.0);
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

    pub fn add_handler(&mut self, path: impl Into<String>, method: Method, handler: Handler) {
        self.handlers.insert((path.into(), method), handler);
    }

    pub fn get(&mut self, path: impl Into<String>, handler: Handler) {
        self.add_handler(path, Method::Get, handler);
    }

    pub fn post(&mut self, path: impl Into<String>, handler: Handler) {
        self.add_handler(path, Method::Post, handler);
    }

    pub fn set_default_handler(&mut self, handler: Handler) {
        self.default_handler = Some(handler);
    }

    fn send_to_thread_pool(&self, ctx: Context, handler: &Handler) {
        let handler = handler.clone();
        self.pool.execute(move || {
            if let Err(res) = handler(ctx) {
                log::error!("Cannot process a request: {res}")
            }
        });
    }
}
