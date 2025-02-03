use crate::context::Context;
use std::error::Error;

pub type Handler = fn(Context) -> Result<(), Box<dyn Error>>;
