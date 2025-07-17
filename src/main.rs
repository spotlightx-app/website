use spotlightx::start_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind to address.");
    start_app(listener).await
}
