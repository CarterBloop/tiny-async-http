#[cfg(test)]
mod response_tests {
    use HTTP::response::{Response, StatusCode};

    #[test]
    fn response_formatting() {
        let mut response = Response::new(StatusCode::OK);
        response.set_header("Content-Type", "text/plain");
        response.set_body("Hello, World!");

        let formatted_response = format!("{}", response);

        assert!(formatted_response.contains("HTTP/1.1 200 OK"));
        assert!(formatted_response.contains("Content-Type: text/plain"));
        assert!(formatted_response.contains("Hello, World!"));
    }

    // Additional tests for various response scenarios
}
