// =============================
// Imports
// =============================
use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml::{Deserializer, Mapping, Value as YamlValue};
use std::{fs, path::PathBuf};
use tera::{Context, Tera};
use tracing::{debug, error, trace};
use tracing_subscriber::{EnvFilter, fmt};

// =============================
// Type Definitions
// =============================

/// CLI tool to generate handler stubs from YAML specs.
#[derive(Parser)]
#[command(name = "generate-handlers", about = "Generate Axum handlers from spec YAML files.")]
pub struct Cli {
    /// Directory containing spec YAML files
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/public/fdic"))]
    pub fdic_yaml_dir: PathBuf,

    /// Output directory for generated handlers
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/src/handlers"))]
    pub handler_dir: PathBuf,

    /// Output path for generated OpenAPI JSON (openapi.json)
    #[arg(long, default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/public/openapi.json"))]
    pub openapi_json_path: PathBuf,

    /// Whether to render the OpenAPI JSON file (default: true)
    #[arg(long, default_value_t = true)]
    pub render_openapi_json: bool,
}

/// Canonical struct for extracted property info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyInfo {
    pub name: String,
    pub rust_type: String,
    pub openapi_type: String,
    pub format: Option<String>,
    pub title: String,
    pub description: String,
    pub example: String,
    pub required: bool,
    pub schema: serde_json::Value,
    pub elastic_type: Option<String>,
}

/// Canonical struct for extracted parameter info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub rust_type: String,
    pub openapi_type: String,
    pub format: Option<String>,
    pub description: String,
    pub description_raw: String,
    pub example: String,
    pub example_raw: String,
    pub required: bool,
    pub schema: serde_json::Value,
}

// =============================
// Main Function
// =============================

