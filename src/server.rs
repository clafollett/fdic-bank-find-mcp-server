//! Async server and signal runner for fdic-bank-find-mcp-server
//
// This module provides clean, idiomatic orchestration for running the MCP server and
// signal handling concurrently, using tokio::select! to enable hot reloads and graceful shutdown.

//! Unified server orchestration for fdic-bank-find-mcp-server
//!
//! Handles transport selection (stdio, SSE/Axum), async signal handling (hot reload/shutdown),
//! and launches the correct server loop. All logic is modular, idiomatic, and testable.

// === Imports ===
// Internal imports (std, crate)
use crate::config::Config;
use crate::handlers::FdicBankFindMcpServer;
use crate::signal::{SignalEvent, spawn_signal_listener};

// External imports (alphabetized)
use dotenvy::dotenv;
use log::debug;
use rmcp::{
    ServiceExt,
    transport::{
        sse_server::{SseServer, SseServerConfig},
        stdio,
    },
};
use std::{process, sync::Arc, time::Duration};

use tokio::sync::{Mutex, Notify};
use tokio_util::sync::CancellationToken;
use tracing::info;

// === Type Definitions ===

/// ServerMode defines which server to run: stdio (CLI) or SSE/Axum (web).
#[derive(Debug, Clone)]
pub enum ServerMode {
    Stdio,
    Sse(SseConfig),
}

/// Configuration for SSE/Axum server mode.
#[derive(Debug, Clone)]
pub struct SseConfig {
    pub addr: std::net::SocketAddr,
    pub sse_path: String,
    pub post_path: String,
    pub keep_alive: Option<Duration>,
}

/// Runs the unified server orchestrator.
///
/// - Selects transport (stdio or SSE) and builds config
/// - Spawns the server and async signal handler
/// - Uses tokio::select! to manage graceful shutdown and hot reload
/// - Keeps logging guards alive for the duration
pub async fn start(
    cfg: Arc<Mutex<Config>>, file_guard: impl Send + Sync + 'static, stderr_guard: impl Send + Sync + 'static,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mode, _sse_mode) = {
        let cfg_guard = cfg.lock().await;
        select_server_mode(&*cfg_guard)
    };
    let notify = Arc::new(Notify::new());
    let event = Arc::new(Mutex::new(None));

    spawn_signal_listener(notify.clone(), event.clone()).await;

    // Launch the appropriate server as a task
    let server_task = tokio::spawn(async move {
        let res = match mode {
            ServerMode::Stdio => run_stdio_server().await,
            ServerMode::Sse(cfg) => run_sse_server(cfg).await,
        };
        if let Err(e) = res {
            info!(target = "server", "Server exited with error: {:?}", e);
        }
    });
    let signal_task = tokio::spawn(signal_loop(notify.clone(), event.clone(), cfg.clone()));

    // Wait for either the server or a signal event (shutdown/reload)
    tokio::select! {
        res = server_task => {
            info!(target = "server", "Server task ended: {:?}", res);
        }
        res = signal_task => {
            info!(target = "server", "Signal handler task ended: {:?}", res);
        }
    }

    // Guards must remain alive for the duration of main
    let _ = (file_guard, stderr_guard);
    Ok(())
}

// === Private Helpers ===

/// Runs the stdio (CLI/Inspector) server loop.
async fn run_stdio_server() -> Result<(), Box<dyn std::error::Error>> {
    debug!("[FDIC MCP] run_stdio_server start");

    // Use an explicitly non-buffered stdio transport
    let service = FdicBankFindMcpServer::new().serve(stdio()).await?;

    debug!("[FDIC MCP] run_stdio_server acquired service, about to wait");

    let waiting_res = service.waiting().await;
    debug!("[FDIC MCP] run_stdio_server waiting completed: {:?}", waiting_res);

    waiting_res?;
    Ok(())
}

/// Runs the SSE/Axum (web) server loop.
async fn run_sse_server(cfg: SseConfig) -> Result<(), Box<dyn std::error::Error>> {
    let sse_config = SseServerConfig {
        bind: cfg.addr,
        sse_path: cfg.sse_path,
        post_path: cfg.post_path,
        ct: CancellationToken::new(),
        sse_keep_alive: cfg.keep_alive,
    };
    let (sse_server, router) = SseServer::new(sse_config);
    let _ct = sse_server.with_service(move || FdicBankFindMcpServer::new());
    debug!("[FDIC MCP] Starting SSE/Axum server on {}...", cfg.addr);
    let listener = tokio::net::TcpListener::bind(cfg.addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

/// Reads config and selects the server mode (stdio or SSE/Axum).
/// Returns the mode and a bool for SSE mode.
fn select_server_mode(cfg: &Config) -> (ServerMode, bool) {
    if cfg.transport.eq_ignore_ascii_case("sse") {
        debug!("[FDIC MCP] SSE mode selected");
        (
            ServerMode::Sse(SseConfig {
                addr: cfg.sse_addr,
                sse_path: "/sse".to_string(),
                post_path: "/message".to_string(),
                keep_alive: Some(cfg.sse_keep_alive),
            }),
            true,
        )
    } else {
        debug!("[FDIC MCP] Stdio mode selected");
        (ServerMode::Stdio, false)
    }
}

/// Async signal event loop for hot reload and graceful shutdown.
async fn signal_loop(notify: Arc<Notify>, event: Arc<Mutex<Option<SignalEvent>>>, cfg: Arc<Mutex<Config>>) {
    loop {
        notify.notified().await;
        let mut ev = event.lock().await;
        match *ev {
            Some(SignalEvent::Reload) => {
                info!(target: "signal", "Hot reload triggered – reloading config");
                dotenv().ok();
                let new_cfg = Config::load();
                {
                    let mut cfg_guard = cfg.lock().await;
                    *cfg_guard = new_cfg.clone();
                    info!(target: "signal", "Config reloaded: {:?}", *cfg_guard);
                }
            }
            Some(SignalEvent::Shutdown) => {
                info!(target: "signal", "Shutdown signal received – shutting down gracefully");
                process::exit(0);
            }
            None => {}
        }
        *ev = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::net::SocketAddr;
    use std::time::Duration;

    #[test]
    fn test_select_server_mode_stdio() {
        let cfg = Config {
            log_dir: "logs".to_string(),
            transport: "stdio".to_string(),
            sse_addr: "1.2.3.4:8000".parse::<SocketAddr>().unwrap(),
            sse_keep_alive: Duration::from_secs(5),
        };
        let (mode, sse) = select_server_mode(&cfg);
        assert!(matches!(mode, ServerMode::Stdio));
        assert!(!sse);
    }

    #[test]
    fn test_select_server_mode_sse() {
        let mut cfg = Config {
            log_dir: "logs".to_string(),
            transport: "stdio".to_string(),
            sse_addr: "1.2.3.4:9000".parse::<SocketAddr>().unwrap(),
            sse_keep_alive: Duration::from_secs(10),
        };
        cfg.transport = "sSe".to_string();
        let (mode, sse_b) = select_server_mode(&cfg);
        match mode {
            ServerMode::Sse(sse_cfg) => {
                assert_eq!(sse_cfg.addr, cfg.sse_addr);
                assert_eq!(sse_cfg.keep_alive.unwrap(), cfg.sse_keep_alive);
            }
            _ => panic!("Expected Sse mode"),
        }
        assert!(sse_b);
    }
}
