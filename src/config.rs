//! Common configuration for FDIC BankFind MCP Server
//!
//! Contains configuration structs shared across handlers and all application env/config.

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

/// Returns the default log directory, relative to the running binary.
pub fn default_log_dir() -> String {
    // Compute binary path, then its parent 'logs' directory
    let log_path = env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.join("logs")))
        .unwrap_or_else(|| PathBuf::from("logs"));
    // Use display().to_string() for safe conversion
    log_path.display().to_string()
}

/// Config for FDIC BankFind API base URL.
#[derive(Clone, Debug)]
pub struct FdicApiConfig {
    pub base_url: String,
}

/// Centralized application configuration, loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Directory for logs (default: "logs")
    pub log_dir: String,
    /// Transport mode: "stdio" or "sse" (default: "stdio")
    pub transport: String,
    /// Address for SSE server (default: "127.0.0.1:7878")
    pub sse_addr: SocketAddr,
    /// Keep-alive interval for SSE (default: 15s)
    pub sse_keep_alive: Duration,
    // Add more fields as needed!
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_dir: default_log_dir(),
            transport: "stdio".to_string(),
            sse_addr: "127.0.0.1:7878".parse().unwrap(),
            sse_keep_alive: Duration::from_secs(15),
        }
    }
}

impl Config {
    /// Loads config from environment variables, falling back to defaults.
    pub fn load() -> Self {
        Self {
            log_dir: env::var("FDIC_MCP_LOG_DIR").unwrap_or_else(|_| default_log_dir()),
            transport: env::var("MCP_TRANSPORT").unwrap_or_else(|_| Self::default().transport),
            sse_addr: env::var("MCP_SSE_ADDR")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(|| Self::default().sse_addr),
            sse_keep_alive: env::var("MCP_SSE_KEEP_ALIVE_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .map(Duration::from_secs)
                .unwrap_or_else(|| Self::default().sse_keep_alive),
        }
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use std::env;
    use std::net::SocketAddr;
    use std::time::Duration;

    #[test]
    fn test_default_log_dir_ends_with_logs() {
        let dir = default_log_dir();
        assert!(dir.ends_with("logs"), "default_log_dir should end with 'logs'");
    }

    #[test]
    fn test_config_default_values() {
        // NOTE: set_var and remove_var are unsafe only in a multi-threaded, non Windows environment, but we're testing them here
        unsafe {
            env::remove_var("FDIC_MCP_LOG_DIR");
            env::remove_var("MCP_TRANSPORT");
            env::remove_var("MCP_SSE_ADDR");
            env::remove_var("MCP_SSE_KEEP_ALIVE_SECS");
        }
        let cfg = Config::load();
        assert_eq!(cfg.transport, "stdio");
        assert_eq!(cfg.sse_addr, "127.0.0.1:7878".parse::<SocketAddr>().unwrap());
        assert_eq!(cfg.sse_keep_alive, Duration::from_secs(15));
    }

    #[test]
    fn test_config_env_overrides() {
        // NOTE: set_var and remove_var are unsafe only in a multi-threaded, non Windows environment, but we're testing them here
        unsafe {
            env::set_var("FDIC_MCP_LOG_DIR", "/tmp/logs");
            env::set_var("MCP_TRANSPORT", "sse");
            env::set_var("MCP_SSE_ADDR", "0.0.0.0:9000");
            env::set_var("MCP_SSE_KEEP_ALIVE_SECS", "30");
        }
        let cfg = Config::load();
        assert_eq!(cfg.log_dir, "/tmp/logs");
        assert_eq!(cfg.transport, "sse");
        assert_eq!(cfg.sse_addr, "0.0.0.0:9000".parse::<SocketAddr>().unwrap());
        assert_eq!(cfg.sse_keep_alive, Duration::from_secs(30));
        // NOTE: set_var and remove_var are unsafe only in a multi-threaded, non Windows environment, but we're testing them here
        unsafe {
            env::remove_var("FDIC_MCP_LOG_DIR");
            env::remove_var("MCP_TRANSPORT");
            env::remove_var("MCP_SSE_ADDR");
            env::remove_var("MCP_SSE_KEEP_ALIVE_SECS");
        }
    }
}
