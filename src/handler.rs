use crate::{context::Context, prelude::Response};
use std::error::Error;

/// Any function which handles a context
pub type Handler<T> = fn(&mut Context<T>) -> Result<Response, Box<dyn Error>>;
