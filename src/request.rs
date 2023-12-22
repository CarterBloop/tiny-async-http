use std::collections::HashMap;

pub struct Request {
    method: String,
    uri: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub fn parse(raw_request: &str) -> Result<Request, &'static str> {
        let mut lines = raw_request.lines();

        // Parse the request line
        let request_line = lines.next().ok_or("Request line missing")?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or("Method missing")?.to_string();
        let uri = parts.next().ok_or("URI missing")?.to_string();
        let http_version = parts.next().ok_or("HTTP version missing")?.to_string();

        // Parse headers
        let mut headers = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }
            let mut parts = line.splitn(2, ':');
            let key = parts.next().ok_or("Header key missing")?.trim().to_string();
            let value = parts.next().ok_or("Header value missing")?.trim().to_string();
            headers.insert(key, value);
        }

        // Parse body if present
        let body = lines.next().map(|s| s.to_string());

        Ok(Request {
            method,
            uri,
            http_version,
            headers,
            body,
        })
    }
}