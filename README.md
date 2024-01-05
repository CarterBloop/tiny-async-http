# tiny-async-http

`tiny-async-http` is a lightweight and asynchronous HTTP server library for Rust. Inspired by the simplicity and ease of use of express.js, this library aims to provide a straightforward approach to building HTTP servers in Rust with minimal setup.

## Features

- **Asynchronous Processing**: Fully asynchronous handling of HTTP requests and responses.
- **Modular Design**: Organized into clear modules: connection, header, request, response, router, and server.
- **Express.js Inspired API**: Familiar and easy-to-use API for those accustomed to Express.js.
- **Custom Routing**: Simple and flexible routing with support for various HTTP methods (GET, POST, PUT, DELETE, etc.).
- **Request and Response Handling**: Comprehensive handling of HTTP requests and responses including headers, body, and status codes.

## Installation

### Using a Local Path

To use `tiny-async-http` from a local path during development:

1. Reference the library in your `Cargo.toml`:

    ```toml
    [dependencies]
    tiny-async-http = { path = "../path/to/tiny-async-http" }
    ```

    Adjust the path to where your library is located on your system.

### Using a Git Repository

To use the library from a git repository:

1. Add the library to your `Cargo.toml` using the git URL:

    ```toml
    [dependencies]
    tiny-async-http = { git = "https://github.com/yourusername/tiny-async-http.git" }
    ```

## Quick Start

Here's a basic example of setting up a server using `tiny-async-http`:

```rust
use tiny_async_http::{Request, Response, StatusCode, ServerBuilder};

#[tokio::main]
async fn main() {
    let server = ServerBuilder::new()
        .get("/", |_req| {
            Response::new()
                .status(StatusCode::OK)
                .set_body("Hello, World!")
        })
        // Additional route handlers...
        .build();

    server.listen(3000, || {
        println!("Server is running on http://localhost:3000");
    }).await;
}
```

## Modules

### connection.rs
Handles incoming TCP connections asynchronously, parsing HTTP requests and routing them to appropriate handlers.

### header.rs
Represents and manipulates HTTP headers.

### request.rs
Defines the `Request` struct, handling HTTP request parsing including methods, URIs, headers, and body.

### response.rs
Defines the `Response` struct, enabling the creation and manipulation of HTTP responses with status codes, headers, and body.

### router.rs
Manages routing, associating HTTP requests with their corresponding handlers based on method and URI.

### server.rs
Facilitates the building and running of the HTTP server, providing methods to define routes and start the server.

## Contributing

Contributions are welcome! If you'd like to contribute to `tiny-async-http`, please submit a pull request.

## License

`tiny-async-http` is open source and is licensed under the MIT License.

---

This README template provides a comprehensive overview of your library, its usage, and its structure. You can modify it to include more specific examples, advanced usage, or any additional information you deem necessary.