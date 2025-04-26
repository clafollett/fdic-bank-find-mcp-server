// Internal imports (std, crate)
use serde::{Deserialize, Serialize, Deserializer, de};

// Public/external imports (alphabetized)
use utoipa::{IntoParams, ToSchema};

/// Shared FDIC BankFind API query parameters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct CommonParameters {
    /// API key used for api.fdic.gov
    #[param(rename = "api_key")]
    pub api_key: Option<String>,

    /// Filter expression (all values must be UPPERCASE)
    #[param(rename = "filters")]
    pub filters: Option<String>,

    /// Comma-delimited list of fields to return (all values must be UPPERCASE)
    #[param(rename = "fields")]
    pub fields: Option<String>,

    /// Field to sort by (default: `NAME`, all values must be UPPERCASE)
    #[param(rename = "sort_by")]
    pub sort_by: Option<String>,

    /// Sort order: `ASC` or `DESC` (default: `ASC`)
    #[param(rename = "sort_order")]
    pub sort_order: Option<String>,

    /// Number of records to return (default: 10, max: 10,000)
    #[param(rename = "limit")]
    #[serde(default, deserialize_with = "de_option_u32")]
    pub limit: Option<u32>,

    /// Pagination offset (default: 0)
    #[param(rename = "offset")]
    #[serde(default, deserialize_with = "de_option_u32")]
    pub offset: Option<u32>,

    /// Response format (json/csv/xml)
    #[param(rename = "fileFormat")]
    pub file_format: Option<String>,

    /// Download flag (if set, triggers file download)
    #[param(rename = "fileDownload")]
    pub file_download: Option<bool>,

    /// Custom filename for download
    #[param(rename = "fileName")]
    pub file_name: Option<String>,
}

// Custom deserializer to accept strings or numbers for Option<u32>
use std::fmt;

