use std::{collections::HashMap, fmt, io, io::Write, net::TcpStream};

use crate::resource::Resource;

#[non_exhaustive]
pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<&'static str, WritableValue>,
    pub message_body: Option<Box<dyn Resource>>,
}

/// A type which can be a value of a header
#[derive(Clone)]
#[non_exhaustive]
pub enum WritableValue {
    String(String),
    Number(usize),
    StaticString(&'static str),
}

impl From<String> for WritableValue {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}

impl From<&'static str> for WritableValue {
    fn from(val: &'static str) -> Self {
        Self::StaticString(val)
    }
}

impl From<usize> for WritableValue {
    fn from(val: usize) -> Self {
        Self::Number(val)
    }
}

impl fmt::Display for WritableValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(s) => write!(f, "{s}"),
            Self::StaticString(s) => write!(f, "{s}"),
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            status_code: 200,
            headers: HashMap::new(),
            message_body: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn status_code(mut self, code: u16) -> Self {
        self.status_code = code;
        self
    }

    #[inline]
    #[must_use]
    pub fn header<T: Into<WritableValue>>(mut self, header: &'static str, data: T) -> Self {
        self.headers.insert(header, data.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn body(mut self, data: impl Resource + 'static) -> Self {
        self = data.add_headers(self);
        self.message_body = Some(Box::new(data));
        self
    }
}

fn status_code_as_string(code: u16) -> &'static str {
    match code {
        100 => "Continue",
        101 => "Switching Protocols",
        102 => "Processing",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        207 => "Multi-Status",
        208 => "Already Reported",
        226 => "IM Used",
        300 => "Multiple Choices",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        304 => "Not Modified",
        305 => "Use Proxy",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        402 => "Payment Required",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        406 => "Not Acceptable",
        407 => "Proxy Authentication Required",
        408 => "Request Timeout",
        409 => "Conflict",
        410 => "Gone",
        411 => "Length Required",
        412 => "Precondition Failed",
        413 => "Payload Too Large",
        414 => "URI Too Long",
        415 => "Unsupported Media Type",
        416 => "Range Not Satisfiable",
        417 => "Expectation Failed",
        418 => "I'm a teapot",
        421 => "Misdirected Request",
        422 => "Unprocessable Entity",
        423 => "Locked",
        424 => "Failed Dependency",
        425 => "Too Early",
        426 => "Upgrade Required",
        428 => "Precondition Required",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        451 => "Unavailable For Legal Reasons",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        505 => "HTTP Version Not Supported",
        506 => "Variant Also Negotiates",
        507 => "Insufficient Storage",
        508 => "Loop Detected",
        510 => "Not Extended",
        511 => "Network Authentication Required",
        _ => panic!("Unknown status code!"),
    }
}

impl Response {
    pub fn write_to(&self, source: &mut TcpStream) -> io::Result<()> {
        write!(
            source,
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            status_code_as_string(self.status_code)
        )?;

        for (name, data) in self.headers.iter() {
            write!(source, "{name}: {data}")?;
            source.write(b"\r\n")?;
        }

        source.write(b"\r\n")?;

        if let Some(ref body) = self.message_body {
            body.write_to_stream(source)?;
        }

        source.write(b"\r\n")?;

        Ok(())
    }
}
