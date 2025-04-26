use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde_yaml::{Deserializer, Value};
use std::{fs, path::PathBuf};
use tera::{Context, Tera};
use tracing::{debug, error, trace, warn};

/// CLI tool to generate handler stubs from YAML specs.
#[derive(Parser)]
#[command(name = "generate-handlers", about = "Generate Axum handlers from spec YAML files.")]
struct Cli {
    /// Directory containing spec YAML files
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/public/fdic"))]
    fdic_yaml_dir: PathBuf,

    /// Output directory for generated handlers
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/src/handlers"))]
    handler_dir: PathBuf,
}

// Helper to generate snake_case Rust identifiers from parameter names
fn slugify(s: &str) -> String {
    // Basic slug: non-alphanumeric -> '_'
    let slug: String = s
        .chars()
        .map(|c| {
            let lc = c.to_ascii_lowercase();
            if lc.is_ascii_alphanumeric() { lc } else { '_' }
        })
        .collect();

    // Collapse consecutive underscores into one
    let mut result = String::with_capacity(slug.len());
    let mut prev_underscore = false;
    for ch in slug.chars() {
        if ch == '_' {
            if !prev_underscore {
                result.push(ch);
                prev_underscore = true;
            }
        } else {
            result.push(ch);
            prev_underscore = false;
        }
    }

    // Trim leading/trailing underscores
    let trimmed = result.trim_matches('_');
    let mut name = if trimmed.is_empty() { "field".to_string() } else { trimmed.to_string() };

    // Prefix underscore if it starts with a digit
    if name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        name = format!("_{}", name);
    }

    name
}

/// Sanitizes Markdown for Rust doc comments and Swagger UI.
///
/// - Converts triple-backtick code blocks (```...```) to indented code blocks (4 spaces per line).
/// - Preserves inline code (single backticks).
/// - Leaves all other Markdown formatting unchanged.
///
/// # Arguments
/// * `input` - The Markdown string to sanitize.
///
/// # Returns
/// A sanitized String suitable for embedding in Rust doc comments and OpenAPI docs.
fn sanitize_markdown(input: &str) -> String {
    use regex::Regex;
    // Replace tabs with spaces, trim whitespace, collapse blank lines, escape quotes/backslashes
    let mut out = String::with_capacity(input.len());
    let mut in_code_block = false;
    let mut last_blank = false;

    // Regex for problematic Unicode (e.g., smart quotes, em-dash)
    let unicode_cleanup = Regex::new(r"[\u2018\u2019\u201C\u201D\u2014]").unwrap();

    for line in input.lines() {
        let mut line = line.replace('\t', "    ");
        // Remove problematic Unicode using regex
        line = unicode_cleanup
            .replace_all(&line, |caps: &regex::Captures| match &caps[0] {
                "\u{2018}" | "\u{2019}" => "'",
                "\u{201C}" | "\u{201D}" => "\"",
                "\u{2014}" => "-",
                _ => "",
            })
            .to_string();
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            continue; // skip the ``` line
        }
        if in_code_block {
            // Inside code block: DO NOT escape quotes or backslashes
            out.push_str("    "); // 4 spaces for code block
            out.push_str(&line);
            out.push('\n');
            continue;
        }
        // Outside code blocks: escape double quotes and backslashes for Rust doc attributes
        let mut safe_line = line.replace("\\", "\\\\").replace('"', "\\\"");
        // Optionally escape `{`, `}`, `[`, `]` outside code blocks
        safe_line = safe_line
            .replace("{", "&#123;")
            .replace("}", "&#125;")
            .replace("[", "&#91;")
            .replace("]", "&#93;");
        // Collapse multiple blank lines
        if safe_line.trim().is_empty() {
            if last_blank {
                continue;
            }
            last_blank = true;
        } else {
            last_blank = false;
        }
        out.push_str(&safe_line);
        out.push('\n');
    }
    out.trim_end().to_string()
}

/// Resolve a $ref string (e.g. "#/components/parameters/fileFormat") into a serde_yaml::Value
/// Extracts a parameter definition (mapping) as a serde_json::Value suitable for codegen.
/// Handles OpenAPI parameter fields: name, description, type (via schema)
fn extract_parameter_json(param: &Value) -> Option<serde_json::Value> {
    let pm = param.as_mapping()?;
    let name = pm
        .get(&Value::String("name".to_string()))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let description = pm
        .get(&Value::String("description".to_string()))
        .and_then(Value::as_str)
        .map(sanitize_markdown)
        .unwrap_or_else(String::new);
    let schema = pm
        .get(&Value::String("schema".to_string()))
        .and_then(Value::as_mapping);
    let ty_str = schema
        .and_then(|s| s.get(&Value::String("type".to_string())))
        .and_then(Value::as_str)
        .unwrap_or("string");
    let ty = match ty_str {
        "string" => "String",
        "integer" => "u32",
        "number" => "f64",
        "boolean" => "bool",
        _ => "String",
    }
    .to_string();
    let example = pm
        .get(&Value::String("example".to_string()))
        .map(|ex| {
            if let Some(s) = ex.as_str() {
                serde_json::Value::String(sanitize_markdown(s))
            } else {
                serde_json::to_value(ex).unwrap_or(serde_json::Value::Null)
            }
        })
        .unwrap_or(serde_json::Value::Null);
    Some(serde_json::json!({
        "name": name,
        "rust_name": slugify(&name),
        "ty": ty,
        "description": description,
        "example": example
    }))
}

