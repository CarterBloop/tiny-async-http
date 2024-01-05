use crate::request::Request;
use crate::response::{Response, StatusCode};
use crate::router::Router;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

pub async fn handle_connection_async(stream: TcpStream, router: Arc<Router>) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);

    // Read the request line and headers
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;
    if request_line.is_empty() {
        let mut stream = reader.into_inner();
        return send_error_response_async(&mut stream, StatusCode::BadRequest, "Empty request line").await;
    }

    let mut headers = Vec::new();
    let mut content_length: Option<usize> = None;
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line).await?;
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
        Err(e) => {
            let mut stream = reader.into_inner();
            return send_error_response_async(&mut stream, StatusCode::BadRequest, e).await;
        }
    };

    // Read the body if Content-Length is specified
    let mut body = String::new();
    if let Some(length) = content_length {
        if length > 0 {
            let mut buffer = vec![0; length];
            reader.read_exact(&mut buffer).await?;
            body = String::from_utf8_lossy(&buffer).to_string();
        }
    }

    // Construct complete request object
    let request = Request {
        body: Some(body),
        ..request
    };

    // Route the request to the appropriate handler
    if let Some(handler) = router.route(&request) {
        let response = handler(&request);
        let mut stream = reader.into_inner();
        response.send_async(&mut stream).await?;
    } else {
        let mut stream = reader.into_inner();
        send_error_response_async(&mut stream, StatusCode::NotFound, "Not Found").await?
    }

    Ok(())
}

// Utility function to send error responses asynchronously
async fn send_error_response_async(
    stream: &mut TcpStream,
    status_code: StatusCode,
    reason_phrase: &str,
) -> std::io::Result<()> {
    let mut response = Response::new();
    response.status(status_code).set_body(reason_phrase);
    response.send_async(stream).await
}