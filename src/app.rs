/* TODO */

use crate::{context::Context, handler::Handler, request::Method, threadpool::ThreadPool};
use std::{
    collections::HashMap,
    io,
    net::{TcpListener, ToSocketAddrs},
    sync::{Arc, Mutex},
};

pub struct App<T> {
    listener: TcpListener,
    pool: ThreadPool,
    handlers: HashMap<(String, Method), Handler<T>>,
    default_handler: Option<Handler<T>>,

    state: Arc<Mutex<T>>,
}

impl<T: Send + 'static> App<T> {
    pub fn new<P>(path: P, threads: usize, state: T) -> Self
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
            state: Arc::new(Mutex::new(state)),
        }
    }

    fn receive_next(&self) -> io::Result<Context<T>> {
        let next = self.listener.accept()?;
        let ctx = Context::new(next.0, next.1.ip(), Arc::clone(&self.state));
        Ok(ctx)
    }

    fn handle(&self, mut ctx: Context<T>) {
        let req = match ctx.request() {
            Ok(req) => req,
            Err(e) => {
                log::error!(
                    "Error parsing the request from {}: {e}",
                    ctx.request_address()
                );
                return;
            }
        };

        let req = (req.path, req.method);

        if let Some(handler) = self.handlers.get(&req) {
            self.send_to_thread_pool(ctx, *handler);
        } else if let Some(handler) = self.default_handler.as_ref() {
            self.send_to_thread_pool(ctx, *handler);
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
            }
        }
    }

    pub fn poll_forever(&self) -> ! {
        loop {
            self.poll_once();
        }
    }

    pub fn add_handler<S: Into<String>>(&mut self, path: S, method: Method, handler: Handler<T>) {
        self.handlers.insert((path.into(), method), handler);
    }

    pub fn get<S: Into<String>>(&mut self, path: S, handler: Handler<T>) {
        self.add_handler(path, Method::Get, handler);
    }

    pub fn post<S: Into<String>>(&mut self, path: S, handler: Handler<T>) {
        self.add_handler(path, Method::Post, handler);
    }

    pub fn set_default_handler(&mut self, handler: Handler<T>) {
        self.default_handler = Some(handler);
    }

    fn send_to_thread_pool(&self, mut ctx: Context<T>, handler: Handler<T>) {
        self.pool.execute(move || match handler(&mut ctx) {
            Ok(response) => {
                let res = response.write_to(&mut ctx.stream);
                if let Err(e) = res {
                    log::error!("Cannot write the response to stream: {e}")
                }
            }
            Err(res) => {
                if res.to_string() == "The mutex was poisoned" {
                    ctx.state.clear_poison();
                }
                log::error!("Cannot process a request: {res}")
            }
        });
    }
}
