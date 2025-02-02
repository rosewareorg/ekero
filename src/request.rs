use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: u8,
    pub headers: HashMap<String, Vec<u8>>,
}
