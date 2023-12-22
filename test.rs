use reqwest::blocking::{Client, Response};
use std::error::Error;

fn test_get(client: &Client, url: &str) -> Result<Response, Box<dyn Error>> {
    let response = client.get(url).send()?;
    Ok(response)
}

fn test_post(client: &Client, url: &str, data: &str) -> Result<Response, Box<dyn Error>> {
    let response = client.post(url).body(data.to_string()).send()?;
    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let base_url = "http://localhost:3000";

    // Test GET endpoints
    let get_responses = [
        test_get(&client, &format!("{}/", base_url)),
        test_get(&client, &format!("{}/about", base_url)),
    ];

    // Test POST endpoint
    let post_response = test_post(&client, &format!("{}/data", base_url), "Sample data");

    // Check and print the responses
    for response in get_responses.iter() {
        match response {
            Ok(resp) => println!("GET Status: {}, Body: {:?}", resp.status(), resp.text()?),
            Err(e) => println!("Error: {}", e),
        }
    }

    match post_response {
        Ok(resp) => println!("POST Status: {}, Body: {:?}", resp.status(), resp.text()?),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}