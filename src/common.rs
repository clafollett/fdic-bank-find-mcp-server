// Internal imports (std, crate)
use crate::config::FdicApiConfig;
use std::collections::HashMap;

// Public/external imports (alphabetized)
use log;
use reqwest;
use rmcp::model::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::json;
use utoipa::ToSchema;

/// Maximum allowed FDIC API response Content-Length in bytes.
/// Configurable via the FDIC_MAX_RESPONSE_CONTENT_LENGTH env variable (default: 5MB).
pub const FDIC_MAX_RESPONSE_CONTENT_LENGTH: usize = 5 * 1024 * 1024; // 5MB

/// Shared FDIC BankFind API query parameters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct CommonParameters {
    #[schemars(description = r#"API key used for api.fdic.gov"#)]
    pub api_key: Option<String>,

    #[schemars(description = r#"Filter expression for advanced querying using FDIC BankFind syntax. 
    
    **Format:** `FIELD:VALUE`, `FIELD:>=VALUE`, `FIELD:<=VALUE`, combined with `AND`/`OR`.
    
    **Examples:**
      - Single value: `CERT:10002`
      - Date range: `FAILDATE:>=20200101 AND FAILDATE:<=20201231`
      - Multiple filters: `STATE:CA AND ACTIVE:1`
    
    **Note:** Incorrect syntax will result in a 400 error from the FDIC API.
    
    For full details, see: https://banks.data.fdic.gov/docs
    
    All Fields must be UPPERCASE.
    "#)]
    pub filters: Option<String>,

    #[schemars(description = r#"Comma-delimited list of fields to return (all fields must be UPPERCASE)"#)]
    pub fields: Option<String>,

    #[schemars(description = r#"Field to sort by (default: `NAME`, all fields must be UPPERCASE)"#)]
    pub sort_by: Option<String>,

    #[schemars(description = r#"Sort order: `ASC` or `DESC` (default: `ASC`)"#)]
    pub sort_order: Option<String>,

    #[schemars(description = r#"Number of records to return (default: 10, max: 10,000)"#)]
    pub limit: Option<u32>,

    #[schemars(description = r#"Pagination offset (default: 0)"#)]
    pub offset: Option<u32>,

    #[schemars(description = r#"Response format (json/csv/xml)"#)]
    pub file_format: Option<String>,

    #[schemars(description = r#"Download flag (if set, triggers file download)"#)]
    pub file_download: Option<bool>,

    #[schemars(description = r#"Custom filename for download"#)]
    pub file_name: Option<String>,
}

/// Trait for endpoint-specific parameter injection.
pub trait QueryParameters {
    const VALID_FIELDS: &'static [&'static str];

    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>);

    /// Mutable access to shared common params
    fn common_mut(&mut self) -> &mut CommonParameters;
}

/// Trait to associate a parameter type with its FDIC endpoint name.
pub trait FdicEndpoint {
    fn name() -> &'static str;
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ResponseMeta {
    pub index: Option<ResponseMetaIndex>,
    pub parameters: Option<serde_json::Value>, // Or a more specific struct if you want
    pub total: Option<u64>,
}

/// Represents the 'totals' object in FDIC API responses (present in all successful endpoint responses).
/// This struct is generic and should be used for any endpoint that returns a 'totals' object.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ResponseTotals {
    /// Total number of records matching the query (may be missing for some endpoints)
    pub count: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ResponseMetaIndex {
    #[serde(rename = "createTimestamp")]
    pub create_timestamp: Option<String>,
    pub name: Option<String>,
}

/// Proxies FDIC BankFind query parameters and endpoint-specific parameters to the FDIC BankFind API, executes the proxied HTTP request.
/// Returns the result or our local FdicProxyError.
pub async fn get_fdic_bank_find_mcp_response<Q, R>(config: &FdicApiConfig, params: &Q) -> Result<R, rmcp::Error>
where
    Q: QueryParameters + FdicEndpoint + Clone + Send + Sync,
    R: Serialize + DeserializeOwned,
{
    // Clone params to allow modification without affecting caller's original
    let mut params = params.clone();
    let common = params.common_mut();

    // Apply validation and normalization to parameters before sending to FDIC API
    // Normalize filters (uppercase everything except quoted substrings)
    if common.filters.is_some() {
        common.filters = crate::param_utils::normalize_filters(&common.filters);
    }

    // Normalize and validate fields if present
    if common.fields.is_some() {
        match crate::param_utils::normalize_fields(&common.fields, Q::VALID_FIELDS) {
            Ok(normalized) => common.fields = normalized,
            Err(err) => {
                log::warn!("Invalid fields parameter: {}", err);
                return Err(rmcp::Error::from(ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid FDIC API field parameter: {}", err),
                    None,
                )));
            }
        }
    }

