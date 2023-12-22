mod request;
mod response;
mod router;
mod connection;
mod server;

use server::HttpServer;
use request::Request;
use response::Response;
use response::StatusCode;
use server::ServerBuilder;

use std::sync::Arc;

fn main() {
     let server = ServerBuilder::new()
        .get("/", |req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("Hello, World!");
            response
        })
        .get("/about", |req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("About Us");
            response
        })
        .post("/data", |req| {
            let mut response = Response::new();
            let data = req.body.clone().unwrap_or_else(|| "No data provided".to_string());
            response.status(StatusCode::OK)
                    .set_body(&format!("Received data: {}", data));
            response
        })
        .delete("/reset", |req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("Resetting server");
            response
        })
        .build();

    // Start the server on port 3000
    server.listen(3000, || {
        println!("Server is running on http://localhost:3000");
    });
}