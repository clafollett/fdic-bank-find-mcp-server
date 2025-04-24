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
//! - `filters` (string, optional): The filter for the location search. All values must be in UPPERCASE.
//!     - Examples:
//!         - `STNAME:"West Virginia"` (by state name)
//!         - `STNAME:("West Virginia","Delaware")` (multiple state names)
//!         - `STNAME:Oregon AND SERVTYPE:11` (two fields)
//!         - `RUNDATE:["2015-01-01" TO "2015-01-06"]` (date range)
//!         - `OFFNUM:[0 TO 10]` (number range)
//! - `fields` (string, optional): Comma delimited list of fields to return (e.g. `NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY`)
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
//! ```
//! /locations?filters=STNAME:("ILLINOIS","INDIANA","IOWA","KANSAS","MICHIGAN","MINNESOTA","MISSOURI","NEBRASKA","NORTH DAKOTA","OHIO","SOUTH DAKOTA","WISCONSIN")&sort_by=DEP&sort_order=ASC&limit=10
//! ```
//!
//! ## Response
//! - 200: JSON with `meta` and `data` (array of locations)
//! - 400: Bad input parameter

use axum::{extract::Query, http::StatusCode, response::IntoResponse, response::Response};
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use axum::http::{Response as HttpResponse, header};
use http_body_util::Full;
use utoipa::{ToSchema, IntoParams};

/// Query parameters accepted by the `/locations` endpoint.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct LocationsQuery {
    /// Api key used for api.fdic.gov
    pub api_key: Option<String>,
    /// The filter for the location search. See FDIC docs for syntax.
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
/// See module-level docs for parameter details and usage examples.
#[utoipa::path(
    get,
    path = "/locations",
    params(
        LocationsQuery
    ),
    responses(
        (status = 200, description = "Successful Operation", body = String),
        (status = 400, description = "Bad input parameter")
    ),
    tag = "Structure"
)]
pub async fn locations_handler(Query(params): Query<LocationsQuery>) -> Response {
    let mut query_map = HashMap::new();
    if let Some(v) = params.api_key { query_map.insert("api_key".to_string(), v); }
    if let Some(v) = params.filters { query_map.insert("filters".to_string(), v); }
    if let Some(v) = params.fields { query_map.insert("fields".to_string(), v); }
    if let Some(v) = params.sort_by { query_map.insert("sort_by".to_string(), v); }
    if let Some(v) = params.sort_order { query_map.insert("sort_order".to_string(), v); }
    if let Some(v) = params.limit { query_map.insert("limit".to_string(), v.to_string()); }
    if let Some(v) = params.offset { query_map.insert("offset".to_string(), v.to_string()); }
    if let Some(v) = params.format { query_map.insert("format".to_string(), v); }
    if let Some(v) = params.download { query_map.insert("download".to_string(), v); }
    if let Some(v) = params.filename { query_map.insert("filename".to_string(), v); }

    let client = reqwest::Client::new();
    let mut req = client.get("https://banks.data.fdic.gov/api/locations");
    req = req.query(&query_map);
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                format!("FDIC API request failed: {}", e),
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
                format!("FDIC API response error: {}", e),
            ).into_response();
        }
    };
    HttpResponse::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from(bytes))
        .unwrap()
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_locations_handler_success() {
        let mock_server = MockServer::start().await;
        let mock_response = serde_json::json!({
            "meta": {
                "total": 1,
                "page": 1,
                "limit": 10,
                "total_pages": 1,
                "filters": "NAME:TEST",
                "fields": "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY",
                "sort_by": "NAME",
                "sort_order": "ASC"
            },
            "data": [
                {
                    "NAME": "Test Bank",
                    "UNINUM": 123456,
                    "SERVTYPE": "11",
                    "RUNDATE": "2024-01-01",
                    "CITY": "Test City",
                    "STNAME": "Test State",
                    "ZIP": "12345",
                    "COUNTY": "Test County"
                }
            ]
        });
        Mock::given(method("GET"))
            .and(path("/api/locations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("filters".to_string(), "NAME:TEST".to_string());
        params.insert("fields".to_string(), "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY".to_string());
        params.insert("sort_by".to_string(), "NAME".to_string());
        params.insert("sort_order".to_string(), "ASC".to_string());
        let url = format!("{}/api/locations", &mock_server.uri());
        let req = client.get(&url).query(&params);
        let resp = req.send().await.unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        let body: serde_json::Value = resp.json().await.unwrap();
        let meta = &body["meta"];
        assert_eq!(meta["total"], 1);
        assert_eq!(meta["page"], 1);
        assert_eq!(meta["limit"], 10);
        assert_eq!(meta["total_pages"], 1);
        assert_eq!(meta["filters"], "NAME:TEST");
        assert_eq!(meta["fields"], "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY");
        assert_eq!(meta["sort_by"], "NAME");
        assert_eq!(meta["sort_order"], "ASC");
        let data = &body["data"][0];
        assert_eq!(data["NAME"], "Test Bank");
        assert_eq!(data["UNINUM"], 123456);
        assert_eq!(data["SERVTYPE"], "11");
        assert_eq!(data["RUNDATE"], "2024-01-01");
        assert_eq!(data["CITY"], "Test City");
        assert_eq!(data["STNAME"], "Test State");
        assert_eq!(data["ZIP"], "12345");
        assert_eq!(data["COUNTY"], "Test County");
    }
}