/// Main entry point for the CLI tool.
fn main() -> Result<()> {
    dotenv().ok();

    fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Load templates
    let mut tera = Tera::default();
    tera.add_template_file("templates/handler.rs.tera", Some("handler"))?;
    tera.add_template_file("templates/handlers_mod.rs.tera", Some("handlers_mod.rs.tera"))?;

    // Load and parse swagger.yaml for all endpoint definitions
    let swagger_path = cli.fdic_yaml_dir.join("swagger.yaml");
    let swagger_str = fs::read_to_string(&swagger_path)?;
    let swagger_doc: YamlValue = serde_yaml::from_str(&swagger_str)?;
    let paths_map: &Mapping = swagger_doc
        .get("paths")
        .and_then(YamlValue::as_mapping)
        .expect("paths section missing in swagger.yaml");

    // Debug: print out the path and YAML structure for troubleshooting
    debug!("swagger_path: {:?}", swagger_path);

    // For each path in OpenAPI, generate handler/parameters/properties
    let mut all_endpoints = Vec::new();
    trace!("All endpoints to process:");

    trace!("Start of endpoint list");
    for ep in paths_map.keys() {
        trace!("  >{{{:?}}}<", ep);
    }
    trace!("End of endpoint list");

    let mut endpoint_data: Vec<serde_json::Value> = Vec::new();
    for (path_key, path_item) in paths_map.iter() {
        let path_str = path_key.as_str().expect("Path key should be a string");
        let endpoint = if path_str.starts_with('/') { &path_str[1..] } else { path_str };
        let path_obj = path_item.as_mapping().expect("Path item must be mapping");
        let (properties_yaml, spec_file_name) = match extract_properties_yaml_value(&swagger_doc, path_obj, &cli.fdic_yaml_dir, endpoint) {
            Ok((doc, spec_file)) => {
                let file_name_only = spec_file.as_ref().map(|s| {
                    std::path::Path::new(s)
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| s.clone())
                });
                (doc, file_name_only)
            }
            Err(e) => {
                trace!("extract_properties_yaml_value failed for endpoint: >{{{}}}<: {}", endpoint, e);
                (YamlValue::Null, None)
            }
        };
        let endpoint_cap = endpoint[..1].to_uppercase() + &endpoint[1..];
        let parameters_type = format!("{}Parameters", endpoint_cap);

        // Use helper for YAML doc loading
        let selected_doc = properties_yaml;
        // Use helper for metadata
        let (summary, description, tags) = extract_operation_metadata(path_obj);
        // Extract parameters for handler using canonical extraction (with $ref support)
        let parameters = extract_parameters_for_handler(&swagger_doc, path_obj);
        // Extract the actual row/item properties for the Properties struct
        let row_properties = extract_row_properties(&selected_doc);
        // Extract envelope properties for the Response struct (meta, data, totals, etc)
        let envelope_properties = selected_doc
            .get("properties")
            .and_then(YamlValue::as_mapping)
            .map(|mapping| extract_properties(mapping))
            .unwrap_or_default();
        // Canonical: Build endpoint metadata struct for both handler and OpenAPI, no duplication
        let properties_struct_name = format!("{}Properties", endpoint[..1].to_uppercase() + &endpoint[1..]);
        let response_struct_name = format!("{}Response", endpoint[..1].to_uppercase() + &endpoint[1..]);
        let properties_schema = build_properties_schema(&row_properties);
        let response_schema = build_response_schema(&properties_struct_name);
        let valid_fields: Vec<String> = row_properties
            .iter()
            .map(|p| p.name.to_ascii_uppercase())
            .collect();
        let endpoint_obj = serde_json::json!({
            "endpoint": endpoint,
            "endpoint_cap": endpoint_cap,
            "parameters_type": parameters_type,
            "properties_type": properties_struct_name,
            "envelope_properties": serde_json::to_value(&envelope_properties).unwrap(),
            "properties": serde_json::to_value(&row_properties).unwrap(),
            "properties_for_handler": row_properties.iter().map(|p| p.name.to_ascii_uppercase()).collect::<Vec<_>>(),
            "parameters": serde_json::to_value(&parameters).unwrap(),
            "common_parameter_names": vec![
                "api_key", "filters", "fields", "sort_by", "sort_order",
                "limit", "offset", "format", "download", "filename"
            ],
            "endpoint_specific_parameters": row_properties
                .iter()
                .filter(|p| {
                    p.name != "api_key"
                        && p.name != "filters"
                        && p.name != "fields"
                        && p.name != "sort_by"
                        && p.name != "sort_order"
                        && p.name != "limit"
                        && p.name != "offset"
                        && p.name != "format"
                        && p.name != "download"
                        && p.name != "filename"
                })
                .map(|p| p.name.to_ascii_uppercase())
                .collect::<Vec<_>>(),
            "summary": summary,
            "description": description,
            "tags": tags,
            "properties_struct_name": properties_struct_name,
            "response_struct_name": response_struct_name,
            "properties_schema": properties_schema,
            "response_schema": response_schema,
            "spec_file_name": spec_file_name,
            "valid_fields": valid_fields,
        });
        endpoint_data.push(endpoint_obj);

        // Use helper for context
        let context = context_from_endpoint_obj(endpoint_data.last().unwrap());
        // Render and write handler stub
        let rendered = tera.render("handler", &context)?;
        let handler_out_path = cli.handler_dir.join(format!("{}.rs", endpoint));

        fs::write(&handler_out_path, rendered)?;
        all_endpoints.push(endpoint.to_string());

        debug!("Generated handler stub: {}", handler_out_path.display());
    }
    // After all handlers are parsed and schemas collected, add metadata and totals schemas from swagger.yaml
    let mut schemas = serde_json::Map::new();
    add_metadata_and_totals_schemas(&swagger_doc, &mut schemas);

    // After all handler files are generated, render the register_handlers.rs.tera template
    let mut reg_ctx = Context::new();
    reg_ctx.insert("endpoints", &all_endpoints);

    // Also generate handlers/mod.rs from handlers_mod.rs.tera
    let mod_rendered = tera.render("handlers_mod.rs.tera", &reg_ctx)?;
    let mod_out_path = cli.handler_dir.join("mod.rs");
    fs::write(&mod_out_path, mod_rendered)?;
    debug!("Generated handler mod.rs: {}", mod_out_path.display());

    if cli.render_openapi_json {
        // Use endpoint_data for OpenAPI generation (single source of truth)
        generate_openapi_json(&all_endpoints, &endpoint_data, &cli.openapi_json_path, &schemas)?;
        debug!("Generated OpenAPI JSON: {}", cli.openapi_json_path.display());
    }
    Ok(())
}

