use std::io::{Read, Write, BufReader, BufRead};
use std::net::TcpStream;
use std::sync::Arc;
use crate::request::Request;
use crate::response::{Response, StatusCode};
use crate::router::Router;

pub fn handle_connection(mut stream: TcpStream, router: Arc<Router>) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    // Read the request line and headers
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    if request_line.is_empty() {
        return send_error_response(&mut stream, StatusCode::BadRequest, "Empty request line");
    }

    let mut headers = Vec::new();
    let mut content_length: Option<usize> = None;
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line)?;
        if header_line == "\r\n" {
            break;
        }

        if let Some((key, value)) = header_line.split_once(':') {
            if key.trim().eq_ignore_ascii_case("Content-Length") {
                content_length = value.trim().parse().ok();
            }
            headers.push((key.trim().to_string(), value.trim().to_string()));
        }
    }

    // Parse request line and headers
    let request = match Request::from_request_line_and_headers(&request_line, headers) {
        Ok(req) => req,
        Err(e) => return send_error_response(&mut stream, StatusCode::BadRequest, e),
    };

    // Read the body if Content-Length is specified
    let mut body = String::new();
    if let Some(length) = content_length {
        if length > 0 {
            let mut body_reader = reader.take(length as u64);
            body_reader.read_to_string(&mut body)?;
        }
    }

    // Construct complete request object
    let mut request = Request {
        body: Some(body),
        ..request
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
fn send_error_response(stream: &mut TcpStream, status_code: StatusCode, reason_phrase: &str) -> std::io::Result<()> {
    let mut response = Response::new();
    response.status(status_code)
            .set_body(reason_phrase);
    response.send(stream)
}