use axum::{Router, http::StatusCode, response::IntoResponse, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

async fn locations_handler() -> impl IntoResponse {
    // Placeholder: Will call FDIC API in future steps
    (StatusCode::OK, "Locations endpoint: Not yet implemented")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/locations", get(locations_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