// =============================
// Public Functions (alphabetical)
// =============================

/// Extracts the metadata and totals schemas from a loaded swagger.yaml Value and adds them to the schemas map.
pub fn add_metadata_and_totals_schemas(
    swagger_yaml: &serde_yaml::Value,
    schemas: &mut serde_json::Map<String, serde_json::Value>,
) {
    let components = swagger_yaml.get("components").and_then(|c| c.get("schemas"));
    if let Some(schemas_map) = components.and_then(|s| s.as_mapping()) {
        if let Some(metadata_schema) = schemas_map.get(&serde_yaml::Value::String("metadata".to_string())) {
            schemas.insert(
                "metadata".to_string(),
                serde_json::to_value(metadata_schema).expect("Failed to convert metadata schema to JSON")
            );
        }
        if let Some(totals_schema) = schemas_map.get(&serde_yaml::Value::String("totals".to_string())) {
            schemas.insert(
                "totals".to_string(),
                serde_json::to_value(totals_schema).expect("Failed to convert totals schema to JSON")
            );
        }
    }
}

/// Extract all parameter info from a serde_yaml::Value (OpenAPI param)
pub fn extract_parameter_info(param: &YamlValue) -> Option<ParameterInfo> {
    let pm = param.as_mapping()?;
    let name = get_str_field(pm, "name");
    let description_raw = get_str_field(pm, "description");
    let description = sanitize_markdown(&description_raw);
    let example_raw = get_str_field(pm, "example");
    let example = sanitize_markdown(&example_raw);
    let required = get_bool_field(pm, "required");
    let schema = get_value_field(pm, "schema");
    let schema_json = serde_json::to_value(schema).unwrap_or(serde_json::Value::Null);
    // Type inference
    let (openapi_type, format) = pm
        .get(&YamlValue::String("schema".to_string()))
        .and_then(YamlValue::as_mapping)
        .map(|s| {
            let ty = s
                .get(&YamlValue::String("type".to_string()))
                .and_then(YamlValue::as_str)
                .unwrap_or("string")
                .to_string();
            let fmt = s
                .get(&YamlValue::String("format".to_string()))
                .and_then(YamlValue::as_str)
                .map(|f| f.to_string());
            (ty, fmt)
        })
        .unwrap_or(("string".to_string(), None));
    let rust_type = openapi_type_to_rust_type(&openapi_type, format.as_deref()).to_string();
    Some(ParameterInfo {
        name,
        rust_type,
        openapi_type,
        format,
        description,
        description_raw,
        example,
        example_raw,
        required,
        schema: schema_json,
    })
}

// =============================
// Small field extraction helpers
// =============================

fn get_str_field<'a>(map: &'a serde_yaml::Mapping, key: &str) -> String {
    map.get(&YamlValue::String(key.to_string()))
        .and_then(YamlValue::as_str)
        .unwrap_or("")
        .to_string()
}

fn get_bool_field(map: &serde_yaml::Mapping, key: &str) -> bool {
    map.get(&YamlValue::String(key.to_string()))
        .and_then(YamlValue::as_bool)
        .unwrap_or(false)
}

fn get_value_field(map: &serde_yaml::Mapping, key: &str) -> YamlValue {
    map.get(&YamlValue::String(key.to_string()))
        .cloned()
        .unwrap_or(YamlValue::Null)
}

// =============================
// Private/Helper Functions (alphabetical)
// =============================

