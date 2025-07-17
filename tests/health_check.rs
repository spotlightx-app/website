use spotlightx::start_app;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind to address.");
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(start_app(listener));

    format!("http://127.0.0.1:{port}")
}
