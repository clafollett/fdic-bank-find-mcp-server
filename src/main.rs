// Internal modules
mod common;
mod config;
mod handlers;
mod param_utils;

// Internal imports (std, crate)
use crate::handlers::FdicBankFindMcpServer;

// External imports (alphabetized)
use dotenvy::dotenv;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use rmcp::{
    ServiceExt,
    transport::{
        sse_server::{SseServer, SseServerConfig},
        stdio,
    },
};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("=== FDIC MCP main() reached ===");
    dotenv().ok();

    // === Dual Logging Setup (configurable) ===
    // Get log directory from env (default: "logs")
    let log_dir = std::env::var("FDIC_MCP_LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    std::fs::create_dir_all(&log_dir)?;

    // 1. File logger (daily rotation)
    let file_appender = RollingFileAppender::new(Rotation::DAILY, &log_dir, "fdic-mcp.log");
    let (file_writer, _file_guard): (_, WorkerGuard) = tracing_appender::non_blocking(file_appender);

    // 2. Stderr logger (non-blocking)
    let (stderr_writer, _stderr_guard): (_, WorkerGuard) = tracing_appender::non_blocking(std::io::stderr());

    // 3. Combine writers using .and()
    let multi_writer = file_writer.and(stderr_writer);

    tracing_subscriber::fmt()
        .json()
        .with_writer(multi_writer)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    eprintln!("[FDIC MCP] After tracing_subscriber setup");

    // Select transport at runtime (env var MCP_TRANSPORT)
    let transport = match std::env::var("MCP_TRANSPORT").as_deref() {
        Ok("sse") => {
            // SSE/Axum mode
            eprintln!("[FDIC MCP] SSE mode selected");
            let addr: std::net::SocketAddr = std::env::var("MCP_SSE_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:7878".to_string())
                .parse()?;
            let sse_config = SseServerConfig {
                bind: addr,
                sse_path: "/sse".to_string(),
                post_path: "/message".to_string(),
                ct: CancellationToken::new(),
                sse_keep_alive: Some(Duration::from_secs(15)),
            };
            let (sse_server, router) = SseServer::new(sse_config);
            let _ct = sse_server.with_service(move || FdicBankFindMcpServer::new());
            eprintln!("[FDIC MCP] Starting SSE/Axum server on {}...", addr);
            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router).await?;
            return Ok(());
        }
        _ => {
            eprintln!("[FDIC MCP] Stdio mode selected");
            stdio()
        }
    };

    // Stdio mode: Inspector/CLI
    eprintln!("[FDIC MCP] Before server()");
    let service = FdicBankFindMcpServer::new().serve(transport).await?;
    eprintln!("[FDIC MCP] After serve()");
    service.waiting().await?;
    eprintln!("[FDIC MCP] After waiting()");
    Ok(())
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