/// Builds a map of property schemas from a slice of PropertyInfo.
fn build_properties_schema(properties: &[PropertyInfo]) -> serde_json::Map<String, serde_json::Value> {
    properties
        .iter()
        .map(|prop| {
            let mut schema = match prop.rust_type.as_str() {
                "String" => serde_json::json!({"type": "string"}),
                "i64" => serde_json::json!({"type": "integer", "format": "int64"}),
                "f64" => serde_json::json!({"type": "number", "format": "double"}),
                "bool" => serde_json::json!({"type": "boolean"}),
                _ => serde_json::json!({"type": "string"}),
            };
            if let Some(fmt) = &prop.format {
                schema["format"] = serde_json::json!(fmt);
            }
            if !prop.example.is_empty() {
                schema["example"] = serde_json::json!(prop.example);
            }
            if !prop.description.is_empty() {
                schema["description"] = serde_json::json!(prop.description);
            }
            if let Some(elastic_type) = &prop.elastic_type {
                schema["x-elastic-type"] = serde_json::json!(elastic_type);
            }
            (prop.name.clone(), schema)
        })
        .collect()
}

/// Builds the OpenAPI response schema for an endpoint.
fn build_response_schema(properties_struct_name: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "meta": {
                "type": "object",
                "description": "Metadata about the query, parameters, and FDIC index info. This is passed through from the upstream FDIC API and may include fields such as total, parameters, and index."
            },
            "data": {
                "type": "array",
                "items": {"$ref": format!("#/components/schemas/{}", properties_struct_name)},
                "description": "Array of result objects for the endpoint. Each object contains the main data fields, and may include additional dynamic fields such as highlight and score if present."
            },
            "totals": {
                "type": "object",
                "description": "Aggregate totals and counts for the query, as provided by the upstream FDIC API. Present only if the FDIC response includes a totals object."
            }
        },
        "description": "MCP response envelope. Core fields are meta, data, and (optionally) totals, matching the FDIC API response structure. Additional dynamic fields may be present in data entries."
    })
}

/// Converts a serde_json::Value endpoint object into a Tera context.
fn context_from_endpoint_obj(obj: &serde_json::Value) -> Context {
    let mut context = Context::new();
    if let Some(map) = obj.as_object() {
        for (k, v) in map {
            context.insert(k, v);
        }
    }
    context
}