fn de_option_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where D: Deserializer<'de>,
{
    struct OptionU32Visitor;

    impl<'de> de::Visitor<'de> for OptionU32Visitor {
        type Value = Option<u32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an integer or string representing an integer")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where E: de::Error,
        {
            Ok(Some(v as u32))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error,
        {
            let s = v.trim();
            if s.is_empty() {
                return Ok(None);
            }
            s.parse::<u32>().map(Some).map_err(|e| de::Error::custom(e.to_string()))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D2>(self, deserializer: D2) -> Result<Self::Value, D2::Error>
        where D2: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }

    deserializer.deserialize_option(OptionU32Visitor)
}

// Generic list handler to reduce boilerplate across endpoints
use crate::config::FDICApiConfig;
use crate::param_utils::validate_common_params;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use reqwest;
use serde_json::json;
use std::collections::HashMap;

/// Standard MCP error response shape, following Anthropic MCP protocol and Rust best practices.
#[derive(Debug, Serialize, ToSchema)]
pub struct MCPError {
    /// Always "error" for MCP error responses.
    #[serde(rename = "type")]
    pub error_type: String,
    /// Error detail object (always present)
    pub error: MCPErrorDetail,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MCPErrorDetail {
    /// Machine-readable error type (e.g., "fdic_proxy_error", "invalid_request_error")
    #[serde(rename = "type")]
    pub kind: String,
    /// Human-readable error message
    pub message: String,
    /// Optional HTTP status code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// Optional additional error details (e.g., FDIC error details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Optional error source (e.g., which parameter was invalid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    /// Optional metadata (e.g., timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
    /// Optionally include the full FDIC error object for advanced debugging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fdic_raw: Option<serde_json::Value>,
}

/// Standard MCP success response shape, following Anthropic MCP protocol and Rust best practices.
#[derive(Debug, Serialize, ToSchema)]
pub struct MCPResponse<T> {
    /// Always "success" for MCP success responses.
    #[serde(rename = "type")]
    pub response_type: String,
    /// Main response data (payload).
    pub data: T,
    /// Optional metadata (e.g., FDIC index info, parameters, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Trait for endpoint-specific parameter injection.
#[allow(dead_code)]
pub trait QueryParameters {
    const VALID_FIELDS: &'static [&'static str];
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>);
    /// Mutable access to shared common params
    fn common_mut(&mut self) -> &mut CommonParameters;
}

/// Proxy common and endpoint-specific params to FDIC BankFind and return parsed response.
#[allow(dead_code)]
pub async fn list_endpoint<Q>(State(config): State<FDICApiConfig>, Query(mut params): Query<Q>, endpoint: &str) -> Response
where
    Q: QueryParameters,
{
    // Validate and collect shared FDIC parameters
    let common = params.common_mut();
    if let Err(e) = validate_common_params(common, Q::VALID_FIELDS) {
        return e.into_response();
    }
    let mut query = HashMap::new();
    if let Some(v) = common.api_key.clone() {
        query.insert("api_key".into(), v);
    }
    if let Some(v) = &common.filters {
        query.insert("filters".into(), v.clone());
    }
    if let Some(v) = &common.fields {
        query.insert("fields".into(), v.clone());
    }
    if let Some(v) = &common.sort_by {
        query.insert("sort_by".into(), v.clone());
    }
    if let Some(v) = &common.sort_order {
        query.insert("sort_order".into(), v.clone());
    }
    if let Some(v) = common.limit.map(|n| n.to_string()) {
        query.insert("limit".into(), v);
    }
    if let Some(v) = common.offset.map(|n| n.to_string()) {
        query.insert("offset".into(), v);
    }
    if let Some(v) = &common.file_format {
        query.insert("fileFormat".into(), v.clone());
    }
    if let Some(v) = common.file_download {
        query.insert("fileDownload".into(), v.to_string());
    }
    if let Some(v) = &common.file_name {
        query.insert("fileName".into(), v.clone());
    }

    // Add endpoint-specific params via trait
    params.insert_endpoint_specific(&mut query);

    let client = reqwest::Client::new();
    let url = format!("{}/{}", config.base_url.trim_end_matches('/'), endpoint);
    let res = match client.get(&url).query(&query).send().await {
        Ok(r) => r,
        Err(e) => {
            let err = MCPError {
                error_type: "error".to_string(),
                error: MCPErrorDetail {
                    kind: "fdic_proxy_error".to_string(),
                    message: format!("FDIC API unreachable: {}", e),
                    status: Some(StatusCode::BAD_GATEWAY.as_u16()),
                    detail: None,
                    source: None,
                    meta: None,
                    fdic_raw: None,
                },
            };
            return (StatusCode::BAD_GATEWAY, Json(err)).into_response();
        }
    };

    let status = StatusCode::from_u16(res.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let bytes = match res.bytes().await {
        Ok(b) => b,
        Err(e) => {
            let err = MCPError {
                error_type: "error".to_string(),
                error: MCPErrorDetail {
                    kind: "fdic_proxy_error".to_string(),
                    message: format!("FDIC API response error: {}", e),
                    status: Some(StatusCode::BAD_GATEWAY.as_u16()),
                    detail: None,
                    source: None,
                    meta: None,
                    fdic_raw: None,
                },
            };
            return (StatusCode::BAD_GATEWAY, Json(err)).into_response();
        }
    };

    // Try to parse FDIC JSON error or data
    if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        let meta = val.get("meta");
        let data = val.get("data");
        if meta.is_some() && data.is_some() {
            let resp = MCPResponse {
                response_type: "success".to_string(),
                data: data.unwrap().clone(),
                meta: meta.cloned(),
            };
            return (status, Json(resp)).into_response();
        }
        // FDIC error shape: { "errors": [ ... ] }
        if let Some(errors) = val.get("errors") {
            // Use the first error if present
            let fdic_error = errors.get(0).cloned().unwrap_or_else(|| json!({"message": "Unknown FDIC error"}));
            let message = fdic_error.get("title").and_then(|v| v.as_str()).unwrap_or("FDIC API error").to_string();
            let detail = fdic_error.get("detail").and_then(|v| v.as_str()).map(|s| s.to_string());
            let status_code = fdic_error.get("status").and_then(|v| v.as_u64()).map(|n| n as u16);
            let source = fdic_error.get("source").cloned();
            let meta_val = fdic_error.get("meta").cloned();
            let err = MCPError {
                error_type: "error".to_string(),
                error: MCPErrorDetail {
                    kind: "fdic_proxy_error".to_string(),
                    message,
                    status: status_code,
                    detail,
                    source,
                    meta: meta_val,
                    fdic_raw: Some(fdic_error.clone()),
                },
            };
            return (status, Json(err)).into_response();
        }
        // If not a recognized FDIC shape, return as data
        let resp = MCPResponse {
            response_type: "success".to_string(),
            data: val,
            meta: None,
        };
        return (status, Json(resp)).into_response();
    }

    // Fallback for non-JSON
    let fallback = match String::from_utf8(bytes.to_vec()) {
        Ok(text) => text,
        Err(_) => STANDARD.encode(&bytes),
    };
    let err = MCPError {
        error_type: "error".to_string(),
        error: MCPErrorDetail {
            kind: "fdic_proxy_error".to_string(),
            message: "FDIC API returned non-JSON response".to_string(),
            status: Some(StatusCode::BAD_GATEWAY.as_u16()),
            detail: None,
            source: None,
            meta: Some(json!({"raw_response": fallback})),
            fdic_raw: None,
        },
    };
    (StatusCode::BAD_GATEWAY, Json(err)).into_response()
}
