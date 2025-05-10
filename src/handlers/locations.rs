//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/locations` endpoint.

// Internal imports (std, crate)
use crate::common::*;
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::handler::server::tool::IntoCallToolResult;
use rmcp::model::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info};
use utoipa::ToSchema;

/// Auto-generated parameters struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct LocationsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for LocationsParameters {
    fn name() -> &'static str {
        "locations"
    }
}

// Implement QueryParameters for generic handler
impl QueryParameters for LocationsParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ADDRESS",
        "BKCLASS",
        "CBSA",
        "CBSA_DIV",
        "CBSA_DIV_FLG",
        "CBSA_DIV_NO",
        "CBSA_METRO",
        "CBSA_METRO_FLG",
        "CBSA_METRO_NAME",
        "CBSA_MICRO_FLG",
        "CBSA_NO",
        "CERT",
        "CITY",
        "COUNTY",
        "CSA",
        "CSA_FLG",
        "CSA_NO",
        "ESTYMD",
        "FI_UNINUM",
        "LATITUDE",
        "LONGITUDE",
        "MDI_STATUS_CODE",
        "MDI_STATUS_DESC",
        "MAINOFF",
        "NAME",
        "OFFNAME",
        "OFFNUM",
        "RUNDATE",
        "SERVTYPE",
        "SERVTYPE_DESC",
        "STALP",
        "STCNTY",
        "STNAME",
        "UNINUM",
        "ZIP",
    ];

    #[allow(unused_variables)] // the `query` parameter is unused if there are no endpoint-specific parameters
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct LocationsProperties {
    #[schemars(description = r#"Street Address - The street address in which an institution or branch office is physically located."#)]
    pub address: Option<String>,

    #[schemars(description = r#"Institution Class - A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#)]
    pub bkclass: Option<String>,

    #[schemars(description = r#"Core Based Statistical Area Name - Name of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Name - Name of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_div: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Flag - A flag (1=Yes) indicating member of a Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_div_flg: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Number - Numeric code of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_div_no: Option<String>,

    #[schemars(description = r#"Metropolitan Division Number - Numeric code of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#)]
    pub cbsa_metro: Option<String>,

    #[schemars(description = r#"Metropolitan Division Flag - A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#)]
    pub cbsa_metro_flg: Option<String>,

    #[schemars(description = r#"Metropolitan Division Name - Name of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#)]
    pub cbsa_metro_name: Option<String>,

    #[schemars(description = r#"Micropolitan Division Flag - A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#)]
    pub cbsa_micro_flg: Option<String>,

    #[schemars(description = r#"Core Based Statistical Areas - Numeric code of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_no: Option<String>,

    #[schemars(description = r#"FDIC Certificate # - A unique number assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#)]
    pub cert: Option<String>,

    #[schemars(description = r#"City - The city in which an institution or branch office is physically located."#)]
    pub city: Option<String>,

    #[schemars(description = r#"County - The county name in which an institution or branch office is physically located."#)]
    pub county: Option<String>,

    #[schemars(description = r#"Combined Statistical Area Name - Name of the Combined Statistical Area (CSA) as defined by the US Census Bureau."#)]
    pub csa: Option<String>,

    #[schemars(description = r#"Combined Statistical Area Flag  (Branch) - Flag (1=Yes) indicating member of a Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#)]
    pub csa_flg: Option<String>,

    #[schemars(description = r#"Combined Statistical Area Number  (Branch) - Numeric code of the Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#)]
    pub csa_no: Option<String>,

    #[schemars(description = r#"Branch Established Date - The date on which the branch began operations."#)]
    pub estymd: Option<String>,

    #[schemars(description = r#"FDIC's unique number - FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps the branch back to the parent financial institution."#)]
    pub fi_uninum: Option<String>,

    #[schemars(description = r#"Location Address Latitude - The latitude of the physical address."#)]
    pub latitude: Option<f32>,

    #[schemars(description = r#"Location Address Latitude - The longitude of the physical address."#)]
    pub longitude: Option<f32>,

    #[schemars(description = r#"Minority Status Code - A numeric flag used to indicate whether an institution is primarily a minority owned institution."#)]
    pub mdi_status_code: Option<String>,

    #[schemars(description = r#"Minority Status Description - A descriptive flag used to indicate type of minority owned institution."#)]
    pub mdi_status_desc: Option<String>,

    #[schemars(description = r#"Main Office - Flag (1=Yes) indicating this location is the main office for the institution."#)]
    pub mainoff: Option<f32>,

    #[schemars(description = r#"Institution Name (Search-Eligible) - The legal title or name of the institution. This field can be used for search and filtering."#)]
    pub name: Option<String>,

    #[schemars(description = r#"Office Name - Branch office name."#)]
    pub offname: Option<String>,

    #[schemars(description = r#"Branch Number - An institution's branch office number used internally by FDIC."#)]
    pub offnum: Option<String>,

    #[schemars(description = r#"Run Date - The day the institution information was updated."#)]
    pub rundate: Option<String>,

    #[schemars(description = r#"Service Type Code - Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch"#)]
    pub servtype: Option<f32>,

    #[schemars(description = r#"Service Type Code Description (Search-Eligible) - Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch This field can be used for search and filtering."#)]
    pub servtype_desc: Option<String>,

    #[schemars(description = r#"State Alpha Code - The state abbreviation of the location of the institution's main office."#)]
    pub stalp: Option<String>,

    #[schemars(description = r#"State and County Number - A five digit number representing the state and county in which the institution is physically located.  The first two digits represent the FIPS state numeric code and the last three digits represent the FIPS county numeric code."#)]
    pub stcnty: Option<String>,

    #[schemars(description = r#"Branch State - State in which the  branch is physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#)]
    pub stname: Option<String>,

    #[schemars(description = r#"FDIC's Unique Number - FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#)]
    pub uninum: Option<String>,

    #[schemars(description = r#"Zip Code - The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#)]
    pub zip: Option<String>,

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct LocationsResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
    pub totals: ResponseTotals,
}

