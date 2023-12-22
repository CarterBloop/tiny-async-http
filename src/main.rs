use std::net::TcpListener;
use std::thread;

mod request;
mod response;
mod header;
mod connection;

fn main() -> std::io::Result<()> {
    // Specify the address and port to listen on
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;

    println!("HTTP server running on http://{}", address);

    // Accept incoming TCP connections and handle them in separate threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread for each connection
                thread::spawn(move || {
                    if let Err(e) = connection::handle_connection(stream) {
                        eprintln!("Failed to handle connection: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}