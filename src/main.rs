use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on port 8080");

    loop {
        let (_client, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {addr}");

        // INSIDE the loop, AFTER accept: dial the backend for THIS client.
        match TcpStream::connect("127.0.0.1:9001").await {
            Ok(_backend) => println!("  -> connected to backend 127.0.0.1:9001"),
            Err(e)       => println!("  -> could not reach backend: {e}"),
        }
    }
}