impl IntoContents for LocationsResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize LocationsResponse to Content")]
    }
}

/// FDIC BankFind API `/locations` endpoint handler
/// Get Institution Locations
/// Returns locations/branches of financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter for the location search. All values must be entered in UPPERCASE. - `fields` (String, optional): Comma delimited list of fields to return. All values must be entered in UPPERCASE.
NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
NAME - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /locations
Parameters: LocationsParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Structure"#]
pub async fn locations_handler(config: &FdicApiConfig, params: &LocationsParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "locations",
        method = "GET",
        path = "/locations",
        params = serde_json::to_string(params).unwrap()
    );
    debug!(target = "handler", event = "before_fdic_api_call", endpoint = "locations");
    let resp = get_fdic_bank_find_mcp_response::<_, LocationsResponse>(config, params).await;

    match &resp {
        Ok(r) => {
            info!(
                target = "handler",
                event = "fdic_api_response",
                endpoint = "locations",
                meta = ?r.meta,
                totals = ?r.totals
            );
        },
        Err(e) => {
            error!(target = "handler", event = "fdic_api_error", endpoint = "locations", error = ?e);
        }
    }

    // Log outgoing FDIC API request as structured JSON
    resp.and_then(|r| r.into_call_tool_result())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = LocationsParameters {
            common: CommonParameters::default(),
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }

    #[test]
    fn test_properties_struct_serialization() {
        let props = LocationsProperties {
            address: None,
            bkclass: None,
            cbsa: None,
            cbsa_div: None,
            cbsa_div_flg: None,
            cbsa_div_no: None,
            cbsa_metro: None,
            cbsa_metro_flg: None,
            cbsa_metro_name: None,
            cbsa_micro_flg: None,
            cbsa_no: None,
            cert: None,
            city: None,
            county: None,
            csa: None,
            csa_flg: None,
            csa_no: None,
            estymd: None,
            fi_uninum: None,
            latitude: None,
            longitude: None,
            mdi_status_code: None,
            mdi_status_desc: None,
            mainoff: None,
            name: None,
            offname: None,
            offnum: None,
            rundate: None,
            servtype: None,
            servtype_desc: None,
            stalp: None,
            stcnty: None,
            stname: None,
            uninum: None,
            zip: None,
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