    // Normalize and validate sort_by if present
    if common.sort_by.is_some() {
        match crate::param_utils::normalize_sort_by(&common.sort_by, Q::VALID_FIELDS) {
            Ok(normalized) => common.sort_by = normalized,
            Err(err) => {
                log::warn!("Invalid sort_by parameter: {}", err);
                return Err(rmcp::Error::from(ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid FDIC API sort_by parameter: {}", err),
                    None,
                )));
            }
        }
    }

    // Normalize sort_order if present (must be ASC or DESC)
    if common.sort_order.is_some() {
        match crate::param_utils::normalize_sort_order(&common.sort_order) {
            Ok(normalized) => common.sort_order = normalized,
            Err(err) => {
                log::warn!("Invalid sort_order parameter: {}", err);
                return Err(rmcp::Error::from(ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid FDIC API sort_order parameter: {}", err),
                    None,
                )));
            }
        }
    }

    let mut query = HashMap::new();
    // Add common parameters if they exist (uppercasing relevant fields)
    if let Some(v) = &common.api_key {
        query.insert("api_key".into(), v.clone());
    }
    if let Some(v) = &common.filters {
        query.insert("filters".into(), v.to_uppercase());
    }
    if let Some(v) = &common.fields {
        query.insert("fields".into(), v.to_uppercase());
    }
    if let Some(v) = &common.sort_by {
        query.insert("sort_by".into(), v.to_uppercase());
    }
    if let Some(v) = &common.sort_order {
        query.insert("sort_order".into(), v.to_uppercase());
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
    let url = format!("{}/{}", config.base_url.trim_end_matches('/'), Q::name());

    log::debug!("Sending FDIC request: URL={}, Query={:?}", url, query);

    // --- Execute Request ---
    let res = client
        .get(&url)
        .query(&query)
        .send()
        .await
        .map_err(|e| reqwest_to_rmcp_error(e))?;

    let status = res.status();
    log::debug!("Received FDIC response status: {}", status);

    // Enforce Content-Length limit if present
    let body_limit = get_fdic_max_allowed_content_length();
    if let Some(content_length) = res.content_length() {
        if content_length > body_limit as u64 {
            log::error!("FDIC response Content-Length {} exceeds limit {}", content_length, body_limit);
            return Err(rmcp::Error::from(ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("FDIC API response body too large ({} bytes, limit is {})", content_length, body_limit),
                Some(json!({
                    "content_length": content_length,
                    "limit": body_limit,
                })),
            )));
        }
    }
    let bytes = res.bytes().await.map_err(|e| reqwest_to_rmcp_error(e))?;

    // --- Parse Response ---
    match serde_json::from_slice::<serde_json::Value>(&bytes) {
        Ok(val) => {
            log::debug!("Successfully parsed FDIC JSON response");
            if status.is_client_error() || status.is_server_error() {
                let message = val
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown FDIC API error")
                    .to_string();
                log::warn!("FDIC API returned error status {}: {}", status, message);
                let custom_code = format!("FDIC_API_ERROR_{}", status.as_u16());
                let error_data = ErrorData::new(
                    ErrorCode::INTERNAL_ERROR,
                    message,
                    Some(json!({
                        "source": "fdic_api",
                        "original_code": custom_code,
                        "status": status.as_u16(),
                        "fdic_raw": val
                    })),
                );
                return Err(rmcp::Error::from(error_data));
            }

            let parsed: R = serde_json::from_value(val).map_err(|e| {
                rmcp::model::ErrorData::new(
                    rmcp::model::ErrorCode::INTERNAL_ERROR,
                    format!("Failed to deserialize FDIC API response: {e}"),
                    None,
                )
            })?;

            Ok(parsed)
        }
        Err(e) => {
            log::error!("Failed to parse FDIC response as JSON: {}. Status: {}", e, status);
            Err(serde_json_to_rmcp_error(e))
        }
    }
}

/// Returns the configured body size limit for FDIC API responses (in bytes).
/// Reads FDIC_MAX_RESPONSE_CONTENT_LENGTH from the environment, or uses the default.
fn get_fdic_max_allowed_content_length() -> usize {
    std::env::var("FDIC_MAX_RESPONSE_CONTENT_LENGTH")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(FDIC_MAX_RESPONSE_CONTENT_LENGTH)
}

// Map reqwest errors to rmcp::Error
fn reqwest_to_rmcp_error(e: reqwest::Error) -> rmcp::Error {
    let message = e.to_string();
    let status = e.status().map(|s| s.as_u16());
    let custom_code_str = match e {
        _ if e.is_connect() => "NETWORK_CONNECTION_ERROR",
        _ if e.is_timeout() => "NETWORK_TIMEOUT_ERROR",
        _ if e.is_request() => "HTTP_REQUEST_ERROR",
        _ if e.is_status() => "HTTP_STATUS_ERROR",
        _ if e.is_body() | e.is_decode() => "HTTP_RESPONSE_BODY_ERROR",
        _ => "FDIC_PROXY_ERROR",
    };

    let error_data = ErrorData::new(
        ErrorCode::INTERNAL_ERROR,
        message,
        Some(json!({
            "source": "reqwest",
            "original_code": custom_code_str,
            "status": status,
        })),
    );

    rmcp::Error::from(error_data)
}

// Map serde_json errors to FdicProxyError
fn serde_json_to_rmcp_error(e: serde_json::Error) -> rmcp::Error {
    let error_data = ErrorData::new(
        ErrorCode::INVALID_PARAMS,
        e.to_string(),
        Some(json!({
            "source": "serde_json",
            "original_code": "JSON_PARSING_ERROR",
            "line": e.line(),
            "column": e.column(),
        })),
    );
    rmcp::Error::from(error_data)
}
