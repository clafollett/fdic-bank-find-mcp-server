//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/sod` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/sod` endpoint.
/// Spec: sod_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct SodParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_by")]
    pub agg_by: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_term_fields")]
    pub agg_term_fields: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_sum_fields")]
    pub agg_sum_fields: Option<String>,
    #[doc = r#"The limit on how many aggregated results will be displayed"#]
    #[param(rename = "agg_limit")]
    pub agg_limit: Option<u32>,
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
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct SodProperties {
    #[doc = r#"## FDIC Field:: `ADDRESBR`
    Title: ADDRESS (BRANCH)
    Description: ADDRESS (BRANCH)"#]
    #[serde(rename="ADDRESBR")]
    pub address_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `ADDRESS`
    Title: ADDRESS
    Description: ADDRESS"#]
    #[serde(rename="ADDRESS")]
    pub address: Option<String>,

    #[doc = r#"## FDIC Field:: `ASSET`
    Title: TOTAL ASSETS
    Description: TOTAL ASSETS"#]
    #[serde(rename="ASSET")]
    pub total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `BKCLASS`
    Title: INSTITUTION CLASS
    Description: INSTITUTION CLASS"#]
    #[serde(rename="BKCLASS")]
    pub institution_class: Option<String>,

    #[doc = r#"## FDIC Field:: `BKMO`
    Title: MAIN OFFICE DESIGNATION FLAG
    Description: MAIN OFFICE DESIGNATION FLAG"#]
    #[serde(rename="BKMO")]
    pub main_office_designation_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `BRCENM`
    Title: CENCODES C
    Description: CENCODES C"#]
    #[serde(rename="BRCENM")]
    pub cencodes_c: Option<String>,

    #[doc = r#"## FDIC Field:: `BRNUM`
    Title: BRANCH NUMBER
    Description: BRANCH NUMBER"#]
    #[serde(rename="BRNUM")]
    pub branch_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `BRSERTYP`
    Title: BRANCH SERVICE TYPE
    Description: BRANCH SERVICE TYPE"#]
    #[serde(rename="BRSERTYP")]
    pub branch_service_type: Option<f64>,

    #[doc = r#"## FDIC Field:: `CALL`
    Title: REPORT TYPE
    Description: REPORT TYPE"#]
    #[serde(rename="CALL")]
    pub report_type: Option<String>,

    #[doc = r#"## FDIC Field:: `CB`
    Title: COMMUNITY BANK FLAG
    Description: COMMUNITY BANK FLAG"#]
    #[serde(rename="CB")]
    pub community_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `CBSA_DIV_NAMB`
    Title: CBSA DIVISION NAME (BRANCH)
    Description: CBSA DIVISION NAME (BRANCH)"#]
    #[serde(rename="CBSA_DIV_NAMB")]
    pub cbsa_division_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: FDIC CERT NUMBER
    Description: FDIC CERT NUMBER"#]
    #[serde(rename="CERT")]
    pub fdic_cert_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHARTER`
    Title: CHARTER
    Description: CHARTER"#]
    #[serde(rename="CHARTER")]
    pub charter: Option<String>,

    #[doc = r#"## FDIC Field:: `CHRTAGNN`
    Title: CHARTER AGENT NAME
    Description: CHARTER AGENT NAME"#]
    #[serde(rename="CHRTAGNN")]
    pub charter_agent_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CHRTAGNT`
    Title: CHARTER AGENT CODE
    Description: CHARTER AGENT CODE"#]
    #[serde(rename="CHRTAGNT")]
    pub charter_agent_code: Option<String>,

    #[doc = r#"## FDIC Field:: `CITY`
    Title: CITY (MAIN OFFICE)
    Description: CITY (MAIN OFFICE)"#]
    #[serde(rename="CITY")]
    pub city_main_office: Option<String>,

    #[doc = r#"## FDIC Field:: `CITY2BR`
    Title: PREFERRED CITY (BRANCH)
    Description: PREFERRED CITY (BRANCH)"#]
    #[serde(rename="CITY2BR")]
    pub preferred_city_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CITYBR`
    Title: CITY (BRANCH)
    Description: CITY (BRANCH)"#]
    #[serde(rename="CITYBR")]
    pub city_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CITYHCR`
    Title: CITY-HOLDING CO.- REGULATORY
    Description: CITY-HOLDING CO.- REGULATORY"#]
    #[serde(rename="CITYHCR")]
    pub city_holding_co_regulatory: Option<String>,

    #[doc = r#"## FDIC Field:: `CLCODE`
    Title: CLASS NUMBER
    Description: CLASS NUMBER"#]
    #[serde(rename="CLCODE")]
    pub class_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CNTRYNA`
    Title: COUNTRY NAME (MAIN OFFICE)
    Description: COUNTRY NAME (MAIN OFFICE)"#]
    #[serde(rename="CNTRYNA")]
    pub country_name_main_office: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTRYNAB`
    Title: COUNTRY NAME (BRANCH)
    Description: COUNTRY NAME (BRANCH)"#]
    #[serde(rename="CNTRYNAB")]
    pub country_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTYNAMB`
    Title: COUNTY NAME (BRANCH)
    Description: COUNTY NAME (BRANCH)"#]
    #[serde(rename="CNTYNAMB")]
    pub county_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTYNUMB`
    Title: FIPS COUNTY CODE (BRANCH)
    Description: FIPS COUNTY CODE (BRANCH)"#]
    #[serde(rename="CNTYNUMB")]
    pub fips_county_code_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `CONSOLD`
    Title: CONSOLIDATED BRANCH NUMBER
    Description: CONSOLIDATED BRANCH NUMBER"#]
    #[serde(rename="CONSOLD")]
    pub consolidated_branch_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CSABR`
    Title: CSA NUMBER (BRANCH)
    Description: CSA NUMBER (BRANCH)"#]
    #[serde(rename="CSABR")]
    pub csa_number_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `CSANAMBR`
    Title: CSA NAME (BRANCH)
    Description: CSA NAME (BRANCH)"#]
    #[serde(rename="CSANAMBR")]
    pub csa_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `DENOVO`
    Title: DENOVO FLAG
    Description: DENOVO FLAG"#]
    #[serde(rename="DENOVO")]
    pub denovo_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPDOM`
    Title: TOTAL DOMESTIC DEPOSITS
    Description: TOTAL DOMESTIC DEPOSITS"#]
    #[serde(rename="DEPDOM")]
    pub total_domestic_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSUM`
    Title: TOTAL DEPOSITS
    Description: TOTAL DEPOSITS"#]
    #[serde(rename="DEPSUM")]
    pub total_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPSUMBR`
    Title: DOMESTIC DEPOSITS (SOD)
    Description: DOMESTIC DEPOSITS (SOD)"#]
    #[serde(rename="DEPSUMBR")]
    pub domestic_deposits_sod: Option<f64>,

    #[doc = r#"## FDIC Field:: `DIVISIONB`
    Title: CBSA DIVISION CODE (BRANCH)
    Description: CBSA DIVISION CODE (BRANCH)"#]
    #[serde(rename="DIVISIONB")]
    pub cbsa_division_code_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `DOCKET`
    Title: OTS DOCKET NUMBER
    Description: OTS DOCKET NUMBER"#]
    #[serde(rename="DOCKET")]
    pub ots_docket_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `ESCROW`
    Title: ESCROW ACCOUNTS - TFR
    Description: ESCROW ACCOUNTS - TFR"#]
    #[serde(rename="ESCROW")]
    pub escrow_accounts_tfr: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICDBS`
    Title: FDIC REGION NUMBER
    Description: FDIC REGION NUMBER"#]
    #[serde(rename="FDICDBS")]
    pub fdic_region_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICNAME`
    Title: FDIC REGION NAME
    Description: FDIC REGION NAME"#]
    #[serde(rename="FDICNAME")]
    pub fdic_region_name: Option<String>,

    #[doc = r#"## FDIC Field:: `FED`
    Title: FRB DISTRICT NUMBER
    Description: FRB DISTRICT NUMBER"#]
    #[serde(rename="FED")]
    pub frb_district_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `FEDNAME`
    Title: FED DISTRICT NAME
    Description: FED DISTRICT NAME"#]
    #[serde(rename="FEDNAME")]
    pub fed_district_name: Option<String>,

    #[doc = r#"## FDIC Field:: `HCTMULT`
    Title: MULTI-BANK HOLDING CO
    Description: MULTI-BANK HOLDING CO"#]
    #[serde(rename="HCTMULT")]
    pub multi_bank_holding_co: Option<String>,

    #[doc = r#"## FDIC Field:: `INSAGNT1`
    Title: PRIMARY INSURANCE FUND
    Description: PRIMARY INSURANCE FUND"#]
    #[serde(rename="INSAGNT1")]
    pub primary_insurance_fund: Option<String>,

    #[doc = r#"## FDIC Field:: `INSBRDD`
    Title: DEMAND DEPOSITS IN INSURED BRANCHES
    Description: DEMAND DEPOSITS IN INSURED BRANCHES"#]
    #[serde(rename="INSBRDD")]
    pub demand_deposits_in_insured_branches: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSBRTS`
    Title: TIME & SAVINGS DEPOSITS IN INSURED BRANCHES
    Description: TIME & SAVINGS DEPOSITS IN INSURED BRANCHES"#]
    #[serde(rename="INSBRTS")]
    pub time_savings_deposits_in_insured_branches: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSURED`
    Title: INSURED
    Description: INSURED"#]
    #[serde(rename="INSURED")]
    pub insured: Option<String>,

    #[doc = r#"## FDIC Field:: `METROBR`
    Title: METRO FLAG (BRANCH)
    Description: METRO FLAG (BRANCH)"#]
    #[serde(rename="METROBR")]
    pub metro_flag_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `MICROBR`
    Title: MICRO FLAG (BRANCH)
    Description: MICRO FLAG (BRANCH)"#]
    #[serde(rename="MICROBR")]
    pub micro_flag_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSABR`
    Title: MSA (BRANCH)
    Description: MSA (BRANCH)"#]
    #[serde(rename="MSABR")]
    pub msa_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `MSANAMB`
    Title: MSA NAME (BRANCH)
    Description: MSA NAME (BRANCH)"#]
    #[serde(rename="MSANAMB")]
    pub msa_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `NAMEBR`
    Title: INSTITUTION NAME (BRANCH)
    Description: INSTITUTION NAME (BRANCH)"#]
    #[serde(rename="NAMEBR")]
    pub institution_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `NAMEFULL`
    Title: INSTITUTION NAME (MAIN OFFICE)
    Description: INSTITUTION NAME (MAIN OFFICE)"#]
    #[serde(rename="NAMEFULL")]
    pub institution_name_main_office: Option<String>,

    #[doc = r#"## FDIC Field:: `NAMEHCR`
    Title: BANK HOLDING CO. NAME
    Description: BANK HOLDING CO. NAME"#]
    #[serde(rename="NAMEHCR")]
    pub bank_holding_co_name: Option<String>,

    #[doc = r#"## FDIC Field:: `NECNAMB`
    Title: NECTA NAME (BRANCH)
    Description: NECTA NAME (BRANCH)"#]
    #[serde(rename="NECNAMB")]
    pub necta_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `NECTABR`
    Title: NECTA (BRANCH)
    Description: NECTA (BRANCH)"#]
    #[serde(rename="NECTABR")]
    pub necta_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `OCCDIST`
    Title: OCC DISTRICT NUMBER
    Description: OCC DISTRICT NUMBER"#]
    #[serde(rename="OCCDIST")]
    pub occ_district_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `OCCNAME`
    Title: OCC REGION NAME
    Description: OCC REGION NAME"#]
    #[serde(rename="OCCNAME")]
    pub occ_region_name: Option<String>,

    #[doc = r#"## FDIC Field:: `PLACENUM`
    Title: PLACE CODE NUMBER (DF)
    Description: PLACE CODE NUMBER (DF)"#]
    #[serde(rename="PLACENUM")]
    pub place_code_number_df: Option<f64>,

    #[doc = r#"## FDIC Field:: `REGAGNT`
    Title: PRIMARY FEDERAL REGULATOR
    Description: PRIMARY FEDERAL REGULATOR"#]
    #[serde(rename="REGAGNT")]
    pub primary_federal_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `RSSDHCR`
    Title: FRB ID NUMBER - BHC
    Description: FRB ID NUMBER - BHC"#]
    #[serde(rename="RSSDHCR")]
    pub frb_id_number_bhc: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSSDID`
    Title: FRB ID NUMBER
    Description: FRB ID NUMBER"#]
    #[serde(rename="RSSDID")]
    pub frb_id_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_ACQUIRED_DATE`
    Title: SIMS ACQUIRED DATE
    Description: SIMS ACQUIRED DATE"#]
    #[serde(rename="SIMS_ACQUIRED_DATE")]
    pub sims_acquired_date: Option<String>,

    #[doc = r#"## FDIC Field:: `SIMS_DESCRIPTION`
    Title: SIMS MATCH CODE (DESCRIPTION)
    Description: SIMS MATCH CODE (DESCRIPTION)"#]
    #[serde(rename="SIMS_DESCRIPTION")]
    pub sims_match_code_description: Option<String>,

    #[doc = r#"## FDIC Field:: `SIMS_ESTABLISHED_DATE`
    Title: SIMS ESTABLISHED DATE
    Description: SIMS ESTABLISHED DATE"#]
    #[serde(rename="SIMS_ESTABLISHED_DATE")]
    pub sims_established_date: Option<String>,

    #[doc = r#"## FDIC Field:: `SIMS_LATITUDE`
    Title: SIMS GEOGRAPHIC LATITUDE
    Description: SIMS GEOGRAPHIC LATITUDE"#]
    #[serde(rename="SIMS_LATITUDE")]
    pub sims_geographic_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_LONGITUDE`
    Title: SIMS GEOGRAPHIC LONGITUDE
    Description: SIMS GEOGRAPHIC LONGITUDE"#]
    #[serde(rename="SIMS_LONGITUDE")]
    pub sims_geographic_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_PROJECTION`
    Title: SIMS SCORE (PROJECTION)
    Description: SIMS SCORE (PROJECTION)"#]
    #[serde(rename="SIMS_PROJECTION")]
    pub sims_score_projection: Option<String>,

    #[doc = r#"## FDIC Field:: `SPECDESC`
    Title: SPECGRP DESCRIPTION
    Description: SPECGRP DESCRIPTION"#]
    #[serde(rename="SPECDESC")]
    pub specgrp_description: Option<String>,

    #[doc = r#"## FDIC Field:: `SPECGRP`
    Title: INDUSTRY SPECIALIZATION GROUP
    Description: INDUSTRY SPECIALIZATION GROUP"#]
    #[serde(rename="SPECGRP")]
    pub industry_specialization_group: Option<f64>,

    #[doc = r#"## FDIC Field:: `STALP`
    Title: FIPS STATE ALPHA CODE (MAIN OFFICE)
    Description: FIPS STATE ALPHA CODE (MAIN OFFICE)"#]
    #[serde(rename="STALP")]
    pub fips_state_alpha_code_main_office: Option<String>,

    #[doc = r#"## FDIC Field:: `STALPBR`
    Title: FIPS STATE ALPHA CODE (BRANCH)
    Description: FIPS STATE ALPHA CODE (BRANCH)"#]
    #[serde(rename="STALPBR")]
    pub fips_state_alpha_code_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `STALPHCR`
    Title: FIPS STATE ALPHA CODE - BHC
    Description: FIPS STATE ALPHA CODE - BHC"#]
    #[serde(rename="STALPHCR")]
    pub fips_state_alpha_code_bhc: Option<String>,

    #[doc = r#"## FDIC Field:: `STCNTY`
    Title: FIPS STATE & COUNTY NO. (MAIN OFFICE)
    Description: FIPS STATE & COUNTY NO. (MAIN OFFICE)"#]
    #[serde(rename="STCNTY")]
    pub fips_state_county_no_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `STCNTYBR`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="STCNTYBR")]
    pub missing_title: Option<f64>,

    #[doc = r#"## FDIC Field:: `STNAME`
    Title: FIPS STATE NAME (MAIN OFFICE)
    Description: FIPS STATE NAME (MAIN OFFICE)"#]
    #[serde(rename="STNAME")]
    pub fips_state_name_main_office: Option<String>,

    #[doc = r#"## FDIC Field:: `STNAMEBR`
    Title: STATE NAME (BRANCH)
    Description: STATE NAME (BRANCH)"#]
    #[serde(rename="STNAMEBR")]
    pub state_name_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `STNUMBR`
    Title: FIPS STATE CODE (BRANCH)
    Description: FIPS STATE CODE (BRANCH)"#]
    #[serde(rename="STNUMBR")]
    pub fips_state_code_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNINUMBR`
    Title: UNINUM (BRANCH)
    Description: UNINUM (BRANCH)"#]
    #[serde(rename="UNINUMBR")]
    pub uninum_branch: Option<f64>,

    #[doc = r#"## FDIC Field:: `UNIT`
    Title: UNIT BANK FLAG
    Description: UNIT BANK FLAG"#]
    #[serde(rename="UNIT")]
    pub unit_bank_flag: Option<f64>,

    #[doc = r#"## FDIC Field:: `USA`
    Title: USA LOCATED INSTITUTION
    Description: USA LOCATED INSTITUTION"#]
    #[serde(rename="USA")]
    pub usa_located_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `YEAR`
    Title: YEAR
    Description: YEAR"#]
    #[serde(rename="YEAR")]
    pub year: Option<f64>,

    #[doc = r#"## FDIC Field:: `ZIP_RAW`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ZIP_RAW")]
    pub missing_title_zip_raw: Option<String>,

    #[doc = r#"## FDIC Field:: `ZIPBR_RAW`
    Title: MISSING TITLE
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="ZIPBR_RAW")]
    pub missing_title_zipbr_raw: Option<String>,

    #[doc = r#"## FDIC Field:: `ZIP`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#]
    #[serde(rename="ZIP")]
    pub zip_code: Option<String>,

    #[doc = r#"## FDIC Field:: `ZIPBR`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#]
    #[serde(rename="ZIPBR")]
    pub zip_code_zipbr: Option<String>,

}

