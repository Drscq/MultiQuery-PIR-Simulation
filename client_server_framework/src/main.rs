use client_server_framework::{Server, Client};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Create a server
    let server = Server::bind("127.0.0.1:8080").await.unwrap();

    // Create a client and connect to the server
    let mut client = Client::connect("127.0.0.1:8080").await.unwrap();

    // Send a message
    client.send(b"Hello, world!").await.unwrap();

    // Receive a response
    let response = client.receive().await.unwrap();

    // Print the server's response
    println!("{:?}", response);
}
