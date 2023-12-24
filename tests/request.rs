#[cfg(test)]
mod request_tests {

    use HTTP::request::{Method, Request};

    #[test]
    fn parse_valid_request() {
        let raw_request = "GET /index.html HTTP/1.1\r\nhost: example.com\r\n\r\n";
        let request = Request::parse(raw_request).expect("Failed to parse");

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.uri, "/index.html");
        assert_eq!(request.http_version, "HTTP/1.1");
        assert!(request.headers.contains_key("host"));
        assert_eq!(request.headers.get("host").unwrap(), "example.com");
    }

    #[test]
    fn parse_invalid_request() {
        let raw_request = "INVALID REQUEST";
        assert!(Request::parse(raw_request).is_err());
    }

    // Add more tests covering different request scenarios
}
