//! FDIC BankFind API `/locations` endpoint handler
//!
//! This handler proxies requests to the FDIC BankFind `/locations` endpoint.
//!
//! ## Endpoint: `/locations`
//!
//! **Purpose:** Returns locations/branches of financial institutions.
//!
//! ### Query Parameters
//! - `api_key` (string, optional): Api key used for api.fdic.gov
//! - `filters` (string, optional): The filter for the location search. **Supports all fields and filter syntax documented in the official FDIC BankFind API documentation.**
//!     - Examples:
//!         - `STNAME:"West Virginia"` (by state name)
//!         - `STNAME:("West Virginia","Delaware")` (multiple state names)
//!         - `CITY:"Chicago" AND BKCLASS:NM` (by city and class)
//!         - `!(STNAME:"Virginia")` (exclude a state)
//!         - `DATEUPDT:[2010-01-01 TO 2010-12-31]` (date range)
//!         - `FI_UNINUM:[10000 TO 15000]` (numeric range)
//!     - **See [FDIC Filter Syntax](https://banks.data.fdic.gov/docs/) for the full list of supported fields and syntax.**
//! - `fields` (string, optional): Comma delimited list of fields to return
//! - `sort_by` (string, optional): Field name to sort by (default: `NAME`)
//! - `sort_order` (string, optional): `ASC` or `DESC` (default: `ASC`)
//! - `limit` (integer, optional): Number of records to return (default: 10, max: 10,000)
//! - `offset` (integer, optional): The offset of page to return (default: 0)
//! - `format` (string, optional): File format (e.g., `json`, `csv`)
//! - `download` (string, optional): If set, triggers file download
//! - `filename` (string, optional): Custom file name for download
//!
//! **All parameter values must be in UPPERCASE.**
//!
//! ## Example Midwest Query
//! To get the top 10 banks in the Midwest sorted by deposits ascending:
//!
//! ```text
//! /locations?filters=STNAME:("ILLINOIS","INDIANA","IOWA","KANSAS","MICHIGAN","MINNESOTA","MISSOURI","NEBRASKA","NORTH DAKOTA","OHIO","SOUTH DAKOTA","WISCONSIN")&sort_by=NAME&sort_order=ASC&limit=10
//! ```
//!
//! ## Response
//! - 200: JSON with `meta` and `data` (array of locations)
//! - 400: Bad input parameter

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, response::Response};
use reqwest;
use std::collections::HashMap;
use axum::http::{Response as HttpResponse, header};
use http_body_util::Full;
use utoipa::{ToSchema, IntoParams};
use serde::{Serialize, Deserialize};

/// Config for FDIC BankFind API base URL.
#[derive(Clone, Debug)]
pub struct FdicApiConfig {
    pub base_url: String,
}

/// MCP response for /locations endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationsResponse {
    /// Metadata about the query
    pub meta: serde_json::Value,
    /// Array of location properties
    pub data: Vec<LocationProperties>,
}

/// Properties of a bank location, matching FDIC Bank Find API.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LocationProperties {
    /// Street Address
    #[serde(rename = "ADDRESS")]
    pub address: Option<String>,
    /// Institution Class
    #[serde(rename = "BKCLASS")]
    pub bkclass: Option<String>,
    /// Core Based Statistical Area Name
    #[serde(rename = "CBSA")]
    pub cbsa: Option<String>,
    /// Metropolitan Divisions Name
    #[serde(rename = "CBSA_DIV")]
    pub cbsa_div: Option<String>,
    /// Metropolitan Divisions Flag
    #[serde(rename = "CBSA_DIV_FLG")]
    pub cbsa_div_flg: Option<String>,
    /// Metropolitan Divisions Number
    #[serde(rename = "CBSA_DIV_NO")]
    pub cbsa_div_no: Option<String>,
    /// Metropolitan Division Number
    #[serde(rename = "CBSA_METRO")]
    pub cbsa_metro: Option<String>,
    /// Metropolitan Division Flag
    #[serde(rename = "CBSA_METRO_FLG")]
    pub cbsa_metro_flg: Option<String>,
    /// Metropolitan Division Name
    #[serde(rename = "CBSA_METRO_NAME")]
    pub cbsa_metro_name: Option<String>,
    /// Micropolitan Division Flag
    #[serde(rename = "CBSA_MICRO_FLG")]
    pub cbsa_micro_flg: Option<String>,
    /// Core Based Statistical Areas
    #[serde(rename = "CBSA_NO")]
    pub cbsa_no: Option<String>,
    /// FDIC Certificate #
    #[serde(rename = "CERT")]
    pub cert: Option<String>,
    /// City
    #[serde(rename = "CITY")]
    pub city: Option<String>,
    /// County
    #[serde(rename = "COUNTY")]
    pub county: Option<String>,
    /// Combined Statistical Area Name
    #[serde(rename = "CSA")]
    pub csa: Option<String>,
    /// Combined Statistical Area Flag
    #[serde(rename = "CSA_FLG")]
    pub csa_flg: Option<String>,
    /// Combined Statistical Area Number
    #[serde(rename = "CSA_NO")]
    pub csa_no: Option<String>,
    /// Branch Established Date
    #[serde(rename = "ESTYMD")]
    pub estymd: Option<String>,
    /// FDIC's unique number
    #[serde(rename = "FI_UNINUM")]
    pub fi_uninum: Option<String>,
    /// Latitude
    #[serde(rename = "LATITUDE")]
    pub latitude: Option<f64>,
    /// Longitude
    #[serde(rename = "LONGITUDE")]
    pub longitude: Option<f64>,
    /// Minority Status Code
    #[serde(rename = "MDI_STATUS_CODE")]
    pub mdi_status_code: Option<String>,
    /// Minority Status Description
    #[serde(rename = "MDI_STATUS_DESC")]
    pub mdi_status_desc: Option<String>,
    /// Main Office Flag
    #[serde(rename = "MAINOFF")]
    pub mainoff: Option<u8>,
    /// Institution Name
    #[serde(rename = "NAME")]
    pub name: Option<String>,
    /// Office Name
    #[serde(rename = "OFFNAME")]
    pub offname: Option<String>,
    /// Branch Number
    #[serde(rename = "OFFNUM")]
    pub offnum: Option<String>,
    /// Run Date
    #[serde(rename = "RUNDATE")]
    pub rundate: Option<String>,
    /// Service Type Code
    #[serde(rename = "SERVTYPE")]
    pub servtype: Option<u32>,
    /// State Name
    #[serde(rename = "STNAME")]
    pub stname: Option<String>,
    /// ZIP Code
    #[serde(rename = "ZIP")]
    pub zip: Option<String>,
}