/// Extracts summary, description, and tags from a path_item mapping.
fn extract_operation_metadata(path_item: &Mapping) -> (String, String, Vec<String>) {
    let mut summary = String::new();
    let mut description = String::new();
    let mut tags: Vec<String> = Vec::new();
    if let Some(get_item) = path_item
        .get(&YamlValue::String("get".to_string()))
        .and_then(YamlValue::as_mapping)
    {
        summary = get_item
            .get(&YamlValue::String("summary".to_string()))
            .and_then(YamlValue::as_str)
            .map(sanitize_markdown)
            .unwrap_or_else(String::new);
        description = get_item
            .get(&YamlValue::String("description".to_string()))
            .and_then(YamlValue::as_str)
            .map(sanitize_markdown)
            .unwrap_or_else(String::new);
        tags = get_item
            .get(&YamlValue::String("tags".to_string()))
            .and_then(YamlValue::as_sequence)
            .map(|seq| {
                seq.iter()
                    .filter_map(YamlValue::as_str)
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();
    }
    (summary, description, tags)
}

/// Extracts property info from a YamlValue.
fn extract_property_info(name: &str, prop: &YamlValue) -> PropertyInfo {
    let mapping_binding;
    let mapping = match prop.as_mapping() {
        Some(m) => m,
        None => {
            mapping_binding = serde_yaml::Mapping::new();
            &mapping_binding
        }
    };
    let openapi_type = get_str_field(mapping, "type");
    let format = {
        let f = get_str_field(mapping, "format");
        if f.is_empty() { None } else { Some(f) }
    };
    let rust_type = openapi_type_to_rust_type(&openapi_type, format.as_deref()).to_string();
    let title_base = get_str_field(mapping, "title");
    let description_base = get_str_field(mapping, "description");
    let example = get_str_field(mapping, "example");
    let required = false; // Not always present in property schema
    let schema = serde_json::to_value(prop).unwrap_or_default();
    let elastic_type = {
        let e = get_str_field(mapping, "x-elastic-type");
        if e.is_empty() { None } else { Some(e) }
    };
    let (title, description) = if let Some(ref et) = elastic_type {
        if et == "keyword" || et == "text" || et == "searchable" {
            let title = if !title_base.is_empty() {
                format!("{} (Search-Eligible)", title_base)
            } else {
                "Search-Eligible".to_string()
            };
            let description = if !description_base.is_empty() {
                format!("{} This field can be used for search and filtering.", description_base)
            } else {
                "This field can be used for search and filtering.".to_string()
            };
            (title, description)
        } else {
            (title_base, description_base)
        }
    } else {
        (title_base, description_base)
    };
    PropertyInfo {
        name: name.to_string(),
        rust_type,
        openapi_type,
        format,
        title,
        description,
        example,
        required,
        schema,
        elastic_type,
    }
}

/// Extracts properties from a serde_yaml::Mapping.
fn extract_properties(mapping: &serde_yaml::Mapping) -> Vec<PropertyInfo> {
    mapping
        .iter()
        .filter_map(|(k, v)| k.as_str().map(|name| extract_property_info(name, v)))
        .collect()
}

/// Extracts the referenced properties YAML for a given endpoint.
/// Returns a tuple: (parsed YamlValue for the properties, Option<String> for the spec file name).
fn extract_properties_yaml_value(
    swagger_doc: &YamlValue, path_item: &Mapping, fdic_yaml_dir: &PathBuf, endpoint: &str,
) -> Result<(YamlValue, Option<String>), String> {
    let get_item = path_item
        .get(&YamlValue::String("get".to_string()))
        .and_then(YamlValue::as_mapping)
        .ok_or_else(|| {
            error!("No GET operation for endpoint '{}' in path_item", endpoint);
            format!("No GET operation for endpoint '{}'", endpoint)
        })?;
    // Get the schema mapping for the response
    let schema_mapping = get_item
        .get(&YamlValue::String("responses".to_string()))
        .and_then(YamlValue::as_mapping)
        .and_then(|responses| responses.get(&YamlValue::String("200".to_string())))
        .and_then(YamlValue::as_mapping)
        .and_then(|resp_200| resp_200.get(&YamlValue::String("content".to_string())))
        .and_then(YamlValue::as_mapping)
        .and_then(|content| content.get(&YamlValue::String("application/json".to_string())))
        .and_then(YamlValue::as_mapping)
        .and_then(|app_json| app_json.get(&YamlValue::String("schema".to_string())))
        .and_then(YamlValue::as_mapping)
        .ok_or_else(|| {
            error!("Could not find response schema mapping for endpoint '{}'", endpoint);
            format!("Could not find response schema mapping for endpoint '{}'", endpoint)
        })?;
    // Look for properties -> data -> items -> $ref
    let data_items_ref = schema_mapping
        .get(&YamlValue::String("properties".to_string()))
        .and_then(YamlValue::as_mapping)
        .and_then(|props| props.get(&YamlValue::String("data".to_string())))
        .and_then(YamlValue::as_mapping)
        .and_then(|data_map| data_map.get(&YamlValue::String("items".to_string())))
        .and_then(YamlValue::as_mapping)
        .and_then(|items_map| items_map.get(&YamlValue::String("$ref".to_string())))
        .and_then(YamlValue::as_str);
    if let Some(schema_ref) = data_items_ref {
        let schema_name = schema_ref.split('/').last().unwrap_or("");
        let schema_obj = swagger_doc
            .get("components")
            .and_then(YamlValue::as_mapping)
            .and_then(|c| c.get(&YamlValue::String("schemas".to_string())))
            .and_then(YamlValue::as_mapping)
            .and_then(|schemas| schemas.get(&YamlValue::String(schema_name.to_string())))
            .and_then(YamlValue::as_mapping)
            .ok_or_else(|| {
                error!("Could not find schema '{}' in components.schemas for endpoint '{}'", schema_name, endpoint);
                format!("Could not find schema '{}' in components.schemas", schema_name)
            })?;
        // Now look for allOf -> $ref (the YAML file)
        let yaml_ref = schema_obj
            .get(&YamlValue::String("allOf".to_string()))
            .and_then(YamlValue::as_sequence)
            .and_then(|all_of| {
                all_of
                    .iter()
                    .filter_map(|item| item.get("$ref").and_then(YamlValue::as_str))
                    .next()
            });
        if let Some(yaml_ref) = yaml_ref {
            let yaml_path = fdic_yaml_dir.join(yaml_ref);
            if !yaml_path.exists() {
                error!("Referenced properties file '{}' does not exist for endpoint '{}'", yaml_path.display(), endpoint);
                return Err(format!(
                    "Referenced properties file '{}' does not exist for endpoint '{}'",
                    yaml_path.display(),
                    endpoint
                ));
            }
            let mut yaml_str = fs::read_to_string(&yaml_path).map_err(|e| {
                error!("Failed to read properties YAML '{}': {}", yaml_path.display(), e);
                format!("Failed to read properties YAML '{}': {}", yaml_path.display(), e)
            })?;
            const UTF8_BOM: &str = "\u{feff}";
            if yaml_str.starts_with(UTF8_BOM) {
                yaml_str = yaml_str.trim_start_matches(UTF8_BOM).to_string();
            }
            let max_docs = 20;
            let mut docs = Vec::new();
            for (i, doc) in Deserializer::from_str(&yaml_str).enumerate() {
                if i >= max_docs {
                    break;
                }
                match YamlValue::deserialize(doc) {
                    Ok(val) => docs.push(val),
                    Err(_) => break,
                }
            }
            if docs.is_empty() {
                docs.push(serde_yaml::from_str::<YamlValue>(&yaml_str).map_err(|e| {
                    error!("Failed to parse properties YAML '{}': {}", yaml_path.display(), e);
                    format!("Failed to parse properties YAML '{}': {}", yaml_path.display(), e)
                })?);
            }
            let doc = docs
                .into_iter()
                .find(|val| val.get("properties").is_some())
                .ok_or_else(|| {
                    error!("No YAML doc with 'properties' at top level in {}", yaml_path.display());
                    format!("No YAML doc with 'properties' at top level in {}", yaml_path.display())
                })?;
            return Ok((doc, Some(yaml_path.to_string_lossy().to_string())));
        }
    }
    // Fallback: use the OpenAPI schema itself as the doc
    Ok((YamlValue::Mapping(schema_mapping.clone()), None))
}

/// Extracts parameters for a handler from a path object, resolving $ref if present.
fn extract_parameters_for_handler(swagger_doc: &YamlValue, path_obj: &Mapping) -> Vec<ParameterInfo> {
    let mut parameters: Vec<ParameterInfo> = Vec::new();
    if let Some(get_item) = path_obj
        .get(&YamlValue::String("get".to_string()))
        .and_then(YamlValue::as_mapping)
    {
        if let Some(seq) = get_item
            .get(&YamlValue::String("parameters".to_string()))
            .and_then(YamlValue::as_sequence)
        {
            for param in seq {
                // Handle $ref
                if let Some(ref_str) = param.get("$ref").and_then(YamlValue::as_str) {
                    if let Some(resolved) = resolve_parameter_ref(swagger_doc, ref_str) {
                        if let Some(param_info) = extract_parameter_info(resolved) {
                            parameters.push(param_info);
                        }
                    }
                } else if let Some(pm) = param.as_mapping() {
                    if let Some(param_info) = extract_parameter_info(&YamlValue::Mapping(pm.clone())) {
                        parameters.push(param_info);
                    }
                }
            }
        }
    }
    parameters
}

/// Generates the OpenAPI JSON file from endpoint data.
fn generate_openapi_json(
    endpoints: &Vec<String>,
    endpoint_data: &Vec<serde_json::Value>,
    openapi_json_path: &PathBuf,
    schemas: &serde_json::Map<String, serde_json::Value>,
) -> anyhow::Result<()> {
    use serde_json::{Map, Value};
    use std::fs::File;
    use std::io::Write;

    // Build OpenAPI doc structure
    let mut paths = Map::new();

    // Add shared metadata and totals schemas
    let mut openapi_schemas = schemas.clone();

    for ed in endpoint_data {
        let endpoint = ed.get("endpoint").and_then(Value::as_str).unwrap();
        let response_struct_name = ed
            .get("response_struct_name")
            .and_then(Value::as_str)
            .unwrap();
        let properties_struct_name = ed
            .get("properties_struct_name")
            .and_then(Value::as_str)
            .unwrap();
        let response_schema = ed.get("response_schema").unwrap();
        let properties_schema = ed.get("properties_schema").unwrap();

        // Add schemas
        openapi_schemas.insert(response_struct_name.to_string(), response_schema.clone());
        openapi_schemas.insert(properties_struct_name.to_string(), properties_schema.clone());

        // Find parameters for this endpoint in swagger_doc (flattened, idiomatic)
        let parameters_json = ed.get("parameters").unwrap();

        // Add endpoint path
        let path_entry = serde_json::json!({
            "get": {
                "operationId": format!("get_{}", endpoint),
                "summary": format!("MCP Server proxied request to the FDIC Bank Find /{} API Endpoint", endpoint),
                "parameters": parameters_json,
                "responses": {
                    "200": {
                        "description": format!("Successful {} response", endpoint),
                        "content": {
                            "application/json": {
                                "schema": {"$ref": format!("#/components/schemas/{}", response_struct_name)}
                            }
                        }
                    },
                    "400": {"description": "Bad input parameter"},
                    "500": {"description": "Internal Server Error"},
                    "502": {"description": "Bad Gateway"}
                }
            }
        });
        paths.insert(format!("/{}", endpoint), path_entry);
    }

    let openapi_doc = serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "FDIC BankFind MCP API",
            "version": "1.0.0"
        },
        "paths": paths,
        "components": {
            "schemas": openapi_schemas
        }
    });
    let mut file = File::create(openapi_json_path)?;
    file.write_all(serde_json::to_string_pretty(&openapi_doc)?.as_bytes())?;
    Ok(())
}

