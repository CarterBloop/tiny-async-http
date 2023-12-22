use std::io::{Write, Result};
use std::net::TcpStream;
use std::collections::HashMap;

pub struct Response {
    status_code: u16,
    reason_phrase: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    // Create a new Response object with a status code and a reason phrase
    pub fn new(status_code: u16, reason_phrase: &str) -> Self {
        Response {
            status_code,
            reason_phrase: reason_phrase.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    // Add or update a header
    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    // Set the response body
    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    // Send the response to the client
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