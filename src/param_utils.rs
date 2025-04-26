//! Shared parameter validation and transformation utilities for FDIC MCP handlers.
//! These functions ensure consistent validation and uppercasing of query parameters across all endpoint handlers.
use crate::common::CommonParameters;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashSet;

/// Normalizes and uppercases a comma-delimited list of fields against the allowed set.
/// Returns Ok(Some(uppercased_csv)) if all fields are valid, Err(message) if any are invalid.
pub fn normalize_fields(fields: &Option<String>, valid_fields: &[&str]) -> Result<Option<String>, String> {
    if let Some(fields_str) = fields {
        let valid: HashSet<_> = valid_fields
            .iter()
            .map(|f| f.to_ascii_uppercase())
            .collect();
        let mut invalid = Vec::new();
        let uppercased: Vec<_> = fields_str
            .split(',')
            .map(|f| f.trim().to_ascii_uppercase())
            .inspect(|uc| {
                if !valid.contains(uc) {
                    invalid.push(uc.clone());
                }
            })
            .collect();
        if !invalid.is_empty() {
            return Err(format!("Invalid field(s): {}", invalid.join(", ")));
        }
        Ok(Some(uppercased.join(",")))
    } else {
        Ok(None)
    }
}

/// Uppercases filters string, except quoted substrings.
pub fn normalize_filters(filters: &Option<String>) -> Option<String> {
    if let Some(filters) = filters {
        let mut up = String::with_capacity(filters.len());
        let mut in_quote = false;
        for c in filters.chars() {
            match c {
                '"' => {
                    in_quote = !in_quote;
                    up.push(c);
                }
                c if !in_quote => up.push(c.to_ascii_uppercase()),
                c => up.push(c),
            }
        }
        Some(up)
    } else {
        None
    }
}

/// Normalizes and uppercases the sort_by field.
/// Returns Ok(Some(uppercased)) if valid, Err(message) if invalid.
pub fn normalize_sort_by(sort_by: &Option<String>, valid_fields: &[&str]) -> Result<Option<String>, String> {
    if let Some(field) = sort_by {
        let uc = field.trim().to_ascii_uppercase();
        if !valid_fields.iter().any(|f| f.eq_ignore_ascii_case(&uc)) {
            return Err(format!("Invalid sort_by field: {}", field));
        }
        Ok(Some(uc))
    } else {
        Ok(None)
    }
}

/// Normalizes and uppercases the sort_order parameter. Only allows "ASC" or "DESC" (case-insensitive).
/// Returns Ok(Some("ASC")) or Ok(Some("DESC")), or Ok(None) if not present. Returns Err(message) if invalid.
pub fn normalize_sort_order(sort_order: &Option<String>) -> Result<Option<String>, String> {
    if let Some(s) = sort_order {
        let upper = s.to_ascii_uppercase();
        if upper == "ASC" || upper == "DESC" {
            Ok(Some(upper))
        } else {
            Err("sort_order must be ASC or DESC".to_string())
        }
    } else {
        Ok(None)
    }
}

/// Uppercases a parameter if present.
pub fn uppercase_param(param: &Option<String>) -> Option<String> {
    param.as_ref().map(|s| s.to_ascii_uppercase())
}

/// Validates that an optional limit does not exceed a maximum. Returns Ok(()) if valid, Err(message) if exceeded.
pub fn validate_limit(limit: Option<u32>, max: u32) -> Result<(), String> {
    if let Some(lim) = limit {
        if lim > max {
            return Err(format!("limit must be <= {}", max));
        }
    }
    Ok(())
}

/// Validate and normalize all common parameters in one shot.
#[allow(dead_code)]
pub fn validate_common_params(common: &mut CommonParameters, valid_fields: &[&str]) -> Result<(), (StatusCode, impl IntoResponse)> {
    use crate::common::{MCPError, MCPErrorDetail};
    // limit
    validate_limit(common.limit, 10000).map_err(|msg| {
        let err = MCPError {
            error_type: "error".to_string(),
            error: MCPErrorDetail {
                kind: "invalid_request_error".to_string(),
                message: msg,
                status: Some(StatusCode::BAD_REQUEST.as_u16()),
                detail: None,
                source: None,
                meta: None,
                fdic_raw: None,
            },
        };
        (StatusCode::BAD_REQUEST, axum::Json(err))
    })?;
    // sort_order
    common.sort_order = normalize_sort_order(&common.sort_order).map_err(|msg| {
        let err = MCPError {
            error_type: "error".to_string(),
            error: MCPErrorDetail {
                kind: "invalid_request_error".to_string(),
                message: msg,
                status: Some(StatusCode::BAD_REQUEST.as_u16()),
                detail: None,
                source: None,
                meta: None,
                fdic_raw: None,
            },
        };
        (StatusCode::BAD_REQUEST, axum::Json(err))
    })?;
    // sort_by
    common.sort_by = normalize_sort_by(&common.sort_by, valid_fields).map_err(|msg| {
        let err = MCPError {
            error_type: "error".to_string(),
            error: MCPErrorDetail {
                kind: "invalid_request_error".to_string(),
                message: msg,
                status: Some(StatusCode::BAD_REQUEST.as_u16()),
                detail: None,
                source: None,
                meta: None,
                fdic_raw: None,
            },
        };
        (StatusCode::BAD_REQUEST, axum::Json(err))
    })?;
    // fields
    common.fields = normalize_fields(&common.fields, valid_fields).map_err(|msg| {
        let err = MCPError {
            error_type: "error".to_string(),
            error: MCPErrorDetail {
                kind: "invalid_request_error".to_string(),
                message: msg,
                status: Some(StatusCode::BAD_REQUEST.as_u16()),
                detail: None,
                source: None,
                meta: None,
                fdic_raw: None,
            },
        };
        (StatusCode::BAD_REQUEST, axum::Json(err))
    })?;
    // filters
    common.filters = normalize_filters(&common.filters);
    // format, filename uppercase
    common.file_format = uppercase_param(&common.file_format);
    common.file_download = common.file_download;
    common.file_name = common.file_name.clone();
    Ok(())
}