/// Maps OpenAPI types and formats to Rust types for code generation.
fn openapi_type_to_rust_type(openapi_type: &str, format: Option<&str>) -> &'static str {
    match (openapi_type, format) {
        ("string", _) => "String",
        ("integer", Some("int64")) => "i64",
        ("integer", _) => "i32",
        ("number", Some("double")) => "f64",
        ("number", _) => "f32",
        ("boolean", _) => "bool",
        _ => "String",
    }
}

/// Resolve a $ref string (e.g. "#/components/parameters/fileFormat") into a serde_yaml::Value
fn resolve_parameter_ref<'a>(swagger_doc: &'a YamlValue, ref_str: &str) -> Option<&'a YamlValue> {
    let mut node = swagger_doc;
    let path = ref_str.trim_start_matches("#/").split('/');
    for key in path {
        node = node.as_mapping().and_then(|m| m.get(key))?;
    }
    Some(node)
}

/// Sanitizes Markdown for Rust doc comments and Swagger UI.
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

/// Extracts the actual row/item properties for the Properties struct from the YAML doc.
/// Uses properties.data.properties if present, otherwise falls back to top-level properties.
fn extract_row_properties(selected_doc: &serde_yaml::Value) -> Vec<PropertyInfo> {
    selected_doc
        .get("properties")
        .and_then(|props| props.get("data"))
        .and_then(|data| data.get("properties"))
        .and_then(serde_yaml::Value::as_mapping)
        .map(|mapping| extract_properties(mapping))
        .unwrap_or_else(|| {
            selected_doc
                .get("properties")
                .and_then(serde_yaml::Value::as_mapping)
                .map(|mapping| extract_properties(mapping))
                .unwrap_or_default()
        })
}
