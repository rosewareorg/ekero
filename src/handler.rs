/* TODO */

use std::error::Error;

use crate::context::Context;

pub type Handler = fn(Context) -> Result<(), Box<dyn Error>>;
