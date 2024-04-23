use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use std::time::SystemTime;

pub async fn run() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Server running on localhost:7878");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        tokio::spawn(async move {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello world!";
            let now = SystemTime::now();
            println!("Connection from: {} at {:?}", addr, now);
            socket.write_all(response.as_bytes()).await.expect("Failed to write response");
        });
    }
}
