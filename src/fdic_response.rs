use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// MCP-standard response for FDIC BankFind API responses
/// Contains metadata about the query and an array of FDIC Response records.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FDICResponse<T> {
    /// Response Metadata
    /// Metadata about the query (e.g., filters, pagination, total count).
    pub meta: serde_json::Value,
    /// Response Data
    /// Array of FDIC Response records matching the query.
    pub data: Vec<T>,
}
