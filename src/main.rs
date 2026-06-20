use tokio::net::TcpListener;
#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on port 8080");
    loop {
        let (_client, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", addr);
    }
}
