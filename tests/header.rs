#[cfg(test)]
mod header_tests {
    use HTTP::Header;

    #[test]
    fn header_insertion_and_retrieval() {
        let mut headers = Header::new();
        headers.add("Content-Type", "application/json");
        headers.add("Accept", "text/html");

        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("accept").unwrap(), "text/html"); // Testing case insensitivity
    }

    // More tests for parsing and other functionalities
}
