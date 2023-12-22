use std::collections::HashMap;
use std::io::{Write, Result};
use std::net::TcpStream;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            StatusCode::OK => "200 OK",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
            
        })
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format the response as a string
        // Example:
        write!(f, "HTTP/1.1 {} {}\r\n", self.status_code, self.reason_phrase)?;
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        if let Some(body) = &self.body {
            write!(f, "\r\n{}", body)?;
        }
        Ok(())
    }
}

pub struct Response {
    status_code: StatusCode,
    reason_phrase: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status_code: StatusCode::OK,
            reason_phrase: "OK".to_string(), 
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn status(&mut self, code: StatusCode) -> &mut Self {
        self.status_code = code;
        self
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_string());
        self.set_header("Content-Length", &body.len().to_string());
        self
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status_code, self.reason_phrase
        );

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        if let Some(body) = &self.body {
            response.push_str(body);
        }

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}