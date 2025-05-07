//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/sod` endpoint.

// Internal imports (std, crate)
use crate::common::{CommonParameters, FdicEndpoint, QueryParameters, get_fdic_bank_find_mcp_response};
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::model::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Auto-generated parameters struct for `/sod` endpoint.
/// Spec: sod_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
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

    #[allow(unused_variables)]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct SodProperties {
    #[doc = r#"Title: ADDRESS (BRANCH)"#]
    #[doc = r#"Description: ADDRESS (BRANCH)"#]
    pub addresbr: Option<String>,

    #[doc = r#"Title: ADDRESS (Search-Eligible)"#]
    #[doc = r#"Description: ADDRESS This field can be used for search and filtering."#]
    pub address: Option<String>,

    #[doc = r#"Title: TOTAL ASSETS"#]
    #[doc = r#"Description: TOTAL ASSETS"#]
    pub asset: Option<f32>,

    #[doc = r#"Title: INSTITUTION CLASS (Search-Eligible)"#]
    #[doc = r#"Description: INSTITUTION CLASS This field can be used for search and filtering."#]
    pub bkclass: Option<String>,

    #[doc = r#"Title: MAIN OFFICE DESIGNATION FLAG"#]
    #[doc = r#"Description: MAIN OFFICE DESIGNATION FLAG"#]
    pub bkmo: Option<f32>,

    #[doc = r#"Title: CENCODES C"#]
    #[doc = r#"Description: CENCODES C"#]
    pub brcenm: Option<String>,

    #[doc = r#"Title: BRANCH NUMBER"#]
    #[doc = r#"Description: BRANCH NUMBER"#]
    pub brnum: Option<f32>,

    #[doc = r#"Title: BRANCH SERVICE TYPE"#]
    #[doc = r#"Description: BRANCH SERVICE TYPE"#]
    pub brsertyp: Option<f32>,

    #[doc = r#"Title: REPORT TYPE"#]
    #[doc = r#"Description: REPORT TYPE"#]
    pub call: Option<String>,

    #[doc = r#"Title: COMMUNITY BANK FLAG"#]
    #[doc = r#"Description: COMMUNITY BANK FLAG"#]
    pub cb: Option<f32>,

    #[doc = r#"Title: CBSA DIVISION NAME (BRANCH)"#]
    #[doc = r#"Description: CBSA DIVISION NAME (BRANCH)"#]
    pub cbsa_div_namb: Option<String>,

    #[doc = r#"Title: FDIC CERT NUMBER"#]
    #[doc = r#"Description: FDIC CERT NUMBER"#]
    pub cert: Option<f32>,

    #[doc = r#"Title: CHARTER (Search-Eligible)"#]
    #[doc = r#"Description: CHARTER This field can be used for search and filtering."#]
    pub charter: Option<String>,

    #[doc = r#"Title: CHARTER AGENT NAME"#]
    #[doc = r#"Description: CHARTER AGENT NAME"#]
    pub chrtagnn: Option<String>,

    #[doc = r#"Title: CHARTER AGENT CODE"#]
    #[doc = r#"Description: CHARTER AGENT CODE"#]
    pub chrtagnt: Option<String>,

    #[doc = r#"Title: CITY (MAIN OFFICE) (Search-Eligible)"#]
    #[doc = r#"Description: CITY (MAIN OFFICE) This field can be used for search and filtering."#]
    pub city: Option<String>,

    #[doc = r#"Title: PREFERRED CITY (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: PREFERRED CITY (BRANCH) This field can be used for search and filtering."#]
    pub city2br: Option<String>,

    #[doc = r#"Title: CITY (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: CITY (BRANCH) This field can be used for search and filtering."#]
    pub citybr: Option<String>,

    #[doc = r#"Title: CITY-HOLDING CO.- REGULATORY (Search-Eligible)"#]
    #[doc = r#"Description: CITY-HOLDING CO.- REGULATORY This field can be used for search and filtering."#]
    pub cityhcr: Option<String>,

    #[doc = r#"Title: CLASS NUMBER"#]
    #[doc = r#"Description: CLASS NUMBER"#]
    pub clcode: Option<f32>,

    #[doc = r#"Title: COUNTRY NAME (MAIN OFFICE)"#]
    #[doc = r#"Description: COUNTRY NAME (MAIN OFFICE)"#]
    pub cntryna: Option<String>,

    #[doc = r#"Title: COUNTRY NAME (BRANCH)"#]
    #[doc = r#"Description: COUNTRY NAME (BRANCH)"#]
    pub cntrynab: Option<String>,

    #[doc = r#"Title: COUNTY NAME (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: COUNTY NAME (BRANCH) This field can be used for search and filtering."#]
    pub cntynamb: Option<String>,

    #[doc = r#"Title: FIPS COUNTY CODE (BRANCH)"#]
    #[doc = r#"Description: FIPS COUNTY CODE (BRANCH)"#]
    pub cntynumb: Option<f32>,

    #[doc = r#"Title: CONSOLIDATED BRANCH NUMBER"#]
    #[doc = r#"Description: CONSOLIDATED BRANCH NUMBER"#]
    pub consold: Option<f32>,

    #[doc = r#"Title: CSA NUMBER (BRANCH)"#]
    #[doc = r#"Description: CSA NUMBER (BRANCH)"#]
    pub csabr: Option<f32>,

    #[doc = r#"Title: CSA NAME (BRANCH)"#]
    #[doc = r#"Description: CSA NAME (BRANCH)"#]
    pub csanambr: Option<String>,

    #[doc = r#"Title: DENOVO FLAG"#]
    #[doc = r#"Description: DENOVO FLAG"#]
    pub denovo: Option<f32>,

    #[doc = r#"Title: TOTAL DOMESTIC DEPOSITS"#]
    #[doc = r#"Description: TOTAL DOMESTIC DEPOSITS"#]
    pub depdom: Option<f32>,

    #[doc = r#"Title: TOTAL DEPOSITS"#]
    #[doc = r#"Description: TOTAL DEPOSITS"#]
    pub depsum: Option<f32>,

    #[doc = r#"Title: DOMESTIC DEPOSITS (SOD)"#]
    #[doc = r#"Description: DOMESTIC DEPOSITS (SOD)"#]
    pub depsumbr: Option<f32>,

    #[doc = r#"Title: CBSA DIVISION CODE (BRANCH)"#]
    #[doc = r#"Description: CBSA DIVISION CODE (BRANCH)"#]
    pub divisionb: Option<f32>,

    #[doc = r#"Title: OTS DOCKET NUMBER"#]
    #[doc = r#"Description: OTS DOCKET NUMBER"#]
    pub docket: Option<f32>,

    #[doc = r#"Title: ESCROW ACCOUNTS - TFR"#]
    #[doc = r#"Description: ESCROW ACCOUNTS - TFR"#]
    pub escrow: Option<f32>,

    #[doc = r#"Title: FDIC REGION NUMBER"#]
    #[doc = r#"Description: FDIC REGION NUMBER"#]
    pub fdicdbs: Option<f32>,

    #[doc = r#"Title: FDIC REGION NAME"#]
    #[doc = r#"Description: FDIC REGION NAME"#]
    pub fdicname: Option<String>,

    #[doc = r#"Title: FRB DISTRICT NUMBER"#]
    #[doc = r#"Description: FRB DISTRICT NUMBER"#]
    pub fed: Option<f32>,

    #[doc = r#"Title: FED DISTRICT NAME"#]
    #[doc = r#"Description: FED DISTRICT NAME"#]
    pub fedname: Option<String>,

    #[doc = r#"Title: MULTI-BANK HOLDING CO"#]
    #[doc = r#"Description: MULTI-BANK HOLDING CO"#]
    pub hctmult: Option<String>,

    #[doc = r#"Title: PRIMARY INSURANCE FUND"#]
    #[doc = r#"Description: PRIMARY INSURANCE FUND"#]
    pub insagnt1: Option<String>,

    #[doc = r#"Title: DEMAND DEPOSITS IN INSURED BRANCHES"#]
    #[doc = r#"Description: DEMAND DEPOSITS IN INSURED BRANCHES"#]
    pub insbrdd: Option<f32>,

    #[doc = r#"Title: TIME & SAVINGS DEPOSITS IN INSURED BRANCHES"#]
    #[doc = r#"Description: TIME & SAVINGS DEPOSITS IN INSURED BRANCHES"#]
    pub insbrts: Option<f32>,

    #[doc = r#"Title: INSURED (Search-Eligible)"#]
    #[doc = r#"Description: INSURED This field can be used for search and filtering."#]
    pub insured: Option<String>,

    #[doc = r#"Title: METRO FLAG (BRANCH)"#]
    #[doc = r#"Description: METRO FLAG (BRANCH)"#]
    pub metrobr: Option<f32>,

    #[doc = r#"Title: MICRO FLAG (BRANCH)"#]
    #[doc = r#"Description: MICRO FLAG (BRANCH)"#]
    pub microbr: Option<f32>,

    #[doc = r#"Title: MSA (BRANCH)"#]
    #[doc = r#"Description: MSA (BRANCH)"#]
    pub msabr: Option<f32>,

    #[doc = r#"Title: MSA NAME (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: MSA NAME (BRANCH) This field can be used for search and filtering."#]
    pub msanamb: Option<String>,

    #[doc = r#"Title: INSTITUTION NAME (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: INSTITUTION NAME (BRANCH) This field can be used for search and filtering."#]
    pub namebr: Option<String>,

    #[doc = r#"Title: INSTITUTION NAME (MAIN OFFICE) (Search-Eligible)"#]
    #[doc = r#"Description: INSTITUTION NAME (MAIN OFFICE) This field can be used for search and filtering."#]
    pub namefull: Option<String>,

    #[doc = r#"Title: BANK HOLDING CO. NAME (Search-Eligible)"#]
    #[doc = r#"Description: BANK HOLDING CO. NAME This field can be used for search and filtering."#]
    pub namehcr: Option<String>,

    #[doc = r#"Title: NECTA NAME (BRANCH)"#]
    #[doc = r#"Description: NECTA NAME (BRANCH)"#]
    pub necnamb: Option<String>,

    #[doc = r#"Title: NECTA (BRANCH)"#]
    #[doc = r#"Description: NECTA (BRANCH)"#]
    pub nectabr: Option<String>,

    #[doc = r#"Title: OCC DISTRICT NUMBER"#]
    #[doc = r#"Description: OCC DISTRICT NUMBER"#]
    pub occdist: Option<f32>,

    #[doc = r#"Title: OCC REGION NAME"#]
    #[doc = r#"Description: OCC REGION NAME"#]
    pub occname: Option<String>,

    #[doc = r#"Title: PLACE CODE NUMBER (DF)"#]
    #[doc = r#"Description: PLACE CODE NUMBER (DF)"#]
    pub placenum: Option<f32>,

    #[doc = r#"Title: PRIMARY FEDERAL REGULATOR"#]
    #[doc = r#"Description: PRIMARY FEDERAL REGULATOR"#]
    pub regagnt: Option<String>,

    #[doc = r#"Title: FRB ID NUMBER - BHC"#]
    #[doc = r#"Description: FRB ID NUMBER - BHC"#]
    pub rssdhcr: Option<f32>,

    #[doc = r#"Title: FRB ID NUMBER"#]
    #[doc = r#"Description: FRB ID NUMBER"#]
    pub rssdid: Option<f32>,

    #[doc = r#"Title: SIMS ACQUIRED DATE"#]
    #[doc = r#"Description: SIMS ACQUIRED DATE"#]
    pub sims_acquired_date: Option<String>,

    #[doc = r#"Title: SIMS MATCH CODE (DESCRIPTION)"#]
    #[doc = r#"Description: SIMS MATCH CODE (DESCRIPTION)"#]
    pub sims_description: Option<String>,

    #[doc = r#"Title: SIMS ESTABLISHED DATE"#]
    #[doc = r#"Description: SIMS ESTABLISHED DATE"#]
    pub sims_established_date: Option<String>,

    #[doc = r#"Title: SIMS GEOGRAPHIC LATITUDE"#]
    #[doc = r#"Description: SIMS GEOGRAPHIC LATITUDE"#]
    pub sims_latitude: Option<f32>,

    #[doc = r#"Title: SIMS GEOGRAPHIC LONGITUDE"#]
    #[doc = r#"Description: SIMS GEOGRAPHIC LONGITUDE"#]
    pub sims_longitude: Option<f32>,

    #[doc = r#"Title: SIMS SCORE (PROJECTION)"#]
    #[doc = r#"Description: SIMS SCORE (PROJECTION)"#]
    pub sims_projection: Option<String>,

    #[doc = r#"Title: SPECGRP DESCRIPTION"#]
    #[doc = r#"Description: SPECGRP DESCRIPTION"#]
    pub specdesc: Option<String>,

    #[doc = r#"Title: INDUSTRY SPECIALIZATION GROUP"#]
    #[doc = r#"Description: INDUSTRY SPECIALIZATION GROUP"#]
    pub specgrp: Option<f32>,

    #[doc = r#"Title: FIPS STATE ALPHA CODE (MAIN OFFICE) (Search-Eligible)"#]
    #[doc = r#"Description: FIPS STATE ALPHA CODE (MAIN OFFICE) This field can be used for search and filtering."#]
    pub stalp: Option<String>,

    #[doc = r#"Title: FIPS STATE ALPHA CODE (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: FIPS STATE ALPHA CODE (BRANCH) This field can be used for search and filtering."#]
    pub stalpbr: Option<String>,

    #[doc = r#"Title: FIPS STATE ALPHA CODE - BHC (Search-Eligible)"#]
    #[doc = r#"Description: FIPS STATE ALPHA CODE - BHC This field can be used for search and filtering."#]
    pub stalphcr: Option<String>,

    #[doc = r#"Title: FIPS STATE & COUNTY NO. (MAIN OFFICE)"#]
    #[doc = r#"Description: FIPS STATE & COUNTY NO. (MAIN OFFICE)"#]
    pub stcnty: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub stcntybr: Option<f32>,

    #[doc = r#"Title: FIPS STATE NAME (MAIN OFFICE) (Search-Eligible)"#]
    #[doc = r#"Description: FIPS STATE NAME (MAIN OFFICE) This field can be used for search and filtering."#]
    pub stname: Option<String>,

    #[doc = r#"Title: STATE NAME (BRANCH) (Search-Eligible)"#]
    #[doc = r#"Description: STATE NAME (BRANCH) This field can be used for search and filtering."#]
    pub stnamebr: Option<String>,

    #[doc = r#"Title: FIPS STATE CODE (BRANCH)"#]
    #[doc = r#"Description: FIPS STATE CODE (BRANCH)"#]
    pub stnumbr: Option<f32>,

    #[doc = r#"Title: UNINUM (BRANCH)"#]
    #[doc = r#"Description: UNINUM (BRANCH)"#]
    pub uninumbr: Option<f32>,

    #[doc = r#"Title: UNIT BANK FLAG"#]
    #[doc = r#"Description: UNIT BANK FLAG"#]
    pub unit: Option<f32>,

    #[doc = r#"Title: USA LOCATED INSTITUTION"#]
    #[doc = r#"Description: USA LOCATED INSTITUTION"#]
    pub usa: Option<f32>,

    #[doc = r#"Title: YEAR"#]
    #[doc = r#"Description: YEAR"#]
    pub year: Option<f32>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub zip_raw: Option<String>,

    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub zipbr_raw: Option<String>,

    #[doc = r#"Title: Zip Code (Search-Eligible)"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices. This field can be used for search and filtering."#]
    pub zip: Option<String>,

    #[doc = r#"Title: Zip Code (Search-Eligible)"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices. This field can be used for search and filtering."#]
    pub zipbr: Option<String>,

}

/// Auto-generated response envelope struct for `/sod` endpoint.
/// Spec: sod_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SodResponse {
    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub data: Option<String>,

}

/// FDIC BankFind API `/sod` endpoint handler
/// Get Summary of Deposits Information for FDIC Insured Institutions
/// Returns summary of deposits information for institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
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
pub async fn sod_handler(
    config: &FdicApiConfig,
    params: &SodParameters,
) -> Result<CallToolResult, rmcp::Error> {
   
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "sod",
        method = "GET",
        path = "/sod",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp
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