/// FDIC BankFind API `/sod` endpoint handler
/// Get Summary of Deposits Information for FDIC Insured Institutions
/// Returns summary of deposits information for institutions
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
Examples:
* Filter data by the numeric range
`ASSET:&#91;1000 TO 9999&#93;`
    Example: CERT:14
 - `fields` (String, optional): Comma delimited list of fields with quarterly financial data to return. All values must be entered in UPPERCASE.
    Example: CERT,YEAR,ASSET,DEPSUMBR,STALPBR
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: YEAR
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
    Example: CERT
 - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
    Example: YEAR
 - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
    Example: ASSET
 - `agg_limit` (u32, optional): The limit on how many aggregated results will be displayed
    Example: 1
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/sod",
    params(SodParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<SodProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Summary of Deposits"
)]
pub async fn sod_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<SodParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "sod",
    )
    .await
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
            
            address_branch: None,
            address: None,
            total_assets: None,
            institution_class: None,
            main_office_designation_flag: None,
            cencodes_c: None,
            branch_number: None,
            branch_service_type: None,
            report_type: None,
            community_bank_flag: None,
            cbsa_division_name_branch: None,
            fdic_cert_number: None,
            charter: None,
            charter_agent_name: None,
            charter_agent_code: None,
            city_main_office: None,
            preferred_city_branch: None,
            city_branch: None,
            city_holding_co_regulatory: None,
            class_number: None,
            country_name_main_office: None,
            country_name_branch: None,
            county_name_branch: None,
            fips_county_code_branch: None,
            consolidated_branch_number: None,
            csa_number_branch: None,
            csa_name_branch: None,
            denovo_flag: None,
            total_domestic_deposits: None,
            total_deposits: None,
            domestic_deposits_sod: None,
            cbsa_division_code_branch: None,
            ots_docket_number: None,
            escrow_accounts_tfr: None,
            fdic_region_number: None,
            fdic_region_name: None,
            frb_district_number: None,
            fed_district_name: None,
            multi_bank_holding_co: None,
            primary_insurance_fund: None,
            demand_deposits_in_insured_branches: None,
            time_savings_deposits_in_insured_branches: None,
            insured: None,
            metro_flag_branch: None,
            micro_flag_branch: None,
            msa_branch: None,
            msa_name_branch: None,
            institution_name_branch: None,
            institution_name_main_office: None,
            bank_holding_co_name: None,
            necta_name_branch: None,
            necta_branch: None,
            occ_district_number: None,
            occ_region_name: None,
            place_code_number_df: None,
            primary_federal_regulator: None,
            frb_id_number_bhc: None,
            frb_id_number: None,
            sims_acquired_date: None,
            sims_match_code_description: None,
            sims_established_date: None,
            sims_geographic_latitude: None,
            sims_geographic_longitude: None,
            sims_score_projection: None,
            specgrp_description: None,
            industry_specialization_group: None,
            fips_state_alpha_code_main_office: None,
            fips_state_alpha_code_branch: None,
            fips_state_alpha_code_bhc: None,
            fips_state_county_no_main_office: None,
            missing_title: None,
            fips_state_name_main_office: None,
            state_name_branch: None,
            fips_state_code_branch: None,
            uninum_branch: None,
            unit_bank_flag: None,
            usa_located_institution: None,
            year: None,
            missing_title_zip_raw: None,
            missing_title_zipbr_raw: None,
            zip_code: None,
            zip_code_zipbr: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
