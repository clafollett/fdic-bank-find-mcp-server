// Internal modules
mod common;
mod config;
mod handlers;
mod param_utils;
mod server;
mod signal;

// Internal imports (std, crate)
use crate::config::Config;
use std::sync::Arc;
use tokio::sync::Mutex;

// External imports (alphabetized)
use dotenvy::dotenv;
use log::debug;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    debug!("=== FDIC MCP main() reached ===");
    dotenv().ok();

    // Load application config
    let cfg = Arc::new(Mutex::new(Config::load()));
    // Create log directory from config
    let log_dir = {
        let cfg_guard = cfg.lock().await;
        cfg_guard.log_dir.clone()
    };
    std::fs::create_dir_all(&log_dir)?;

    // === Dual Logging Setup (configurable) ===
    // 1. File logger (daily rotation, async non-blocking)
    let file_appender = RollingFileAppender::new(Rotation::DAILY, &log_dir, "fdic-mcp.log");
    let (file_writer, file_guard): (NonBlocking, WorkerGuard) = tracing_appender::non_blocking(file_appender);

    // 2. Stderr logger (async non-blocking)
    let (stderr_writer, stderr_guard): (NonBlocking, WorkerGuard) = tracing_appender::non_blocking(std::io::stderr());
    // IMPORTANT: Keep file_guard and stderr_guard alive for the duration of main() to prevent premature shutdown of logging and stdio, especially in Docker or MCP stdio mode.

    // 3. Combine writers using .and()
    let multi_writer = file_writer.and(stderr_writer);

    tracing_subscriber::fmt()
        .json()
        .with_writer(multi_writer)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    debug!("[FDIC MCP] After tracing_subscriber setup");

    // Run unified server orchestrator (handles transport, hot reload, shutdown)
    server::start(cfg.clone(), file_guard, stderr_guard).await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::Router;
//     use axum::body::Body;
//     use axum::http::{Request, StatusCode};
//     use http_body_util::BodyExt;
//     use serde_json::Value;
//     use tower::ServiceExt; // for `oneshot`

//     #[tokio::test]
//     async fn test_openapi_json_endpoint() {
//         let app = Router::new().route("/openapi.json", get(super::openapi_json));
//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/openapi.json")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::OK);
//         let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
//         let json: Value = serde_json::from_slice(&body_bytes).expect("response is valid JSON");
//         assert!(json.get("openapi").is_some(), "openapi field present");
//         assert!(json.get("paths").is_some(), "paths field present");
//     }

//     #[tokio::test]
//     async fn test_swagger_ui_docs_served() {
//         let app = Router::new().nest_service("/docs", ServeDir::new("public/swagger-ui"));
//         let response = app
//             .oneshot(
//                 Request::builder()
//                     .uri("/docs/index.html")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//         assert_eq!(response.status(), StatusCode::OK);
//         let content_type = response
//             .headers()
//             .get("content-type")
//             .unwrap()
//             .to_str()
//             .unwrap();
//         assert!(content_type.starts_with("text/html"), "Content-Type is HTML");
//         let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
//         let html = String::from_utf8_lossy(&body_bytes).to_lowercase();
//         assert!(html.contains("<!doctype html>"), "Contains <!DOCTYPE html>");
//         assert!(html.contains("<html"), "Contains <html> tag");
//     }

//     #[tokio::test]
//     async fn test_root_handler() {
//         let app = Router::new().route("/", get(root_handler));
//         let response = app
//             .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
//             .await
//             .unwrap();

//         assert_eq!(response.status(), StatusCode::OK);
//         let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
//         let json: Value = serde_json::from_slice(&body_bytes).expect("response is valid JSON");
//         assert_eq!(json["meta"]["service"], "fdic-bank-find-mcp-server");
//         assert_eq!(json["meta"]["status"], "ok");
//         assert!(json["meta"]["version"].is_string());
//         assert_eq!(json["data"]["message"], "MCP server is running");
//     }
// }
