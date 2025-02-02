use std::net::TcpListener;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use threadpool::ThreadPool;

use crate::context::Context;

pub struct App {
    listener: TcpListener,
    sender: Sender<Context>,
    thread_pool: ThreadPool,
}

impl App {
    /*
        pub fn new(path: &str, num_threads: usize) -> Self {
            let listener = TcpListener::bind(path).expect("Cannot create the listener");
            let (sender, receiver) = channel::<Context>();
            let thread_pool = ThreadPool::new(num_threads);

            let receiver = Arc::new(receiver);

            Self {
                listener,
                sender,
                thread_pool,
            }
        }

        pub fn poll_once(&self) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    let ctx = Context::new(stream, addr.ip());
                    self.try_send_job(ctx);
                }
                Err(e) => {
                    log::error!("Cannot accept the next request: {e}")
                }
            }
        }

        fn try_send_job(&self, job: Context) {
            if let Err(e) = self.sender.send(job) {
                log::error!("Cannot process the next request: {e}")
            }
        }

        pub fn poll_forever(&self) -> ! {
            loop {
                self.poll_once()
            }
        }
    */

    // TODO
    // Figure out how to structure this whole polling thing
}
