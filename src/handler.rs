use crate::context::Context;
use std::error::Error;

pub type Handler<T> = fn(&mut Context<T>) -> Result<(), Box<dyn Error>>;
