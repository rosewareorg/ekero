use httparse as http;
use std::collections::HashMap;
use std::error;
use std::fmt;

// An HTTP method
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Patch,
}

#[derive(Debug)]
pub struct UnknownMethod {
    method: String,
}

impl fmt::Display for UnknownMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown HTTP method: {}", self.method)
    }
}

impl error::Error for UnknownMethod {}

impl<'a> TryFrom<&'a str> for Method {
    type Error = UnknownMethod;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "HEAD" => Ok(Self::Head),
            "PATCH" => Ok(Self::Patch),
            _ => Err(UnknownMethod {
                method: value.to_owned(),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: u8,
    pub headers: HashMap<String, Vec<u8>>,
    pub body: Option<Vec<u8>>,
    pub http_query: HashMap<String, String>,
}

pub fn parse_query(string: String, query: &mut HashMap<String, String>) {
    for pair in string.split('&') {
        let mut it = pair.split('=').take(2);
        let kv = match (it.next(), it.next()) {
            (Some(k), Some(v)) => (k.to_owned(), v.to_owned()),
            _ => continue,
        };

        query.insert(kv.0, kv.1);
    }
}

impl Request {
    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn error::Error>> {
        let mut headers = [http::EMPTY_HEADER; 200];
        let mut req = http::Request::new(&mut headers);

        let http::Status::Complete(data_location) = req.parse(bytes)? else {
            return Err("Failed parsing the request".into());
        };

        let body = (data_location != bytes.len()).then(|| bytes[data_location..].to_vec());

        let headers: HashMap<_, _> = req
            .headers
            .iter()
            .map(|header| (header.name.to_owned(), header.value.to_vec()))
            .collect();

        let method = req.method.map_or(Ok(Method::Get), Method::try_from)?;
        let mut path = req.path.unwrap_or("/").to_owned();
        let mut http_query = HashMap::new();

        if let Some(index) = path.find('?') {
            parse_query(path[index..].to_owned(), &mut http_query);
            path = path[0..index].to_owned();
        }

        Ok(Self {
            method,
            headers,
            body,
            version: req.version.unwrap_or_default(),
            path,
            http_query,
        })
    }
}
