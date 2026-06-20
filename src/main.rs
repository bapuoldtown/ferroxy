use tokio::net::{TcpListener, TcpStream};
use tokio::io;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on port 8080");

    loop {
        let (mut client, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", addr);

        // Spawn a task per connection so the listener can continue accepting.
        tokio::spawn(async move {
            match TcpStream::connect("127.0.0.1:7777").await {
                Ok(mut backend) => {
                    println!("  -> connected to backend 127.0.0.1:8000");
                    match io::copy_bidirectional(&mut client, &mut backend).await {
                        Ok((from_client, from_backend)) => {
                            println!("  -> transferred {} bytes from client to backend", from_client);
                            println!("  -> transferred {} bytes from backend to client", from_backend);
                        }
                        Err(e) => println!("  -> error during bidirectional transfer: {}", e),
                    }
                }
                Err(e) => println!("  -> could not reach backend: {}", e),
            }
        });
    }
}