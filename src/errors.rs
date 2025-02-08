use std::{error::Error, fmt::Display};

/// Signifies that a mutex was poisoned. Used to omit local references when mutex.lock() return an error
#[derive(Debug)]
pub struct PoisonError;

impl Display for PoisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The mutex was poisoned")
    }
}

impl Error for PoisonError {}
