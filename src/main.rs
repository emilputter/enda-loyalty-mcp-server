#[tokio::main]

 async fn main() {

    println!("Starting ENDA MCP server");

    let response = reqwest:: get(
       "https://api.hederacourt.site/api/v1/client-classes"
    )
    .await
    .expect("Failed to send request");

    println!("Status: {}", response.status());
    let body = response
    .json()
    .await
    .expect("Failed to read response body");

    println!("Body: {}", body);
}

