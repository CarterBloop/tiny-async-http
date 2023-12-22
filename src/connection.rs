use std::io::{Read, Write, Result};
use std::net::TcpStream;

use crate::request::Request;
use crate::response::Response;
use crate::header::Header;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];

    // Read the request from the TCP stream
    stream.read(&mut buffer)?;

    // Convert buffer to a string and parse the HTTP request
    let raw_request = String::from_utf8_lossy(&buffer);
    let request = match Request::parse(&raw_request) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            return send_error_response(&mut stream, 400, "Bad Request");
        }
    };

    // Process the request and prepare the response
    // This is where you would add more complex logic based on the request
    let mut response = Response::new(200, "OK");
    response.set_header("Content-Type", "text/plain");
    response.set_body("Hello, world!");

    // Send the response back to the client
    response.send(&mut stream)
}

// Utility function to send error responses
fn send_error_response(stream: &mut TcpStream, status_code: u16, reason_phrase: &str) -> Result<()> {
    let mut response = Response::new(status_code, reason_phrase);
    response.set_header("Content-Type", "text/plain");
    response.set_body(reason_phrase);
    response.send(stream)
}