//! Common configuration for FDIC BankFind MCP Server
//!
//! Contains configuration structs shared across handlers.

/// Config for FDIC BankFind API base URL.
#[derive(Clone, Debug)]
pub struct FdicApiConfig {
    pub base_url: String,
}
