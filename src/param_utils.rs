//! Shared parameter utilities for FDIC MCP handlers.
//! All validation is now deferred to the FDIC BankFind API. Only proxy/transformation helpers remain.
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
