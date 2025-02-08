use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PoisonError;

impl Display for PoisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The mutex was poisoned")
    }
}

impl Error for PoisonError {}