/// Query parameters accepted by the `/locations` endpoint.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct LocationsQuery {
    /// Api key used for api.fdic.gov
    pub api_key: Option<String>,
    /// The filter for the location search. **Supports all fields and filter syntax documented in the official FDIC BankFind API documentation.**
    /// See [FDIC Filter Syntax](https://banks.data.fdic.gov/docs/) for details.
    pub filters: Option<String>,
    /// Comma delimited list of fields to return
    pub fields: Option<String>,
    /// Field name to sort by
    pub sort_by: Option<String>,
    /// ASC or DESC
    pub sort_order: Option<String>,
    /// Number of records to return
    pub limit: Option<u32>,
    /// The offset of page to return
    pub offset: Option<u32>,
    /// File format (e.g., json, csv)
    pub format: Option<String>,
    /// If set, triggers file download
    pub download: Option<String>,
    /// Custom file name for download
    pub filename: Option<String>,
}

/// Proxies requests to the FDIC BankFind `/locations` endpoint.
///
/// - The `filters` parameter supports **all fields and filter syntax documented in the official FDIC BankFind API documentation**.
///   See [FDIC Filter Syntax](https://banks.data.fdic.gov/docs/) for details.
/// - All filter values except `api_key` and `filename` must be uppercased.
///
/// See module-level docs for parameter details and usage examples.
#[utoipa::path(
    get,
    path = "/locations",
    params(
        LocationsQuery
    ),
    responses(
        (status = 200, description = "Successful Operation", body = LocationsResponse),
        (status = 400, description = "Bad input parameter")
    ),
    tag = "Structure"
)]
pub async fn locations_handler(
    State(config): State<FdicApiConfig>,
    Query(params): Query<LocationsQuery>
) -> Response {


    use axum::Json;
    use serde_json::json;

    // Validate limit (must be <= 10_000)
    if let Some(limit) = params.limit {
        if limit > 10_000 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Limit must be <= 10,000"
                }))
            ).into_response();
        }
    }
    // Validate sort_order if present
    if let Some(ref order) = params.sort_order {
        let upper = order.to_uppercase();
        if upper != "ASC" && upper != "DESC" {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "sort_order must be 'ASC' or 'DESC'"
                }))
            ).into_response();
        }
    }
    // Build query map, uppercasing all string values except api_key, filename
    let mut query_map = HashMap::new();
    if let Some(v) = params.api_key.as_ref() { query_map.insert("api_key".to_string(), v.clone()); }
    if let Some(v) = params.filters.as_ref() { query_map.insert("filters".to_string(), v.to_uppercase()); }
    if let Some(v) = params.fields.as_ref() { query_map.insert("fields".to_string(), v.to_uppercase()); }
    if let Some(v) = params.sort_by.as_ref() { query_map.insert("sort_by".to_string(), v.to_uppercase()); }
    if let Some(v) = params.sort_order.as_ref() { query_map.insert("sort_order".to_string(), v.to_uppercase()); }
    if let Some(v) = params.limit { query_map.insert("limit".to_string(), v.to_string()); }
    if let Some(v) = params.offset { query_map.insert("offset".to_string(), v.to_string()); }
    if let Some(v) = params.format.as_ref() { query_map.insert("format".to_string(), v.to_uppercase()); }
    if let Some(v) = params.download.as_ref() { query_map.insert("download".to_string(), v.to_uppercase()); }
    if let Some(v) = params.filename.as_ref() { query_map.insert("filename".to_string(), v.clone()); }

    let client = reqwest::Client::new();
    let mut req = client.get(&format!("{}/locations", config.base_url.trim_end_matches('/')));
    req = req.query(&query_map);
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": format!("FDIC API request failed: {}", e) }))
            ).into_response();
        }
    };
    let status = axum::http::StatusCode::from_u16(resp.status().as_u16())
        .unwrap_or(StatusCode::BAD_GATEWAY);
    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": format!("FDIC API response error: {}", e) }))
            ).into_response();
        }
    };
    // Try to parse and reformat as MCP (meta, data)
    let maybe_json: Result<serde_json::Value, _> = serde_json::from_slice(&bytes);
    if let Ok(val) = maybe_json {
        if val.get("meta").is_some() && val.get("data").is_some() {
            return (
                status,
                Json(val)
            ).into_response();
        } else {
            // Wrap as MCP if possible
            return (
                status,
                Json(json!({ "meta": {}, "data": val }))
            ).into_response();
        }
    }
    // Fallback: return raw bytes
    HttpResponse::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from(bytes))
        .unwrap()
        .into_response()
}
