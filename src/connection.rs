use std::io::{Read, Result, Write};
use std::net::TcpStream;
use std::sync::Arc;

use crate::request::Request;
use crate::response::{Response, StatusCode};
use crate::router::Router;

pub fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> Result<()> {
    let mut buffer = [0; 1024];
    let mut total_read = 0;

    // Read the request from the stream
    loop {
        match stream.read(&mut buffer[total_read..]) {
            Ok(0) => break,  // End of stream reached
            Ok(n) => {
                total_read += n;
                if total_read >= 4 && buffer[total_read-4..total_read] == [13, 10, 13, 10] {
                    // Checks for "\r\n\r\n", indicating end of HTTP header
                    break;
                }
            },
            Err(e) => return Err(e),
        }

        if total_read == buffer.len() {
            // Buffer is full, but no end of header found
            return send_error_response(&mut stream, StatusCode::BadRequest, "Request Too Large");
        }
    }

    // Parse the request
    let raw_request = String::from_utf8(buffer[..total_read].to_vec()).unwrap();
    let request = match Request::parse(&raw_request) {
        Ok(req) => req,
        Err(_) => return send_error_response(&mut stream, StatusCode::BadRequest, "Bad Request"),
    };

    // Route the request to the appropriate handler
    if let Some(handler) = router.route(&request) {
        let response = handler(&request);
        response.send(&mut stream)
    } else {
        send_error_response(&mut stream, StatusCode::NotFound, "Not Found")
    }
}

// Utility function to send error responses
fn send_error_response(stream: &mut TcpStream, status_code: StatusCode, reason_phrase: &str) -> Result<()> {
    let mut response = Response::new();
    response.status(status_code)
            .set_body(reason_phrase);
    response.send(stream)
}