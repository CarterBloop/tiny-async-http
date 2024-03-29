use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::GET => "GET",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::DELETE => "DELETE",
                Method::HEAD => "HEAD",
                Method::CONNECT => "CONNECT",
                Method::OPTIONS => "OPTIONS",
                Method::TRACE => "TRACE",
                Method::PATCH => "PATCH",
            }
        )
    }
}

impl FromStr for Method {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err("Invalid HTTP method"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_request_line_and_headers(
        request_line: &str,
        header_lines: Vec<(String, String)>,
    ) -> Result<Request, &'static str> {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid request line");
        }

        let method = parts[0].parse()?;
        let uri = parts[1].to_string();
        let http_version = parts[2].to_string();

        let mut headers = HashMap::new();
        for (key, value) in header_lines {
            headers.insert(key, value);
        }

        Ok(Request {
            method,
            uri,
            http_version,
            headers,
            body: None, // Body is initially None
        })
    }
}