fn resolve_parameter_ref<'a>(swagger_doc: &'a Value, ref_str: &str) -> Option<&'a Value> {
    let mut node = swagger_doc;
    let path = ref_str.trim_start_matches("#/").split('/');
    for key in path {
        node = node.as_mapping().and_then(|m| m.get(key))?;
    }
    Some(node)
}

/// Extracts the referenced properties YAML file for a given endpoint.
/// Returns the PathBuf to the YAML file, or an error message.
fn extract_properties_yaml_path(
    swagger_doc: &Value, path_item: &serde_yaml::Mapping, fdic_yaml_dir: &PathBuf, endpoint: &str,
) -> Result<std::path::PathBuf, String> {
    let get_item = path_item
        .get(&Value::String("get".to_string()))
        .and_then(Value::as_mapping)
        .ok_or_else(|| format!("No GET operation for endpoint '{}'", endpoint))?;

    let schema_ref = get_item
        .get(&Value::String("responses".to_string()))
        .and_then(Value::as_mapping)
        .and_then(|responses| responses.get(&Value::String("200".to_string())))
        .and_then(Value::as_mapping)
        .and_then(|resp_200| resp_200.get(&Value::String("content".to_string())))
        .and_then(Value::as_mapping)
        .and_then(|content| content.get(&Value::String("application/json".to_string())))
        .and_then(Value::as_mapping)
        .and_then(|app_json| app_json.get(&Value::String("schema".to_string())))
        .and_then(Value::as_mapping)
        .and_then(|schema| {
            schema
                .get(&Value::String("properties".to_string()))
                .and_then(Value::as_mapping)
                .and_then(|props| props.get(&Value::String("data".to_string())))
                .and_then(Value::as_mapping)
                .and_then(|data| {
                    data.get(&Value::String("items".to_string()))
                        .and_then(Value::as_mapping)
                        .and_then(|items| items.get(&Value::String("$ref".to_string())))
                        .and_then(Value::as_str)
                        .map(|s| s.to_string())
                        .or_else(|| {
                            data.get(&Value::String("$ref".to_string()))
                                .and_then(Value::as_str)
                                .map(|s| s.to_string())
                        })
                })
        })
        .ok_or_else(|| format!("Could not find $ref in response schema for endpoint '{}'", endpoint))?;

    let schema_name = schema_ref.split('/').last().unwrap();
    let schema_def = swagger_doc
        .get("components")
        .and_then(Value::as_mapping)
        .and_then(|c| c.get(&Value::String("schemas".to_string())))
        .and_then(Value::as_mapping)
        .and_then(|schemas| schemas.get(&Value::String(schema_name.to_string())))
        .and_then(Value::as_mapping)
        .ok_or_else(|| format!("Could not find schema '{}' in components.schemas", schema_name))?;

    let yaml_ref = schema_def
        .get(&Value::String("allOf".to_string()))
        .and_then(Value::as_sequence)
        .and_then(|all_of| {
            all_of
                .iter()
                .filter_map(|item| item.get("$ref").and_then(Value::as_str))
                .next()
        })
        .ok_or_else(|| format!("Could not find YAML $ref in allOf for schema '{}'.", schema_name))?;

    let yaml_path = fdic_yaml_dir.join(yaml_ref);
    if !yaml_path.exists() {
        return Err(format!(
            "Referenced properties file '{}' does not exist for endpoint '{}'",
            yaml_path.display(),
            endpoint
        ));
    }
    Ok(yaml_path)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load templates
    let mut tera = Tera::default();
    tera.add_template_file("templates/handler.rs.tera", Some("handler"))?;
    tera.add_template_file("templates/handlers_mod.rs.tera", Some("handlers_mod.rs.tera"))?;

    // Load and parse swagger.yaml for all endpoint definitions
    let swagger_path = cli.fdic_yaml_dir.join("swagger.yaml");
    let swagger_str = fs::read_to_string(&swagger_path)?;
    let swagger_doc: Value = serde_yaml::from_str(&swagger_str)?;
    let paths_map: &serde_yaml::Mapping = swagger_doc
        .get("paths")
        .and_then(Value::as_mapping)
        .expect("paths section missing in swagger.yaml");

    // For each path in OpenAPI, generate handler/parameters/properties
    let mut all_endpoints = Vec::new();
    debug!("All endpoints to process:");
    for ep in paths_map.keys() {
        debug!("  >{{{:?}}}<", ep);
    }
    debug!("End of endpoint list");
    for (path_key, path_item) in paths_map.iter() {
        let path_str = path_key.as_str().expect("Path key should be a string");
        // Remove leading slash for endpoint name
        let endpoint = if path_str.starts_with('/') { &path_str[1..] } else { path_str };

        // Use modular helper for extracting properties YAML
        let path_obj = path_item.as_mapping().expect("Path item must be mapping");
        let properties_yaml = match extract_properties_yaml_path(&swagger_doc, path_obj, &cli.fdic_yaml_dir, endpoint) {
            Ok(path) => path,
            Err(e) => {
                trace!("extract_properties_yaml_path failed for endpoint: >{{{}}}<: {}", endpoint, e);
                continue;
            }
        };

        all_endpoints.push(endpoint.to_string());

        let endpoint_cap = endpoint[..1].to_uppercase() + &endpoint[1..];
        let properties_type = format!("{}Properties", endpoint_cap);
        let parameters_type = format!("{}Parameters", endpoint_cap);

        // Robust YAML parse with all error handling, timeouts, and limits
        let mut yaml_str = fs::read_to_string(&properties_yaml)?;
        // Detect and strip UTF-8 BOM if present
        const UTF8_BOM: &str = "\u{feff}";
        if yaml_str.starts_with(UTF8_BOM) {
            debug!("Stripping UTF-8 BOM from start of YAML file: {}", properties_yaml.display());
            yaml_str = yaml_str.trim_start_matches(UTF8_BOM).to_string();
        }
        let max_docs = 20;
        let mut docs = Vec::new();

        for (i, doc) in Deserializer::from_str(&yaml_str).enumerate() {
            if i >= max_docs {
                error!("YAML: Hit max doc limit ({}), aborting parse.", max_docs);
                break;
            }
            match Value::deserialize(doc) {
                Ok(val) => {
                    docs.push(val);
                }
                Err(e) => {
                    error!("Failed to deserialize YAML doc #{}: {}", i, e);
                    break;
                }
            }
        }
        if docs.is_empty() {
            warn!("No YAML docs found or all failed to parse. Trying single-doc parse...");
            match serde_yaml::from_str::<Value>(&yaml_str) {
                Ok(val) => {
                    docs.push(val);
                }
                Err(e) => {
                    error!("Single-doc parse failed: {}", e);
                    continue;
                }
            }
        }

        // Select the first doc with top-level 'properties' BEFORE any use
        let selected_doc = match docs.iter().find(|val| val.get("properties").is_some()) {
            Some(val) => val,
            None => {
                error!("No YAML document with 'properties' found in {}", properties_yaml.display());
                continue;
            }
        };

        // Extract properties for handler
        let mut properties = Vec::new();
        let mut properties_for_handler = Vec::new();
        use std::collections::HashSet;
        if let Some(mapping) = selected_doc
            .get("properties")
            .and_then(Value::as_mapping)
            .and_then(|p| p.get("data"))
            .and_then(Value::as_mapping)
            .and_then(|p| p.get("properties"))
            .and_then(Value::as_mapping)
        {
            let mut used_names = HashSet::new();
            for (k, v) in mapping {
                let name = k.as_str().unwrap().to_string();
                let ty = match v.get("type").and_then(|t| t.as_str()) {
                    Some("string") => "String",
                    Some("number") => "f64",
                    Some("integer") => "i64",
                    Some("boolean") => "bool",
                    _ => "String",
                }
                .to_string();
                let title = v
                    .get("title")
                    .and_then(|t| t.as_str())
                    .unwrap_or("MISSING TITLE")
                    .to_string();
                let description = v
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("MISSING DESCRIPTION")
                    .to_string();
                // Fallback rust_name: use field name if title is missing
                let mut rust_name = if title.trim().is_empty() { slugify(&name) } else { slugify(&title) };
                if !used_names.insert(rust_name.clone()) {
                    // Duplicate: append _{slugified_key}
                    rust_name = format!("{}_{}", rust_name, slugify(&name));
                    // Ensure uniqueness, just in case
                    let mut n = 2;
                    while !used_names.insert(rust_name.clone()) {
                        rust_name = format!("{}_{}_{}", slugify(&title), slugify(&name), n);
                        n += 1;
                    }
                }

                properties.push(serde_json::json!({
                    "name": name,
                    "rust_name": rust_name,
                    "ty": ty,
                    "title": title,
                    "description": description,
                }));
                // Ensure all string parameters (except for api_key and filename) are uppercased before proxying to FDIC API
                if name != "api_key" && name != "filename" {
                    properties_for_handler.push(name.to_ascii_uppercase());
                } else {
                    properties_for_handler.push(name.clone());
                }
            }
        }

        // Extract operation metadata and parameters from swagger.yaml
        let mut summary = String::new();
        let mut description = String::new();
        let mut tags: Vec<String> = Vec::new();
        let mut parameters = Vec::new();
        let path_obj = path_item.as_mapping().expect("Path item must be mapping");
        if let Some(get_item) = path_obj
            .get(&Value::String("get".to_string()))
            .and_then(Value::as_mapping)
        {
            summary = get_item
                .get(&Value::String("summary".to_string()))
                .and_then(Value::as_str)
                .map(sanitize_markdown)
                .unwrap_or_else(String::new);
            description = get_item
                .get(&Value::String("description".to_string()))
                .and_then(Value::as_str)
                .map(sanitize_markdown)
                .unwrap_or_else(String::new);
            tags = get_item
                .get(&Value::String("tags".to_string()))
                .and_then(Value::as_sequence)
                .map(|seq| {
                    seq.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();
            if let Some(seq) = get_item
                .get(&Value::String("parameters".to_string()))
                .and_then(Value::as_sequence)
            {
                for param in seq {
                    // Handle $ref
                    if let Some(ref_str) = param.get("$ref").and_then(Value::as_str) {
                        if let Some(resolved) = resolve_parameter_ref(&swagger_doc, ref_str) {
                            if let Some(param_json) = extract_parameter_json(resolved) {
                                parameters.push(param_json);
                            }
                        }
                    } else if let Some(pm) = param.as_mapping() {
                        if let Some(param_json) = extract_parameter_json(&Value::Mapping(pm.clone())) {
                            parameters.push(param_json);
                        }
                    }
                }
            }
        }

        // Prepare Tera context
        let mut context = Context::new();
        context.insert("endpoint", &endpoint);
        context.insert("endpoint_cap", &endpoint_cap);
        context.insert("properties_type", &properties_type);
        context.insert("parameters_type", &parameters_type);
        context.insert("properties", &properties);
        context.insert("properties_for_handler", &properties_for_handler);
        context.insert("parameters", &parameters);

        // Insert endpoint-specific parameters (not in common_list)
        let common_parameter_names = vec![
            "api_key",
            "filters",
            "fields",
            "sort_by",
            "sort_order",
            "limit",
            "offset",
            "format",
            "download",
            "filename",
        ];
        let endpoint_specific_parameters: Vec<_> = parameters
            .iter()
            .filter(|p| {
                p.get("name")
                    .and_then(|v| v.as_str())
                    .map(|name| !common_parameter_names.contains(&name))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        let properties_yaml_file_name = properties_yaml
            .file_name()
            .map(|s| s.to_string_lossy().to_string());

        context.insert("common_parameter_names", &common_parameter_names);
        context.insert("endpoint_specific_parameters", &endpoint_specific_parameters);
        context.insert("summary", &summary);
        context.insert("description", &description);
        context.insert("spec_file", &properties_yaml_file_name);

        // Ensure at least one tag for handler macro
        if tags.is_empty() {
            tags.push(endpoint_cap.clone());
        }

        context.insert("tags", &tags);

        // Compute valid_fields: all uppercase property names except api_key/filename
        let valid_fields: Vec<String> = properties_for_handler
            .iter()
            .filter(|f| *f != "api_key" && *f != "filename")
            .cloned()
            .collect();
        context.insert("valid_fields", &valid_fields);

        // Render and write handler stub
        let rendered = tera.render("handler", &context)?;
        let handler_out_path = cli.handler_dir.join(format!("{}.rs", endpoint));
        fs::write(&handler_out_path, rendered)?;
        debug!("Generated handler stub: {}", handler_out_path.display());
    }

    // After all handler files are generated, render the register_handlers.rs.tera template
    let mut reg_ctx = Context::new();
    reg_ctx.insert("endpoints", &all_endpoints);

    // Also generate handlers/mod.rs from handlers_mod.rs.tera
    let mod_rendered = tera.render("handlers_mod.rs.tera", &reg_ctx)?;
    let mod_out_path = cli.handler_dir.join("mod.rs");
    fs::write(&mod_out_path, mod_rendered)?;
    debug!("Generated handler mod.rs: {}", mod_out_path.display());

    Ok(())
}
