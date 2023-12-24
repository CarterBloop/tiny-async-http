#[cfg(test)]
mod connection_tests {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::time::Duration;
    use HTTP::handle_connection;

    fn setup_test_server() -> TcpListener {
        TcpListener::bind("localhost:0").expect("Failed to bind to address")
    }

    #[test]
    fn handle_connection_response() {
        let listener = setup_test_server();
        let address = listener.local_addr().unwrap();

        let handle = std::thread::spawn(move || {
            loop {
                match listener.accept() {
                    Ok((stream, _)) => {
                        handle_connection(stream).expect("Failed to handle connection");
                        break;
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No connection yet, continue trying
                        std::thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => panic!("Server accept failed: {}", e),
                }
            }
        });

        // Give some time for the server to be ready
        std::thread::sleep(Duration::from_secs(1));

        let mut client = TcpStream::connect(address).expect("Failed to connect to server");
        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("Failed to write to server");

        let mut buffer = vec![];
        client
            .read_to_end(&mut buffer)
            .expect("Failed to read from server");

        let response = String::from_utf8(buffer).expect("Failed to convert response to string");
        assert!(response.contains("HTTP/1.1 200 OK"));

        // Ensure the server thread finishes
        handle.join().expect("Server thread panicked");
    }
}
