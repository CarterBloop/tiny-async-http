use tiny_async_http::ServerBuilder;
use tiny_async_http::response::{Response, StatusCode};
use tiny_async_http::request::Request;
use tokio::sync::oneshot;
use tokio::task;

#[tokio::test]
async fn test_server_responses() {
    let (tx, rx) = oneshot::channel();
    let port = 3001; // Test on a different port to avoid conflicts

    let server = ServerBuilder::new()
        .get("/", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK).set_header("Content-Type", "text/html")
                    .set_body("<html><body><h1>Home Page</h1></body></html>");
            response
        })
        .post("/data", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("POST request received");
            response
        })
        .put("/data", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("PUT request received");
            response
        })
        .delete("/data", |_req| {
            let mut response = Response::new();
            response.status(StatusCode::OK)
                    .set_body("DELETE request received");
            response
        })
        .build();

    let server_task = task::spawn(async move {
        server.listen(port, || tx.send(()).unwrap()).await;
    });

    rx.await.expect("Server did not start");

    let client = reqwest::Client::new();

    // Test GET request
    let res = client.get(format!("http://localhost:{}/", port))
        .send()
        .await
        .expect("Failed to send GET request");
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "<html><body><h1>Home Page</h1></body></html>");

    // Test POST request
    let res = client.post(format!("http://localhost:{}/data", port))
        .send()
        .await
        .expect("Failed to send POST request");
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "POST request received");

    // Test PUT request
    let res = client.put(format!("http://localhost:{}/data", port))
        .send()
        .await
        .expect("Failed to send PUT request");
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "PUT request received");

    // Test DELETE request
    let res = client.delete(format!("http://localhost:{}/data", port))
        .send()
        .await
        .expect("Failed to send DELETE request");
    assert_eq!(res.status(), reqwest::StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "DELETE request received");

    server_task.abort(); // Shut down the server
}