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

use crate::config::FdicApiConfig;
use rmcp::{ServerHandler, model::*, tool};

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
        eprintln!("[FDIC MCP] get_info() called - should show tools!");

        // Set up explicit capabilities for tools and resources
        let mut tools_capability = ToolsCapability::default();
        tools_capability.list_changed = Some(true);
        
        let info = ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities {
                experimental: None,
                logging: None,
                completions: None,
                prompts: None,
                resources: None,
                tools: Some(tools_capability),
            },
            server_info: Implementation::from_build_env(),
            instructions: Some("FDIC Bank Find MCP Server (Unofficial) - https://banks.data.fdic.gov/docs/".into()),
        };

        eprintln!("[FDIC MCP] Returning ServerInfo with enabled tools and resources: {:?}", info);
        info
    }
}
