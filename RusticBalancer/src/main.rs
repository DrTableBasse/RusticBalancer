mod server;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    server::run().await
}
