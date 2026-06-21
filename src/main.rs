use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::main]
async fn main() {
    // Piece 1: a LIST of backends (Vec), wrapped in Arc so couriers can share it.
    let backends = Arc::new(vec![
        "127.0.0.1:7777".to_string(),
        "127.0.0.1:7778".to_string(),
        "127.0.0.1:7779".to_string(),
    ]);
    //We will spin up 3 server process in 3 terminals on port 7777,7778 and 7779
    // Piece 2: the shared, race-proof ticket machine.
    let counter = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on port 8080");

    loop {
        let (mut client, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", addr);
        //get reference of backends and 
        let counter=Arc::clone(&counter);
        let backends = Arc::clone(&backends);



        // Spawn a task per connection so the listener can continue accepting.
        tokio::spawn(async move {
            //Get a ticket from counter and in arace safe manner
            let ticket = counter.fetch_add(1, Ordering::Relaxed);
            let backend_server = backends[ticket % backends.len()].clone();
            println!("  -> routing to backend: {}", backend_server);
            match TcpStream::connect(backend_server.as_str()).await {
                Ok(mut backend) => {
                    println!("  -> connected to backend: {}", backend_server);
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