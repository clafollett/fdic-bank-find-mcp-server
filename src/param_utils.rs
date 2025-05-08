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

/// Uppercases only field names (before the colon) in FDIC filters, never values (after the colon).
/// Leaves all values untouched, regardless of quotes. E.g.:
///   CITY:Chicago => CITY:Chicago
///   CITY:"Chicago" AND STATE:IL => CITY:"Chicago" AND STATE:IL
pub fn normalize_filters(filters: &Option<String>) -> Option<String> {
    if let Some(filters) = filters {
        let mut out = String::with_capacity(filters.len());
        let mut chars = filters.chars().peekable();
        while let Some(&c) = chars.peek() {
            // Copy whitespace and parens as-is
            if c.is_whitespace() || c == '(' || c == ')' {
                out.push(c);
                chars.next();
                continue;
            }
            // Try to parse FIELD:VALUE
            let mut field = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == ':' {
                    break;
                }
                if next_c.is_whitespace() || next_c == '(' || next_c == ')' {
                    break;
                }
                field.push(next_c);
                chars.next();
            }
            if !field.is_empty() && chars.peek() == Some(&':') {
                // Found FIELD: pattern
                out.push_str(&field.to_ascii_uppercase());
                out.push(':');
                chars.next(); // consume the colon
                // Copy value verbatim until whitespace, paren, or end
                let mut in_quotes = false;
                while let Some(&val_c) = chars.peek() {
                    if val_c == '"' {
                        in_quotes = !in_quotes;
                    }
                    if !in_quotes && (val_c.is_whitespace() || val_c == '(' || val_c == ')') {
                        break;
                    }
                    out.push(val_c);
                    chars.next();
                }
            } else {
                // Not a field:value, just copy one char and move on
                if !field.is_empty() {
                    out.push_str(&field);
                } else {
                    out.push(c);
                    chars.next();
                }
            }
        }
        Some(out)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_filters_preserves_quoted_city() {
        let input = Some("CITY:\"Chicago\" AND ACTIVE:1".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("CITY:\"Chicago\" AND ACTIVE:1".to_string()));
    }

    #[test]
    fn test_normalize_filters_uppercases_unquoted_city() {
        let input = Some("CITY:Chicago AND ACTIVE:1".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("CITY:Chicago AND ACTIVE:1".to_string()));
    }

    #[test]
    fn test_normalize_filters_preserves_case_inside_quotes() {
        let input = Some("CITY:\"cHiCaGo\" AND ACTIVE:1".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("CITY:\"cHiCaGo\" AND ACTIVE:1".to_string()));
    }

    #[test]
    fn test_normalize_filters_handles_multiple_quotes() {
        let input = Some("CITY:\"Chicago\" AND STATE:\"IL\"".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("CITY:\"Chicago\" AND STATE:\"IL\"".to_string()));
    }

    #[test]
    fn test_normalize_filters_empty() {
        let input: Option<String> = None;
        let output = normalize_filters(&input);
        assert_eq!(output, None);
    }

    #[test]
    fn test_normalize_filters_nested_quotes() {
        let input = Some("CITY:\"Chi\"cago\" AND ACTIVE:1".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("CITY:\"Chi\"cago\" AND ACTIVE:1".to_string()));
    }

    #[test]
    fn test_normalize_filters_whitespace_and_ops() {
        let input = Some("  AND  (  )  ".to_string());
        let output = normalize_filters(&input);
        assert_eq!(output, Some("  AND  (  )  ".to_string()));
    }
}
