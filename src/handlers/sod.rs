//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/sod` endpoint.

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

/// Auto-generated parameters struct for `/sod` endpoint.
/// Spec: sod_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct SodParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[schemars(description = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_by: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#)]
    pub agg_term_fields: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_sum_fields: Option<String>,
    #[schemars(description = r#"The limit on how many aggregated results will be displayed"#)]
    pub agg_limit: Option<i32>,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for SodParameters {
    fn name() -> &'static str {
        "sod"
    }
}

// Implement QueryParameters for generic handler
impl QueryParameters for SodParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ADDRESBR",
        "ADDRESS",
        "ASSET",
        "BKCLASS",
        "BKMO",
        "BRCENM",
        "BRNUM",
        "BRSERTYP",
        "CALL",
        "CB",
        "CBSA_DIV_NAMB",
        "CERT",
        "CHARTER",
        "CHRTAGNN",
        "CHRTAGNT",
        "CITY",
        "CITY2BR",
        "CITYBR",
        "CITYHCR",
        "CLCODE",
        "CNTRYNA",
        "CNTRYNAB",
        "CNTYNAMB",
        "CNTYNUMB",
        "CONSOLD",
        "CSABR",
        "CSANAMBR",
        "DENOVO",
        "DEPDOM",
        "DEPSUM",
        "DEPSUMBR",
        "DIVISIONB",
        "DOCKET",
        "ESCROW",
        "FDICDBS",
        "FDICNAME",
        "FED",
        "FEDNAME",
        "HCTMULT",
        "INSAGNT1",
        "INSBRDD",
        "INSBRTS",
        "INSURED",
        "METROBR",
        "MICROBR",
        "MSABR",
        "MSANAMB",
        "NAMEBR",
        "NAMEFULL",
        "NAMEHCR",
        "NECNAMB",
        "NECTABR",
        "OCCDIST",
        "OCCNAME",
        "PLACENUM",
        "REGAGNT",
        "RSSDHCR",
        "RSSDID",
        "SIMS_ACQUIRED_DATE",
        "SIMS_DESCRIPTION",
        "SIMS_ESTABLISHED_DATE",
        "SIMS_LATITUDE",
        "SIMS_LONGITUDE",
        "SIMS_PROJECTION",
        "SPECDESC",
        "SPECGRP",
        "STALP",
        "STALPBR",
        "STALPHCR",
        "STCNTY",
        "STCNTYBR",
        "STNAME",
        "STNAMEBR",
        "STNUMBR",
        "UNINUMBR",
        "UNIT",
        "USA",
        "YEAR",
        "ZIP_RAW",
        "ZIPBR_RAW",
        "ZIP",
        "ZIPBR",
    ];

    #[allow(unused_variables)] // the `query` parameter is unused if there are no endpoint-specific parameters
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        
        if let Some(val) = &self.agg_by {
            query.insert("agg_by".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_term_fields {
            query.insert("agg_term_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_sum_fields {
            query.insert("agg_sum_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_limit {
            query.insert("agg_limit".to_string(), val.to_string());
        }
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/sod` endpoint.
/// Spec: sod_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct SodProperties {
    #[schemars(description = r#"ADDRESS (BRANCH) - ADDRESS (BRANCH)"#)]
    pub addresbr: Option<String>,

    #[schemars(description = r#"ADDRESS (Search-Eligible) - ADDRESS This field can be used for search and filtering."#)]
    pub address: Option<String>,

    #[schemars(description = r#"TOTAL ASSETS - TOTAL ASSETS"#)]
    pub asset: Option<f32>,

    #[schemars(description = r#"INSTITUTION CLASS (Search-Eligible) - INSTITUTION CLASS This field can be used for search and filtering."#)]
    pub bkclass: Option<String>,

    #[schemars(description = r#"MAIN OFFICE DESIGNATION FLAG - MAIN OFFICE DESIGNATION FLAG"#)]
    pub bkmo: Option<f32>,

    #[schemars(description = r#"CENCODES C - CENCODES C"#)]
    pub brcenm: Option<String>,

    #[schemars(description = r#"BRANCH NUMBER - BRANCH NUMBER"#)]
    pub brnum: Option<f32>,

    #[schemars(description = r#"BRANCH SERVICE TYPE - BRANCH SERVICE TYPE"#)]
    pub brsertyp: Option<f32>,

    #[schemars(description = r#"REPORT TYPE - REPORT TYPE"#)]
    pub call: Option<String>,

    #[schemars(description = r#"COMMUNITY BANK FLAG - COMMUNITY BANK FLAG"#)]
    pub cb: Option<f32>,

    #[schemars(description = r#"CBSA DIVISION NAME (BRANCH) - CBSA DIVISION NAME (BRANCH)"#)]
    pub cbsa_div_namb: Option<String>,

    #[schemars(description = r#"FDIC CERT NUMBER - FDIC CERT NUMBER"#)]
    pub cert: Option<f32>,

    #[schemars(description = r#"CHARTER (Search-Eligible) - CHARTER This field can be used for search and filtering."#)]
    pub charter: Option<String>,

    #[schemars(description = r#"CHARTER AGENT NAME - CHARTER AGENT NAME"#)]
    pub chrtagnn: Option<String>,

    #[schemars(description = r#"CHARTER AGENT CODE - CHARTER AGENT CODE"#)]
    pub chrtagnt: Option<String>,

    #[schemars(description = r#"CITY (MAIN OFFICE) (Search-Eligible) - CITY (MAIN OFFICE) This field can be used for search and filtering."#)]
    pub city: Option<String>,

    #[schemars(description = r#"PREFERRED CITY (BRANCH) (Search-Eligible) - PREFERRED CITY (BRANCH) This field can be used for search and filtering."#)]
    pub city2br: Option<String>,

    #[schemars(description = r#"CITY (BRANCH) (Search-Eligible) - CITY (BRANCH) This field can be used for search and filtering."#)]
    pub citybr: Option<String>,

    #[schemars(description = r#"CITY-HOLDING CO.- REGULATORY (Search-Eligible) - CITY-HOLDING CO.- REGULATORY This field can be used for search and filtering."#)]
    pub cityhcr: Option<String>,

    #[schemars(description = r#"CLASS NUMBER - CLASS NUMBER"#)]
    pub clcode: Option<f32>,

    #[schemars(description = r#"COUNTRY NAME (MAIN OFFICE) - COUNTRY NAME (MAIN OFFICE)"#)]
    pub cntryna: Option<String>,

    #[schemars(description = r#"COUNTRY NAME (BRANCH) - COUNTRY NAME (BRANCH)"#)]
    pub cntrynab: Option<String>,

    #[schemars(description = r#"COUNTY NAME (BRANCH) (Search-Eligible) - COUNTY NAME (BRANCH) This field can be used for search and filtering."#)]
    pub cntynamb: Option<String>,

    #[schemars(description = r#"FIPS COUNTY CODE (BRANCH) - FIPS COUNTY CODE (BRANCH)"#)]
    pub cntynumb: Option<f32>,

    #[schemars(description = r#"CONSOLIDATED BRANCH NUMBER - CONSOLIDATED BRANCH NUMBER"#)]
    pub consold: Option<f32>,

    #[schemars(description = r#"CSA NUMBER (BRANCH) - CSA NUMBER (BRANCH)"#)]
    pub csabr: Option<f32>,

    #[schemars(description = r#"CSA NAME (BRANCH) - CSA NAME (BRANCH)"#)]
    pub csanambr: Option<String>,

    #[schemars(description = r#"DENOVO FLAG - DENOVO FLAG"#)]
    pub denovo: Option<f32>,

    #[schemars(description = r#"TOTAL DOMESTIC DEPOSITS - TOTAL DOMESTIC DEPOSITS"#)]
    pub depdom: Option<f32>,

    #[schemars(description = r#"TOTAL DEPOSITS - TOTAL DEPOSITS"#)]
    pub depsum: Option<f32>,

    #[schemars(description = r#"DOMESTIC DEPOSITS (SOD) - DOMESTIC DEPOSITS (SOD)"#)]
    pub depsumbr: Option<f32>,

    #[schemars(description = r#"CBSA DIVISION CODE (BRANCH) - CBSA DIVISION CODE (BRANCH)"#)]
    pub divisionb: Option<f32>,

    #[schemars(description = r#"OTS DOCKET NUMBER - OTS DOCKET NUMBER"#)]
    pub docket: Option<f32>,

    #[schemars(description = r#"ESCROW ACCOUNTS - TFR - ESCROW ACCOUNTS - TFR"#)]
    pub escrow: Option<f32>,

    #[schemars(description = r#"FDIC REGION NUMBER - FDIC REGION NUMBER"#)]
    pub fdicdbs: Option<f32>,

    #[schemars(description = r#"FDIC REGION NAME - FDIC REGION NAME"#)]
    pub fdicname: Option<String>,

    #[schemars(description = r#"FRB DISTRICT NUMBER - FRB DISTRICT NUMBER"#)]
    pub fed: Option<f32>,

    #[schemars(description = r#"FED DISTRICT NAME - FED DISTRICT NAME"#)]
    pub fedname: Option<String>,

    #[schemars(description = r#"MULTI-BANK HOLDING CO - MULTI-BANK HOLDING CO"#)]
    pub hctmult: Option<String>,

    #[schemars(description = r#"PRIMARY INSURANCE FUND - PRIMARY INSURANCE FUND"#)]
    pub insagnt1: Option<String>,

    #[schemars(description = r#"DEMAND DEPOSITS IN INSURED BRANCHES - DEMAND DEPOSITS IN INSURED BRANCHES"#)]
    pub insbrdd: Option<f32>,

    #[schemars(description = r#"TIME & SAVINGS DEPOSITS IN INSURED BRANCHES - TIME & SAVINGS DEPOSITS IN INSURED BRANCHES"#)]
    pub insbrts: Option<f32>,

    #[schemars(description = r#"INSURED (Search-Eligible) - INSURED This field can be used for search and filtering."#)]
    pub insured: Option<String>,

    #[schemars(description = r#"METRO FLAG (BRANCH) - METRO FLAG (BRANCH)"#)]
    pub metrobr: Option<f32>,

    #[schemars(description = r#"MICRO FLAG (BRANCH) - MICRO FLAG (BRANCH)"#)]
    pub microbr: Option<f32>,

    #[schemars(description = r#"MSA (BRANCH) - MSA (BRANCH)"#)]
    pub msabr: Option<f32>,

    #[schemars(description = r#"MSA NAME (BRANCH) (Search-Eligible) - MSA NAME (BRANCH) This field can be used for search and filtering."#)]
    pub msanamb: Option<String>,

    #[schemars(description = r#"INSTITUTION NAME (BRANCH) (Search-Eligible) - INSTITUTION NAME (BRANCH) This field can be used for search and filtering."#)]
    pub namebr: Option<String>,

    #[schemars(description = r#"INSTITUTION NAME (MAIN OFFICE) (Search-Eligible) - INSTITUTION NAME (MAIN OFFICE) This field can be used for search and filtering."#)]
    pub namefull: Option<String>,

    #[schemars(description = r#"BANK HOLDING CO. NAME (Search-Eligible) - BANK HOLDING CO. NAME This field can be used for search and filtering."#)]
    pub namehcr: Option<String>,

    #[schemars(description = r#"NECTA NAME (BRANCH) - NECTA NAME (BRANCH)"#)]
    pub necnamb: Option<String>,

    #[schemars(description = r#"NECTA (BRANCH) - NECTA (BRANCH)"#)]
    pub nectabr: Option<String>,

    #[schemars(description = r#"OCC DISTRICT NUMBER - OCC DISTRICT NUMBER"#)]
    pub occdist: Option<f32>,

    #[schemars(description = r#"OCC REGION NAME - OCC REGION NAME"#)]
    pub occname: Option<String>,

    #[schemars(description = r#"PLACE CODE NUMBER (DF) - PLACE CODE NUMBER (DF)"#)]
    pub placenum: Option<f32>,

    #[schemars(description = r#"PRIMARY FEDERAL REGULATOR - PRIMARY FEDERAL REGULATOR"#)]
    pub regagnt: Option<String>,

    #[schemars(description = r#"FRB ID NUMBER - BHC - FRB ID NUMBER - BHC"#)]
    pub rssdhcr: Option<f32>,

    #[schemars(description = r#"FRB ID NUMBER - FRB ID NUMBER"#)]
    pub rssdid: Option<f32>,

    #[schemars(description = r#"SIMS ACQUIRED DATE - SIMS ACQUIRED DATE"#)]
    pub sims_acquired_date: Option<String>,

    #[schemars(description = r#"SIMS MATCH CODE (DESCRIPTION) - SIMS MATCH CODE (DESCRIPTION)"#)]
    pub sims_description: Option<String>,

    #[schemars(description = r#"SIMS ESTABLISHED DATE - SIMS ESTABLISHED DATE"#)]
    pub sims_established_date: Option<String>,

    #[schemars(description = r#"SIMS GEOGRAPHIC LATITUDE - SIMS GEOGRAPHIC LATITUDE"#)]
    pub sims_latitude: Option<f32>,

    #[schemars(description = r#"SIMS GEOGRAPHIC LONGITUDE - SIMS GEOGRAPHIC LONGITUDE"#)]
    pub sims_longitude: Option<f32>,

    #[schemars(description = r#"SIMS SCORE (PROJECTION) - SIMS SCORE (PROJECTION)"#)]
    pub sims_projection: Option<String>,

    #[schemars(description = r#"SPECGRP DESCRIPTION - SPECGRP DESCRIPTION"#)]
    pub specdesc: Option<String>,

    #[schemars(description = r#"INDUSTRY SPECIALIZATION GROUP - INDUSTRY SPECIALIZATION GROUP"#)]
    pub specgrp: Option<f32>,

    #[schemars(description = r#"FIPS STATE ALPHA CODE (MAIN OFFICE) (Search-Eligible) - FIPS STATE ALPHA CODE (MAIN OFFICE) This field can be used for search and filtering."#)]
    pub stalp: Option<String>,

    #[schemars(description = r#"FIPS STATE ALPHA CODE (BRANCH) (Search-Eligible) - FIPS STATE ALPHA CODE (BRANCH) This field can be used for search and filtering."#)]
    pub stalpbr: Option<String>,

    #[schemars(description = r#"FIPS STATE ALPHA CODE - BHC (Search-Eligible) - FIPS STATE ALPHA CODE - BHC This field can be used for search and filtering."#)]
    pub stalphcr: Option<String>,

    #[schemars(description = r#"FIPS STATE & COUNTY NO. (MAIN OFFICE) - FIPS STATE & COUNTY NO. (MAIN OFFICE)"#)]
    pub stcnty: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub stcntybr: Option<f32>,

    #[schemars(description = r#"FIPS STATE NAME (MAIN OFFICE) (Search-Eligible) - FIPS STATE NAME (MAIN OFFICE) This field can be used for search and filtering."#)]
    pub stname: Option<String>,

    #[schemars(description = r#"STATE NAME (BRANCH) (Search-Eligible) - STATE NAME (BRANCH) This field can be used for search and filtering."#)]
    pub stnamebr: Option<String>,

    #[schemars(description = r#"FIPS STATE CODE (BRANCH) - FIPS STATE CODE (BRANCH)"#)]
    pub stnumbr: Option<f32>,

    #[schemars(description = r#"UNINUM (BRANCH) - UNINUM (BRANCH)"#)]
    pub uninumbr: Option<f32>,

    #[schemars(description = r#"UNIT BANK FLAG - UNIT BANK FLAG"#)]
    pub unit: Option<f32>,

    #[schemars(description = r#"USA LOCATED INSTITUTION - USA LOCATED INSTITUTION"#)]
    pub usa: Option<f32>,

    #[schemars(description = r#"YEAR - YEAR"#)]
    pub year: Option<f32>,

    #[schemars(description = r#" - "#)]
    pub zip_raw: Option<String>,

    #[schemars(description = r#" - "#)]
    pub zipbr_raw: Option<String>,

    #[schemars(description = r#"Zip Code (Search-Eligible) - The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices. This field can be used for search and filtering."#)]
    pub zip: Option<String>,

    #[schemars(description = r#"Zip Code (Search-Eligible) - The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices. This field can be used for search and filtering."#)]
    pub zipbr: Option<String>,

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct SodResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
    pub totals: ResponseTotals,
}

impl IntoContents for SodResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize SodResponse to Content")]
    }
}

/// FDIC BankFind API `/sod` endpoint handler
/// Get Summary of Deposits Information for FDIC Insured Institutions
/// Returns summary of deposits information for institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
Examples:
* Filter data by the numeric range
`ASSET:&#91;1000 TO 9999&#93;`
CERT:14 - `fields` (String, optional): Comma delimited list of fields with quarterly financial data to return. All values must be entered in UPPERCASE.
CERT,YEAR,ASSET,DEPSUMBR,STALPBR - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
YEAR - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
CERT - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
YEAR - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
ASSET - `agg_limit` (i32, optional): The limit on how many aggregated results will be displayed - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /sod
Parameters: SodParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Summary of Deposits"#]
pub async fn sod_handler(config: &FdicApiConfig, params: &SodParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "sod",
        method = "GET",
        path = "/sod",
        params = serde_json::to_string(params).unwrap()
    );
    debug!(target = "handler", event = "before_fdic_api_call", endpoint = "sod");
    let resp = get_fdic_bank_find_mcp_response::<_, SodResponse>(config, params).await;

    match &resp {
        Ok(r) => {
            info!(
                target = "handler",
                event = "fdic_api_response",
                endpoint = "sod",
                meta = ?r.meta,
                totals = ?r.totals
            );
        },
        Err(e) => {
            error!(target = "handler", event = "fdic_api_error", endpoint = "sod", error = ?e);
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
        let params = SodParameters {
            common: CommonParameters::default(),
            agg_by: None,
            agg_term_fields: None,
            agg_sum_fields: None,
            agg_limit: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }

    #[test]
    fn test_properties_struct_serialization() {
        let props = SodProperties {
            addresbr: None,
            address: None,
            asset: None,
            bkclass: None,
            bkmo: None,
            brcenm: None,
            brnum: None,
            brsertyp: None,
            call: None,
            cb: None,
            cbsa_div_namb: None,
            cert: None,
            charter: None,
            chrtagnn: None,
            chrtagnt: None,
            city: None,
            city2br: None,
            citybr: None,
            cityhcr: None,
            clcode: None,
            cntryna: None,
            cntrynab: None,
            cntynamb: None,
            cntynumb: None,
            consold: None,
            csabr: None,
            csanambr: None,
            denovo: None,
            depdom: None,
            depsum: None,
            depsumbr: None,
            divisionb: None,
            docket: None,
            escrow: None,
            fdicdbs: None,
            fdicname: None,
            fed: None,
            fedname: None,
            hctmult: None,
            insagnt1: None,
            insbrdd: None,
            insbrts: None,
            insured: None,
            metrobr: None,
            microbr: None,
            msabr: None,
            msanamb: None,
            namebr: None,
            namefull: None,
            namehcr: None,
            necnamb: None,
            nectabr: None,
            occdist: None,
            occname: None,
            placenum: None,
            regagnt: None,
            rssdhcr: None,
            rssdid: None,
            sims_acquired_date: None,
            sims_description: None,
            sims_established_date: None,
            sims_latitude: None,
            sims_longitude: None,
            sims_projection: None,
            specdesc: None,
            specgrp: None,
            stalp: None,
            stalpbr: None,
            stalphcr: None,
            stcnty: None,
            stcntybr: None,
            stname: None,
            stnamebr: None,
            stnumbr: None,
            uninumbr: None,
            unit: None,
            usa: None,
            year: None,
            zip_raw: None,
            zipbr_raw: None,
            zip: None,
            zipbr: None,
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
