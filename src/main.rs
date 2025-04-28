// Internal modules
mod common;
mod config;
mod handlers;
mod param_utils;

// Internal imports (std, crate)
use std::env;
use std::fs;
use std::net::SocketAddr;

// External imports (alphabetized)
use axum::Router;
use axum::response::IntoResponse;
use axum::routing::get;
use dotenvy::dotenv;
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use crate::config::FDICApiConfig;
use crate::handlers::register_fdic_handlers;

/// MCP-compliant root endpoint for health and service metadata.
async fn root_handler() -> impl axum::response::IntoResponse {
    use axum::Json;
    use serde_json::json;

    let response = json!({
        "meta": {
            "service": "fdic-bank-find-mcp-server",
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION")
        },
        "data": {
            "message": "MCP server is running"
        }
    });
    Json(response)
}

async fn openapi_json() -> impl IntoResponse {
    let data = fs::read_to_string("public/openapi.json").expect("openapi.json not found");
    axum::response::Json(serde_json::from_str::<serde_json::Value>(&data).expect("openapi.json is not valid JSON"))
}

async fn not_found_handler() -> impl IntoResponse {
    use axum::http::StatusCode;
    let err = crate::common::MCPError {
        error_type: "error".to_string(),
        error: crate::common::MCPErrorDetail {
            kind: "not_found".to_string(),
            message: "The requested resource was not found.".to_string(),
            status: Some(404),
            detail: None,
            source: None,
            meta: None,
            fdic_raw: None,
        },
    };
    (StatusCode::NOT_FOUND, axum::Json(err))
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present
    dotenv().ok();
    // Initialize tracing subscriber for JSON logging (CloudWatch-friendly)
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let fdic_config = FDICApiConfig { base_url: "https://banks.data.fdic.gov/api".to_string() };
    let router = Router::new()
        .layer(TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version()
            )
        }))
        .route("/", get(root_handler))
        .route("/openapi.json", get(openapi_json))
        .nest_service("/docs", ServeDir::new("public/swagger-ui"))
        .fallback(not_found_handler);

    // MCP auto-generated: Register all FDIC handlers
    let router = register_fdic_handlers(router).with_state(fdic_config);

    // Read bind address and preferred ports from env
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());

    let addr = format!("{}:{}", bind_addr, port)
        .parse::<SocketAddr>()
        .unwrap();

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::warn!("Port {} unavailable on {}: {}", port, bind_addr, e);
            std::process::exit(1);
        }
    };

    tracing::info!("listening on http://{}:{}", bind_addr, port);

    // MCP-compliant advertisement line
    println!("MCP server listening on http://{}:{}", bind_addr, port);

    axum::serve(listener, router).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use serde_json::Value;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_openapi_json_endpoint() {
        let app = Router::new().route("/openapi.json", get(super::openapi_json));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).expect("response is valid JSON");
        assert!(json.get("openapi").is_some(), "openapi field present");
        assert!(json.get("paths").is_some(), "paths field present");
    }

    #[tokio::test]
    async fn test_swagger_ui_docs_served() {
        let app = Router::new().nest_service("/docs", ServeDir::new("public/swagger-ui"));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/docs/index.html")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.starts_with("text/html"), "Content-Type is HTML");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8_lossy(&body).to_lowercase();
        assert!(html.contains("<!doctype html>"), "Contains <!DOCTYPE html>");
        assert!(html.contains("<html"), "Contains <html> tag");
    }

    #[tokio::test]
    async fn test_root_handler() {
        let app = Router::new().route("/", get(root_handler));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).expect("response is valid JSON");
        assert_eq!(json["meta"]["service"], "fdic-bank-find-mcp-server");
        assert_eq!(json["meta"]["status"], "ok");
        assert!(json["meta"]["version"].is_string());
        assert_eq!(json["data"]["message"], "MCP server is running");
    }
}
