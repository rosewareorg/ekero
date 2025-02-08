use crate::prelude::Response;
use std::fmt::Display;

pub trait Resource: Display {
    fn add_headers(&self, source: Response) -> Response;
}

impl Resource for &'static str {
    fn add_headers(&self, source: Response) -> Response {
        source.header("Content-Length", self.len())
    }
}

impl Resource for String {
    fn add_headers(&self, source: Response) -> Response {
        source.header("Content-Length", self.len())
    }
}
