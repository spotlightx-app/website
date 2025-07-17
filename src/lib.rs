use tokio::net::TcpListener;

use axum::{Router, routing::get};

fn build_router() -> Router {
    Router::new().route("/health_check", get(|| async {}))
}

pub async fn start_app(listener: TcpListener) {
    let router = build_router();
    axum::serve(listener, router).await.unwrap()
}
