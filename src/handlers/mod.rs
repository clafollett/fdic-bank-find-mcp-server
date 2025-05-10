//! Do not edit by hand.
//! Auto-generated handler stubs for FDIC BankFind API endpoints.
// MCP auto-generated: Endpoint handler modules
pub mod demographics;
pub mod failures;
pub mod history;
pub mod institutions;
pub mod locations;
pub mod sod;
pub mod summary;
// Skipped endpoints due to ENDPOINT_EXCLUSIONS:
// [EXCLUDED] pub mod financials;

// Internal dependencies
use crate::config::FdicApiConfig;

// External dependencies
use log::debug;
use rmcp::{ServerHandler, Error, model::*, service::*, tool};

pub const FDIC_BASE_URL: &str = "https://banks.data.fdic.gov/api";

#[derive(Clone, Debug, Default)]
pub struct FdicBankFindMcpServer;

impl FdicBankFindMcpServer {
    pub fn new() -> Self {
        FdicBankFindMcpServer::default()
    }
}

#[tool(tool_box)]
impl FdicBankFindMcpServer {
    /// Returns MCP server status for Inspector/health validation
    #[tool(description = "Returns MCP server status for Inspector/health validation")]
    pub async fn ping(&self) -> String {
        "The FDIC Bank Find MCP server is alive!".to_string()
    }
    /// FDIC BankFind API `/demographics` endpoint handler
    #[tool(description = r#"Get Summary of Demographic Information - Returns summary of demographic information - Demographics"#)]
    pub async fn get_demographics(&self, #[tool(aggr)] params: demographics::DemographicsParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = demographics::demographics_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/failures` endpoint handler
    #[tool(description = r#"Get detail on historical bank failures from 1934 to present. - Returns details on failed financial institutions. - Failures"#)]
    pub async fn get_failures(&self, #[tool(aggr)] params: failures::FailuresParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = failures::failures_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/history` endpoint handler
    #[tool(description = r#"Get Detail on Structure Change Events - Returns details on structure change events - History"#)]
    pub async fn get_history(&self, #[tool(aggr)] params: history::HistoryParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = history::history_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/institutions` endpoint handler
    #[tool(description = r#"Get Financial Institutions - Returns a list of financial institutions. - Structure"#)]
    pub async fn get_institutions(&self, #[tool(aggr)] params: institutions::InstitutionsParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = institutions::institutions_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/locations` endpoint handler
    #[tool(description = r#"Get Institution Locations - Returns locations/branches of financial institutions. - Structure"#)]
    pub async fn get_locations(&self, #[tool(aggr)] params: locations::LocationsParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = locations::locations_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/sod` endpoint handler
    #[tool(description = r#"Get Summary of Deposits Information for FDIC Insured Institutions - Returns summary of deposits information for institutions - Summary of Deposits"#)]
    pub async fn get_sod(&self, #[tool(aggr)] params: sod::SodParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = sod::sod_handler(&config, &params).await;
        fdic_response
    }
    /// FDIC BankFind API `/summary` endpoint handler
    #[tool(description = r#"Get Historical Aggregate Data by Year - Returns aggregate financial and structure data, subtotaled by year, regarding finanical institutions. - Historical"#)]
    pub async fn get_summary(&self, #[tool(aggr)] params: summary::SummaryParameters) -> Result<CallToolResult, rmcp::Error> {
        let config = FdicApiConfig { base_url: FDIC_BASE_URL.to_string() };
        let fdic_response = summary::summary_handler(&config, &params).await;
        fdic_response
    }
    // Skipped handler wrappers due to ENDPOINT_EXCLUSIONS:
    // [EXCLUDED] pub async fn financials_handler(...);
}

#[tool(tool_box)]
impl ServerHandler for FdicBankFindMcpServer {
    fn get_info(&self) -> ServerInfo {
        debug!("[FDIC MCP] get_info() called - should show tools!");

        // Set up explicit capabilities for tools and resources
        let mut tools_capability = ToolsCapability::default();
        tools_capability.list_changed = Some(true);

        let mut resources_capability = ResourcesCapability::default();
        resources_capability.list_changed = Some(true);

        let info = ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities {
                experimental: None,
                logging: None,
                completions: None,
                prompts: None,
                resources: Some(resources_capability),
                tools: Some(tools_capability),
            },
            server_info: Implementation::from_build_env(),
            instructions: Some(r#"{
  "fdic_mcp_agent_rules": {
    "critical": "Never assume field, filter, or sort compatibility across endpoints. Always check the endpoint schema.",
    "field_usage": "Use only fields present in the current endpoint schema. Confirm field is allowed for filter, sort, or output.",
    "field_naming_warning": "The field 'STATE' is valid ONLY in certain endpoints (e.g., 'history', 'sod'). It is NOT present in 'institutions' or 'locations'—use 'STALP' or 'STNAME' for those. Always anchor field usage to the specific endpoint schema. Never cross fields between endpoints.",
    "schema_discovery": "Valid fields for each endpoint can always be discovered programmatically via the MCP resources capability. Use the /schema endpoints (see list_resources) to fetch the full schema for any endpoint—never guess or hardcode field lists.",
    "query_validation_checklist": [
      "Field exists in endpoint schema",
      "Field is allowed for intended operation (filter/sort/output)",
      "Uppercase only field names, not values, except where endpoint docs specify otherwise.",
      "Never use a field from another endpoint"
    ],
    "error_handling": "On query error, re-validate schema and adjust. Never retry unchanged queries.",
    "pitfall": "Most common error: using a field from another endpoint or for an unsupported operation.",
    "fdic_filter_syntax": "FDIC 'filters' use an elasticsearch-like query language. For multi-value OR and AND conditions, use parentheses to group conditions, e.g., (STALP:IL OR STALP:WI). Comma-separated lists (e.g., STALP:IL,WI) do NOT work as an OR. Use AND to combine multiple filters. Avoid using OR/AND or parentheses outside of documented patterns. See https://banks.data.fdic.gov/docs for full syntax.",
    "fdic_error_handling": "Full FDIC error payloads (title, detail, links, meta, etc.) are returned in error responses for agent debugging and correction. Always inspect the 'fdic_raw' field for actionable details.",
    "fdic_filter_case_sensitivity": "FDIC BankFind API filter values are CASE SENSITIVE. You must match the exact case as stored in FDIC records (e.g., CITY:Chicago, not CITY:CHICAGO or CITY:chicago).",
    "fdic_filter_quoting": "Multi-word filter values MUST be wrapped in double quotes (e.g., STNAME:\"West Virginia\"). Single-word values do NOT require quotes but are still case sensitive (e.g., CITY:Chicago).",
    "fdic_filter_examples": "Examples: CITY:Chicago (matches), CITY:chicago (no match), CITY:CHICAGO (no match), STNAME:\"West Virginia\" (matches), STNAME:West Virginia (syntax error or no match)."
  }
}"#.into()),
        };

        debug!("[FDIC MCP] Returning ServerInfo with enabled tools and resources: {:?}", info);
        info
    }

    /// Implements MCP resource enumeration for all schema resources (one per endpoint)
    fn list_resources(
        &self, _request: Option<PaginatedRequestParam>, _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, Error>> + Send + '_ {
        use rmcp::model::{Annotated, RawResource};
        let resources = vec![
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "demographics"),
                    name: "demographics".to_string(),
                    description: Some("JSON schema for the /demographics endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "failures"),
                    name: "failures".to_string(),
                    description: Some("JSON schema for the /failures endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "history"),
                    name: "history".to_string(),
                    description: Some("JSON schema for the /history endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "institutions"),
                    name: "institutions".to_string(),
                    description: Some("JSON schema for the /institutions endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "locations"),
                    name: "locations".to_string(),
                    description: Some("JSON schema for the /locations endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "sod"),
                    name: "sod".to_string(),
                    description: Some("JSON schema for the /sod endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
            Annotated {
                raw: RawResource {
                    uri: format!("/schema/{}", "summary"),
                    name: "summary".to_string(),
                    description: Some("JSON schema for the /summary endpoint (fields, types, docs, envelope)".to_string()),
                    mime_type: Some("application/json".to_string()),
                    size: None,
                },
                annotations: Default::default(),
            },
        ];
        std::future::ready(Ok(ListResourcesResult { resources, next_cursor: None }))
    }

    /// Implements MCP resource fetching for schema resources by URI
    fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, Error>> + Send + '_ {
        use rmcp::model::{ResourceContents, ErrorData, ReadResourceResult};
        let uri = request.uri;
        let prefix = "/schema/";
        let result = if let Some(endpoint) = uri.strip_prefix(prefix) {
            let schema_json = match endpoint.to_lowercase().as_str() {
                "demographics" => include_str!("../../public/schemas/demographics.json"),
                "failures" => include_str!("../../public/schemas/failures.json"),
                "history" => include_str!("../../public/schemas/history.json"),
                "institutions" => include_str!("../../public/schemas/institutions.json"),
                "locations" => include_str!("../../public/schemas/locations.json"),
                "sod" => include_str!("../../public/schemas/sod.json"),
                "summary" => include_str!("../../public/schemas/summary.json"),
                _ => return std::future::ready(Err(Error::from(ErrorData::resource_not_found(
                    format!("Schema not found for endpoint '{}': unknown endpoint", endpoint),
                    None,
                )))),
            };
            let resource = ResourceContents::text(schema_json, format!("/schema/{}", endpoint.to_lowercase()));
            Ok(ReadResourceResult {
                contents: vec![resource],
            })
        } else {
            Err(Error::from(ErrorData::resource_not_found(
                format!("Unknown resource URI: {}", uri),
                None,
            )))
        };
        std::future::ready(result)
    }
}
