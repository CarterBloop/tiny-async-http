mod connection;
mod request;
mod response;
mod router;
mod server;

use request::Request;
use response::Response;
use response::StatusCode;
use server::ServerBuilder;

fn main() {
    let server = ServerBuilder::new()
        .get("/", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK).set_body("Hello, World!");
            response
        })
        .get("/about", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK).set_body("About Us");
            response
        })
        .post("/data", |req| {
            let mut response = Response::new();
            let data = req
                .body
                .clone()
                .unwrap_or_else(|| "No data provided".to_string());
            response
                .status(StatusCode::OK)
                .set_body(&format!("Received data: {}", data));
            response
        })
        .put("/data", |req| {
            let mut response = Response::new();
            let data = req
                .body
                .clone()
                .unwrap_or_else(|| "No data provided".to_string());
            response
                .status(StatusCode::OK)
                .set_body(&format!("Updating data: {}", data));
            response
        })
        .delete("/reset", |req| {
            let mut response = Response::new();
            let data = req
                .body
                .clone()
                .unwrap_or_else(|| "No data provided".to_string());
            response
                .status(StatusCode::OK)
                .set_body(&format!("Deleting data: {}", data));
            response
        })
        .build();

    // Start the server on port 3000
    server.listen(3000, || {
        println!("Server is running on http://localhost:3000");
    });
}
