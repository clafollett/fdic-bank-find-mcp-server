//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/history` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/history` endpoint.
/// Spec: history_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct HistoryParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"Flexible text search against institution records
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by Name
`NAME: Island`
* Search by Name (fuzzy match)
`NAME: Iland`
* Search by State
`STATE: VA`"#]
    #[param(rename = "search")]
    pub search: Option<String>,
    #[doc = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_by")]
    pub agg_by: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#]
    #[param(rename = "agg_term_fields")]
    pub agg_term_fields: Option<String>,
    #[doc = r#"The limit on how many aggregated results will be displayed"#]
    #[param(rename = "agg_limit")]
    pub agg_limit: Option<u32>,
}

// Implement QueryParameters for generic handler
impl QueryParameters for HistoryParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "TRANSNUM",
        "CHANGECODE",
        "CHANGECODE_DESC",
        "PROCDATE",
        "EFFDATE",
        "ENDDATE",
        "UNINUM",
        "ACQ_UNINUM",
        "OUT_UNINUM",
        "ORG_ROLE_CDE",
        "REPORT_TYPE",
        "CLASS",
        "BANK_INSURED",
        "ACQ_CHANGECODE",
        "ACQ_ORG_EFF_DTE",
        "ACQ_INSTNAME",
        "ACQ_CERT",
        "ACQ_CLCODE",
        "ACQ_CHARTER",
        "ACQ_CHARTAGENT",
        "ACQ_FDICREGION",
        "ACQ_FDICREGION_DESC",
        "ACQ_PADDR",
        "ACQ_PCITY",
        "ACQ_PSTALP",
        "ACQ_PZIP5",
        "ACQ_PZIPREST",
        "ACQ_MADDR",
        "ACQ_MCITY",
        "ACQ_MSTATE",
        "ACQ_MSTALP",
        "ACQ_MZIP5",
        "ACQ_MZIPREST",
        "ACQ_CLASS",
        "ACQ_CNTYNAME",
        "ACQ_CNTYNUM",
        "ACQ_INSAGENT1",
        "ACQ_INSAGENT2",
        "ACQ_REGAGENT",
        "ACQ_TRUST",
        "ACQ_LATITUDE",
        "ACQ_LONGITUDE",
        "OUT_INSTNAME",
        "OUT_CERT",
        "OUT_CLCODE",
        "OUT_CHARTER",
        "OUT_CHARTAGENT",
        "OUT_FDICREGION",
        "OUT_FDICREGION_DESC",
        "OUT_PADDR",
        "OUT_PCITY",
        "OUT_PSTALP",
        "OUT_PZIP5",
        "OUT_PZIPREST",
        "OUT_MADDR",
        "OUT_MCITY",
        "OUT_MSTATE",
        "OUT_MSTALP",
        "OUT_MZIP5",
        "OUT_MZIPREST",
        "OUT_CLASS",
        "OUT_CNTYNAME",
        "OUT_CNTYNUM",
        "OUT_INSAGENT1",
        "OUT_INSAGENT2",
        "OUT_REGAGENT",
        "OUT_TRUST",
        "OUT_LATITUDE",
        "OUT_LONGITUDE",
        "SUR_CHANGECODE",
        "SUR_CHANGECODE_DESC",
        "SUR_INSTNAME",
        "SUR_CERT",
        "SUR_CLCODE",
        "SUR_CHARTER",
        "SUR_CHARTAGENT",
        "SUR_FDICREGION",
        "SUR_FDICREGION_DESC",
        "SUR_MADDR",
        "SUR_MCITY",
        "SUR_MSTATE",
        "SUR_MSTALP",
        "SUR_MZIP5",
        "SUR_PZIP5",
        "SUR_CLASS",
        "SUR_CNTYNAME",
        "SUR_CNTYNUM",
        "SUR_INSAGENT1",
        "SUR_INSAGENT2",
        "SUR_PADDR",
        "SUR_PCITY",
        "SUR_PSTALP",
        "SUR_PZIPREST",
        "SUR_REGAGENT",
        "SUR_TRUST",
        "SUR_LATITUDE",
        "SUR_LONGITUDE",
        "FRM_CNTYNUM",
        "FRM_PCITY",
        "FRM_REGAGENT",
        "FRM_PSTALP",
        "FRM_TRUST",
        "FRM_CLCODE",
        "FRM_PADDR",
        "FRM_CHARTAGENT",
        "FRM_CLASS",
        "FRM_PZIP5",
        "FRM_PZIPREST",
        "FRM_INSTNAME",
        "FRM_CNTYNAME",
        "FRM_CERT",
        "FRM_OFF_CNTYNAME",
        "FRM_OFF_CNTYNUM",
        "FRM_OFF_PADDR",
        "FRM_OFF_PCITY",
        "FRM_OFF_PSTALP",
        "FRM_OFF_PZIP5",
        "FRM_OFF_PZIPREST",
        "FRM_OFF_SERVTYPE",
        "FRM_OFF_SERVTYPE_DESC",
        "FRM_OFF_STATE",
        "FRM_OFF_NAME",
        "FRM_OFF_NUM",
        "FRM_OFF_TRUST",
        "FRM_OFF_CLCODE",
        "FRM_OFF_LATITUDE",
        "FRM_OFF_LONGITUDE",
        "FRM_LATITUDE",
        "FRM_LONGITUDE",
        "CERT",
        "INSTNAME",
        "CHARTAGENT",
        "CLCODE",
        "FDICREGION",
        "FDICREGION_DESC",
        "CNTYNAME",
        "CNTYNUM",
        "INSAGENT1",
        "INSAGENT2",
        "MADDR",
        "MCITY",
        "MSTATE",
        "MSTALP",
        "MZIP5",
        "MZIPREST",
        "PADDR",
        "PZIP5",
        "PSTALP",
        "PZIPREST",
        "PCITY",
        "STATE",
        "TRUST",
        "REGAGENT",
        "SERVTYPE",
        "SERVTYPE_DESC",
        "OFF_CNTYNAME",
        "OFF_NUM",
        "OFF_CNTYNUM",
        "OFF_PADDR",
        "OFF_PSTATE",
        "OFF_PZIP5",
        "OFF_PZIPREST",
        "OFF_NAME",
        "OFF_PSTALP",
        "OFF_PCITY",
        "OFF_SERVTYPE",
        "OFF_LATITUDE",
        "OFF_LONGITUDE",
        "OFF_SERVTYPE_DESC",
        "ESTDATE",
        "ACQDATE",
        "FI_EFFDATE",
        "FI_UNINUM",
        "ORG_STAT_FLG",
        "LATITUDE",
        "LONGITUDE",
    ];

    #[allow(unused_variables)]
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        if let Some(val) = &self.search {
            query.insert("search".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_by {
            query.insert("agg_by".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_term_fields {
            query.insert("agg_term_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.agg_limit {
            query.insert("agg_limit".to_string(), val.to_string());
        }
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/history` endpoint.
/// Spec: history_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct HistoryProperties {
    #[doc = r#"## FDIC Field:: `TRANSNUM`
    Title: System Transaction Number
    Description: System Transaction Number"#]
    #[serde(rename="TRANSNUM")]
    pub system_transaction_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHANGECODE`
    Title: Activity Event Code
    Description: Activity Event Code"#]
    #[serde(rename="CHANGECODE")]
    pub activity_event_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `CHANGECODE_DESC`
    Title: Activity Event Code Description
    Description: Activity Event Code Description"#]
    #[serde(rename="CHANGECODE_DESC")]
    pub activity_event_code_description: Option<String>,

    #[doc = r#"## FDIC Field:: `PROCDATE`
    Title: Process Date
    Description: A date indicating when an institution's change/event is processed."#]
    #[serde(rename="PROCDATE")]
    pub process_date: Option<String>,

    #[doc = r#"## FDIC Field:: `EFFDATE`
    Title: Effective Date
    Description: Effective Date"#]
    #[serde(rename="EFFDATE")]
    pub effective_date: Option<String>,

    #[doc = r#"## FDIC Field:: `ENDDATE`
    Title: Effective Date
    Description: Effective Date"#]
    #[serde(rename="ENDDATE")]
    pub effective_date_enddate: Option<String>,

    #[doc = r#"## FDIC Field:: `UNINUM`
    Title: FDIC's unique number
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    #[serde(rename="UNINUM")]
    pub fdic_s_unique_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_UNINUM`
    Title: FDIC's unique number of who is Acquiring
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps to the main office for  the acquiring Institution in a merger, acquisition, etc."#]
    #[serde(rename="ACQ_UNINUM")]
    pub fdic_s_unique_number_of_who_is_acquiring: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_UNINUM`
    Title: FDIC's unique number of who is Divesting
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps to the main office for  the divesting Institution in a merger, acquisition, etc."#]
    #[serde(rename="OUT_UNINUM")]
    pub fdic_s_unique_number_of_who_is_divesting: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORG_ROLE_CDE`
    Title: Organization Role Code
    Description: Codes include FI (Financial Institution), BR (Branch), and PA"#]
    #[serde(rename="ORG_ROLE_CDE")]
    pub organization_role_code: Option<String>,

    #[doc = r#"## FDIC Field:: `REPORT_TYPE`
    Title: Report Type
    Description: Type of Report"#]
    #[serde(rename="REPORT_TYPE")]
    pub report_type: Option<f64>,

    #[doc = r#"## FDIC Field:: `CLASS`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="CLASS")]
    pub tbd: Option<String>,

    #[doc = r#"## FDIC Field:: `BANK_INSURED`
    Title: Bank Insurance Status
    Description: Bank Insurance Status"#]
    #[serde(rename="BANK_INSURED")]
    pub bank_insurance_status: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_CHANGECODE`
    Title: Activity Event Code
    Description: Activity Event Code"#]
    #[serde(rename="ACQ_CHANGECODE")]
    pub activity_event_code_acq_changecode: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_ORG_EFF_DTE`
    Title: Effective Date
    Description: Acquiring Institution's Effective Date"#]
    #[serde(rename="ACQ_ORG_EFF_DTE")]
    pub effective_date_acq_org_eff_dte: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_INSTNAME`
    Title: Institution name
    Description: The legal name of the institution."#]
    #[serde(rename="ACQ_INSTNAME")]
    pub institution_name: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="ACQ_CERT")]
    pub fdic_certificate: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="ACQ_CLCODE")]
    pub numeric_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_CHARTER`
    Title: OCC Charter Number
    Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    #[serde(rename="ACQ_CHARTER")]
    pub occ_charter_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_CHARTAGENT`
    Title: Acquiring Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="ACQ_CHARTAGENT")]
    pub acquiring_chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_FDICREGION`
    Title: Supervisory Region Number
    Description: A numeric value associated with the name of an FDIC supervisory region"#]
    #[serde(rename="ACQ_FDICREGION")]
    pub supervisory_region_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_FDICREGION_DESC`
    Title: Supervisory Region Description
    Description: A description associated with the name of an FDIC supervisory region"#]
    #[serde(rename="ACQ_FDICREGION_DESC")]
    pub supervisory_region_description: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="ACQ_PADDR")]
    pub physical_street_address: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="ACQ_PCITY")]
    pub city: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_PSTALP`
    Title: State Alpha code
    Description: State in which the  acquiring institution's main office or one if its branches are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="ACQ_PSTALP")]
    pub state_alpha_code: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="ACQ_PZIP5")]
    pub zip_code: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="ACQ_PZIPREST")]
    pub zip_code_extension: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MADDR`
    Title: Mailing Street Address
    Description: Street address at which the institution or one of its branches receives mail."#]
    #[serde(rename="ACQ_MADDR")]
    pub mailing_street_address: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="ACQ_MCITY")]
    pub city_acq_mcity: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MSTATE`
    Title: Mailing State
    Description: Mailing State"#]
    #[serde(rename="ACQ_MSTATE")]
    pub mailing_state: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MSTALP`
    Title: Mailing State Abbbreviation
    Description: Mailing State Abbbreviation"#]
    #[serde(rename="ACQ_MSTALP")]
    pub mailing_state_abbbreviation: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="ACQ_MZIP5")]
    pub zip_code_acq_mzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_MZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="ACQ_MZIPREST")]
    pub zip_code_extension_acq_mziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_CLASS`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="ACQ_CLASS")]
    pub tbd_acq_class: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="ACQ_CNTYNAME")]
    pub county: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="ACQ_CNTYNUM")]
    pub tbd_acq_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_INSAGENT1`
    Title: Insurance Fund Membership
    Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    #[serde(rename="ACQ_INSAGENT1")]
    pub insurance_fund_membership: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_INSAGENT2`
    Title: Secondary Insurance Fund
    Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    #[serde(rename="ACQ_INSAGENT2")]
    pub secondary_insurance_fund: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_REGAGENT`
    Title: Acquiring Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="ACQ_REGAGENT")]
    pub acquiring_primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="ACQ_TRUST")]
    pub trust_power: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQ_LATITUDE`
    Title: Location Address Latitude
    Description: Surviving Location Address Latitude"#]
    #[serde(rename="ACQ_LATITUDE")]
    pub location_address_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `ACQ_LONGITUDE`
    Title: Location Address Latitude
    Description: Surviving Location Address Latitude"#]
    #[serde(rename="ACQ_LONGITUDE")]
    pub location_address_latitude_acq_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_INSTNAME`
    Title: Institution name
    Description: The legal name of the institution."#]
    #[serde(rename="OUT_INSTNAME")]
    pub institution_name_out_instname: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="OUT_CERT")]
    pub fdic_certificate_out_cert: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="OUT_CLCODE")]
    pub numeric_code_out_clcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_CHARTER`
    Title: OCC Charter Number
    Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    #[serde(rename="OUT_CHARTER")]
    pub occ_charter_number_out_charter: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_CHARTAGENT`
    Title: Outgoing Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="OUT_CHARTAGENT")]
    pub outgoing_chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_FDICREGION`
    Title: Supervisory Region Number
    Description: A numeric value associated with the name of an FDIC supervisory region"#]
    #[serde(rename="OUT_FDICREGION")]
    pub supervisory_region_number_out_fdicregion: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_FDICREGION_DESC`
    Title: Supervisory Region Description
    Description: A description associated with the name of an FDIC supervisory region"#]
    #[serde(rename="OUT_FDICREGION_DESC")]
    pub supervisory_region_description_out_fdicregion_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="OUT_PADDR")]
    pub physical_street_address_out_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="OUT_PCITY")]
    pub city_out_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_PSTALP`
    Title: State Alpha code
    Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="OUT_PSTALP")]
    pub state_alpha_code_out_pstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="OUT_PZIP5")]
    pub zip_code_out_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="OUT_PZIPREST")]
    pub zip_code_extension_out_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MADDR`
    Title: Mailing Street Address
    Description: Street address at which the institution or one of its branches receives mail."#]
    #[serde(rename="OUT_MADDR")]
    pub mailing_street_address_out_maddr: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="OUT_MCITY")]
    pub city_out_mcity: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MSTATE`
    Title: Mailing State
    Description: Mailing State"#]
    #[serde(rename="OUT_MSTATE")]
    pub mailing_state_out_mstate: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MSTALP`
    Title: Mailing State Abbbreviation
    Description: Mailing State Abbbreviation"#]
    #[serde(rename="OUT_MSTALP")]
    pub mailing_state_abbbreviation_out_mstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="OUT_MZIP5")]
    pub zip_code_out_mzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_MZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="OUT_MZIPREST")]
    pub zip_code_extension_out_mziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_CLASS`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="OUT_CLASS")]
    pub tbd_out_class: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="OUT_CNTYNAME")]
    pub county_out_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="OUT_CNTYNUM")]
    pub tbd_out_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_INSAGENT1`
    Title: Insurance Fund Membership
    Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    #[serde(rename="OUT_INSAGENT1")]
    pub insurance_fund_membership_out_insagent1: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_INSAGENT2`
    Title: Secondary Insurance Fund
    Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    #[serde(rename="OUT_INSAGENT2")]
    pub secondary_insurance_fund_out_insagent2: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_REGAGENT`
    Title: Outgoing Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="OUT_REGAGENT")]
    pub outgoing_primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="OUT_TRUST")]
    pub trust_power_out_trust: Option<String>,

    #[doc = r#"## FDIC Field:: `OUT_LATITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="OUT_LATITUDE")]
    pub location_address_latitude_out_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `OUT_LONGITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="OUT_LONGITUDE")]
    pub location_address_latitude_out_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_CHANGECODE`
    Title: Activity Event Code
    Description: Activity Event Code"#]
    #[serde(rename="SUR_CHANGECODE")]
    pub activity_event_code_sur_changecode: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_CHANGECODE_DESC`
    Title: Activity Event Code Description
    Description: Activity Event Code Description"#]
    #[serde(rename="SUR_CHANGECODE_DESC")]
    pub activity_event_code_description_sur_changecode_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_INSTNAME`
    Title: Institution name
    Description: The legal name of the institution."#]
    #[serde(rename="SUR_INSTNAME")]
    pub institution_name_sur_instname: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="SUR_CERT")]
    pub fdic_certificate_sur_cert: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="SUR_CLCODE")]
    pub numeric_code_sur_clcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_CHARTER`
    Title: OCC Charter Number
    Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    #[serde(rename="SUR_CHARTER")]
    pub occ_charter_number_sur_charter: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_CHARTAGENT`
    Title: Surviving Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="SUR_CHARTAGENT")]
    pub surviving_chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_FDICREGION`
    Title: Supervisory Region Number
    Description: A numeric value associated with the name of an FDIC supervisory region"#]
    #[serde(rename="SUR_FDICREGION")]
    pub supervisory_region_number_sur_fdicregion: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_FDICREGION_DESC`
    Title: Supervisory Region Description
    Description: A description associated with the name of an FDIC supervisory region"#]
    #[serde(rename="SUR_FDICREGION_DESC")]
    pub supervisory_region_description_sur_fdicregion_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_MADDR`
    Title: Mailing Street Address
    Description: Street address at which the institution or one of its branches receives mail."#]
    #[serde(rename="SUR_MADDR")]
    pub mailing_street_address_sur_maddr: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_MCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="SUR_MCITY")]
    pub city_sur_mcity: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_MSTATE`
    Title: Mailing State
    Description: Mailing State"#]
    #[serde(rename="SUR_MSTATE")]
    pub mailing_state_sur_mstate: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_MSTALP`
    Title: Mailing State Abbreviation
    Description: Mailing State Abbreviation"#]
    #[serde(rename="SUR_MSTALP")]
    pub mailing_state_abbreviation: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_MZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="SUR_MZIP5")]
    pub zip_code_sur_mzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="SUR_PZIP5")]
    pub zip_code_sur_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_CLASS`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="SUR_CLASS")]
    pub tbd_sur_class: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="SUR_CNTYNAME")]
    pub county_sur_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="SUR_CNTYNUM")]
    pub tbd_sur_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_INSAGENT1`
    Title: Insurance Fund Membership
    Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    #[serde(rename="SUR_INSAGENT1")]
    pub insurance_fund_membership_sur_insagent1: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_INSAGENT2`
    Title: Secondary Insurance Fund
    Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    #[serde(rename="SUR_INSAGENT2")]
    pub secondary_insurance_fund_sur_insagent2: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="SUR_PADDR")]
    pub physical_street_address_sur_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="SUR_PCITY")]
    pub city_sur_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_PSTALP`
    Title: State Alpha code
    Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="SUR_PSTALP")]
    pub state_alpha_code_sur_pstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="SUR_PZIPREST")]
    pub zip_code_extension_sur_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_REGAGENT`
    Title: Surviving Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="SUR_REGAGENT")]
    pub surviving_primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="SUR_TRUST")]
    pub trust_power_sur_trust: Option<String>,

    #[doc = r#"## FDIC Field:: `SUR_LATITUDE`
    Title: Location Address Latitude
    Description: Surviving Location Address Latitude"#]
    #[serde(rename="SUR_LATITUDE")]
    pub location_address_latitude_sur_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `SUR_LONGITUDE`
    Title: Location Address Latitude
    Description: Surviving Location Address Latitude"#]
    #[serde(rename="SUR_LONGITUDE")]
    pub location_address_latitude_sur_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="FRM_CNTYNUM")]
    pub tbd_frm_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="FRM_PCITY")]
    pub city_frm_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_REGAGENT`
    Title: From Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="FRM_REGAGENT")]
    pub from_primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_PSTALP`
    Title: State Alpha code
    Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="FRM_PSTALP")]
    pub state_alpha_code_frm_pstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="FRM_TRUST")]
    pub trust_power_frm_trust: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="FRM_CLCODE")]
    pub numeric_code_frm_clcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="FRM_PADDR")]
    pub physical_street_address_frm_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_CHARTAGENT`
    Title: From/Before Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="FRM_CHARTAGENT")]
    pub from_before_chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_CLASS`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="FRM_CLASS")]
    pub tbd_frm_class: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="FRM_PZIP5")]
    pub zip_code_frm_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="FRM_PZIPREST")]
    pub zip_code_extension_frm_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_INSTNAME`
    Title: Institution name
    Description: The legal name of the institution."#]
    #[serde(rename="FRM_INSTNAME")]
    pub institution_name_frm_instname: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="FRM_CNTYNAME")]
    pub county_frm_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_CERT`
    Title: Previous FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="FRM_CERT")]
    pub previous_fdic_certificate: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="FRM_OFF_CNTYNAME")]
    pub county_frm_off_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="FRM_OFF_CNTYNUM")]
    pub tbd_frm_off_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="FRM_OFF_PADDR")]
    pub physical_street_address_frm_off_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="FRM_OFF_PCITY")]
    pub city_frm_off_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_PSTALP`
    Title: State Alpha code
    Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="FRM_OFF_PSTALP")]
    pub state_alpha_code_frm_off_pstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="FRM_OFF_PZIP5")]
    pub zip_code_frm_off_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="FRM_OFF_PZIPREST")]
    pub zip_code_extension_frm_off_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_SERVTYPE`
    Title: Service Type
    Description: Service Type"#]
    #[serde(rename="FRM_OFF_SERVTYPE")]
    pub service_type: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_SERVTYPE_DESC`
    Title: Service Type Description
    Description: Service Type Description"#]
    #[serde(rename="FRM_OFF_SERVTYPE_DESC")]
    pub service_type_description: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_STATE`
    Title: Office State
    Description: Office State"#]
    #[serde(rename="FRM_OFF_STATE")]
    pub office_state: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_NAME`
    Title: Office Name
    Description: Name of the branch."#]
    #[serde(rename="FRM_OFF_NAME")]
    pub office_name: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_NUM`
    Title: Branch Number
    Description: The branch's corresponding office number."#]
    #[serde(rename="FRM_OFF_NUM")]
    pub branch_number: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="FRM_OFF_TRUST")]
    pub trust_power_frm_off_trust: Option<String>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="FRM_OFF_CLCODE")]
    pub numeric_code_frm_off_clcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_LATITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="FRM_OFF_LATITUDE")]
    pub location_address_latitude_frm_off_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_OFF_LONGITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="FRM_OFF_LONGITUDE")]
    pub location_address_latitude_frm_off_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_LATITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="FRM_LATITUDE")]
    pub location_address_latitude_frm_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `FRM_LONGITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="FRM_LONGITUDE")]
    pub location_address_latitude_frm_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub fdic_certificate_cert: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSTNAME`
    Title: Institution name
    Description: The legal name of the institution."#]
    #[serde(rename="INSTNAME")]
    pub institution_name_instname: Option<String>,

    #[doc = r#"## FDIC Field:: `CHARTAGENT`
    Title: Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="CHARTAGENT")]
    pub chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `CLCODE`
    Title: Numeric code
    Description: Numeric code which identifies the major and minor categories of an institution."#]
    #[serde(rename="CLCODE")]
    pub numeric_code_clcode: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICREGION`
    Title: Supervisory Region Number
    Description: A numeric value associated with the name of an FDIC supervisory region"#]
    #[serde(rename="FDICREGION")]
    pub supervisory_region_number_fdicregion: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICREGION_DESC`
    Title: Supervisory Region Description
    Description: A description associated with the name of an FDIC supervisory region"#]
    #[serde(rename="FDICREGION_DESC")]
    pub supervisory_region_description_fdicregion_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="CNTYNAME")]
    pub county_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="CNTYNUM")]
    pub tbd_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSAGENT1`
    Title: Insurance Fund Membership
    Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    #[serde(rename="INSAGENT1")]
    pub insurance_fund_membership_insagent1: Option<String>,

    #[doc = r#"## FDIC Field:: `INSAGENT2`
    Title: Secondary Insurance Fund
    Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    #[serde(rename="INSAGENT2")]
    pub secondary_insurance_fund_insagent2: Option<String>,

    #[doc = r#"## FDIC Field:: `MADDR`
    Title: Mailing Street Address
    Description: Street address at which the institution or one of its branches receives mail."#]
    #[serde(rename="MADDR")]
    pub mailing_street_address_maddr: Option<String>,

    #[doc = r#"## FDIC Field:: `MCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="MCITY")]
    pub city_mcity: Option<String>,

    #[doc = r#"## FDIC Field:: `MSTATE`
    Title: Mailing State
    Description: Mailing State"#]
    #[serde(rename="MSTATE")]
    pub mailing_state_mstate: Option<String>,

    #[doc = r#"## FDIC Field:: `MSTALP`
    Title: Mailing State
    Description: Mailing State"#]
    #[serde(rename="MSTALP")]
    pub mailing_state_mstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `MZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="MZIP5")]
    pub zip_code_mzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `MZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="MZIPREST")]
    pub zip_code_extension_mziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="PADDR")]
    pub physical_street_address_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="PZIP5")]
    pub zip_code_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `PSTALP`
    Title: State Alpha code
    Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="PSTALP")]
    pub state_alpha_code_pstalp: Option<String>,

    #[doc = r#"## FDIC Field:: `PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="PZIPREST")]
    pub zip_code_extension_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="PCITY")]
    pub city_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `STATE`
    Title: Physical State
    Description: State in which the institution or one of its branches is physically located."#]
    #[serde(rename="STATE")]
    pub physical_state: Option<String>,

    #[doc = r#"## FDIC Field:: `TRUST`
    Title: Trust Power
    Description: Trust Power"#]
    #[serde(rename="TRUST")]
    pub trust_power_trust: Option<String>,

    #[doc = r#"## FDIC Field:: `REGAGENT`
    Title: Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="REGAGENT")]
    pub primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `SERVTYPE`
    Title: Service Type
    Description: Service Type"#]
    #[serde(rename="SERVTYPE")]
    pub service_type_servtype: Option<f64>,

    #[doc = r#"## FDIC Field:: `SERVTYPE_DESC`
    Title: Service Type Description
    Description: Service Type Description"#]
    #[serde(rename="SERVTYPE_DESC")]
    pub service_type_description_servtype_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_CNTYNAME`
    Title: County
    Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    #[serde(rename="OFF_CNTYNAME")]
    pub county_off_cntyname: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_NUM`
    Title: Branch Number
    Description: The branch's corresponding office number."#]
    #[serde(rename="OFF_NUM")]
    pub branch_number_off_num: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFF_CNTYNUM`
    Title: TBD
    Description: TBD"#]
    #[serde(rename="OFF_CNTYNUM")]
    pub tbd_off_cntynum: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFF_PADDR`
    Title: Physical Street Address
    Description: Street address at which the institution or one of its branches is physically located."#]
    #[serde(rename="OFF_PADDR")]
    pub physical_street_address_off_paddr: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_PSTATE`
    Title: Office State
    Description: Office State"#]
    #[serde(rename="OFF_PSTATE")]
    pub office_state_off_pstate: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_PZIP5`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    #[serde(rename="OFF_PZIP5")]
    pub zip_code_off_pzip5: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_PZIPREST`
    Title: Zip Code Extension
    Description: Zip Code Extension"#]
    #[serde(rename="OFF_PZIPREST")]
    pub zip_code_extension_off_pziprest: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_NAME`
    Title: Office name
    Description: The legal name of the office."#]
    #[serde(rename="OFF_NAME")]
    pub office_name_off_name: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_PSTALP`
    Title: State
    Description: State in which the institution or one of its branches is physically located."#]
    #[serde(rename="OFF_PSTALP")]
    pub state: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_PCITY`
    Title: City
    Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    #[serde(rename="OFF_PCITY")]
    pub city_off_pcity: Option<String>,

    #[doc = r#"## FDIC Field:: `OFF_SERVTYPE`
    Title: Service Type
    Description: Service Type"#]
    #[serde(rename="OFF_SERVTYPE")]
    pub service_type_off_servtype: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFF_LATITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="OFF_LATITUDE")]
    pub location_address_latitude_off_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFF_LONGITUDE`
    Title: Location Address Latitude
    Description: Location Address Latitude"#]
    #[serde(rename="OFF_LONGITUDE")]
    pub location_address_latitude_off_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFF_SERVTYPE_DESC`
    Title: Service Type Description
    Description: Service Type Description"#]
    #[serde(rename="OFF_SERVTYPE_DESC")]
    pub service_type_description_off_servtype_desc: Option<String>,

    #[doc = r#"## FDIC Field:: `ESTDATE`
    Title: Office Established Date
    Description: Office Established Date"#]
    #[serde(rename="ESTDATE")]
    pub office_established_date: Option<String>,

    #[doc = r#"## FDIC Field:: `ACQDATE`
    Title: Office Acquired Date
    Description: Office Acquired Date"#]
    #[serde(rename="ACQDATE")]
    pub office_acquired_date: Option<String>,

    #[doc = r#"## FDIC Field:: `FI_EFFDATE`
    Title: Financial Institution Effective Date
    Description: Financial Institution Effective Date"#]
    #[serde(rename="FI_EFFDATE")]
    pub financial_institution_effective_date: Option<String>,

    #[doc = r#"## FDIC Field:: `FI_UNINUM`
    Title: FDIC's unique number
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    #[serde(rename="FI_UNINUM")]
    pub fdic_s_unique_number_fi_uninum: Option<f64>,

    #[doc = r#"## FDIC Field:: `ORG_STAT_FLG`
    Title: Organization Status Flag
    Description: Organization Status Flag"#]
    #[serde(rename="ORG_STAT_FLG")]
    pub organization_status_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `LATITUDE`
    Title: Location Address Latitude
    Description: The latitude of the physical address."#]
    #[serde(rename="LATITUDE")]
    pub location_address_latitude_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `LONGITUDE`
    Title: Location Address Latitude
    Description: The longitude of the physical address."#]
    #[serde(rename="LONGITUDE")]
    pub location_address_latitude_longitude: Option<f64>,

}

/// FDIC BankFind API `/history` endpoint handler
/// Get Detail on Structure Change Events
/// Returns details on structure change events
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records returned. All values must be entered in UPPERCASE.
Examples:
* Filter by State
`STATE:\"VIRGINIA\"`  
* Filter all but a specified value  
`!(STATE:\"VIRGINIA\")`  
* Filter by Date Range
`PROCDATE:&#91;2020-01-01 TO 2020-02-01&#93;`
    Example: STATE:\"VIRGINIA\"
 - `search` (String, optional): Flexible text search against institution records
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by Name
`NAME: Island`
* Search by Name (fuzzy match)
`NAME: Iland`
* Search by State
`STATE: VA`
 - `fields` (String, optional): Comma delimited list of fields of failed financial institutions to return. All values must be entered in UPPERCASE.
    Example: INSTNAME,CERT,PCITY,PSTALP,PZIP5
 - `sort_by` (String, optional): Field name by which to sort returned data
    Example: PROCDATE
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
 - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
 - `agg_limit` (u32, optional): The limit on how many aggregated results will be displayed
    Example: 10
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/history",
    params(HistoryParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<HistoryProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "History"
)]
pub async fn history_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<HistoryParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "history",
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = HistoryParameters {
            common: CommonParameters::default(),
            search: None,
            agg_by: None,
            agg_term_fields: None,
            agg_limit: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }
    #[test]
    fn test_properties_struct_serialization() {
        let props = HistoryProperties {
            
            system_transaction_number: None,
            activity_event_code: None,
            activity_event_code_description: None,
            process_date: None,
            effective_date: None,
            effective_date_enddate: None,
            fdic_s_unique_number: None,
            fdic_s_unique_number_of_who_is_acquiring: None,
            fdic_s_unique_number_of_who_is_divesting: None,
            organization_role_code: None,
            report_type: None,
            tbd: None,
            bank_insurance_status: None,
            activity_event_code_acq_changecode: None,
            effective_date_acq_org_eff_dte: None,
            institution_name: None,
            fdic_certificate: None,
            numeric_code: None,
            occ_charter_number: None,
            acquiring_chartering_agency: None,
            supervisory_region_number: None,
            supervisory_region_description: None,
            physical_street_address: None,
            city: None,
            state_alpha_code: None,
            zip_code: None,
            zip_code_extension: None,
            mailing_street_address: None,
            city_acq_mcity: None,
            mailing_state: None,
            mailing_state_abbbreviation: None,
            zip_code_acq_mzip5: None,
            zip_code_extension_acq_mziprest: None,
            tbd_acq_class: None,
            county: None,
            tbd_acq_cntynum: None,
            insurance_fund_membership: None,
            secondary_insurance_fund: None,
            acquiring_primary_regulator: None,
            trust_power: None,
            location_address_latitude: None,
            location_address_latitude_acq_longitude: None,
            institution_name_out_instname: None,
            fdic_certificate_out_cert: None,
            numeric_code_out_clcode: None,
            occ_charter_number_out_charter: None,
            outgoing_chartering_agency: None,
            supervisory_region_number_out_fdicregion: None,
            supervisory_region_description_out_fdicregion_desc: None,
            physical_street_address_out_paddr: None,
            city_out_pcity: None,
            state_alpha_code_out_pstalp: None,
            zip_code_out_pzip5: None,
            zip_code_extension_out_pziprest: None,
            mailing_street_address_out_maddr: None,
            city_out_mcity: None,
            mailing_state_out_mstate: None,
            mailing_state_abbbreviation_out_mstalp: None,
            zip_code_out_mzip5: None,
            zip_code_extension_out_mziprest: None,
            tbd_out_class: None,
            county_out_cntyname: None,
            tbd_out_cntynum: None,
            insurance_fund_membership_out_insagent1: None,
            secondary_insurance_fund_out_insagent2: None,
            outgoing_primary_regulator: None,
            trust_power_out_trust: None,
            location_address_latitude_out_latitude: None,
            location_address_latitude_out_longitude: None,
            activity_event_code_sur_changecode: None,
            activity_event_code_description_sur_changecode_desc: None,
            institution_name_sur_instname: None,
            fdic_certificate_sur_cert: None,
            numeric_code_sur_clcode: None,
            occ_charter_number_sur_charter: None,
            surviving_chartering_agency: None,
            supervisory_region_number_sur_fdicregion: None,
            supervisory_region_description_sur_fdicregion_desc: None,
            mailing_street_address_sur_maddr: None,
            city_sur_mcity: None,
            mailing_state_sur_mstate: None,
            mailing_state_abbreviation: None,
            zip_code_sur_mzip5: None,
            zip_code_sur_pzip5: None,
            tbd_sur_class: None,
            county_sur_cntyname: None,
            tbd_sur_cntynum: None,
            insurance_fund_membership_sur_insagent1: None,
            secondary_insurance_fund_sur_insagent2: None,
            physical_street_address_sur_paddr: None,
            city_sur_pcity: None,
            state_alpha_code_sur_pstalp: None,
            zip_code_extension_sur_pziprest: None,
            surviving_primary_regulator: None,
            trust_power_sur_trust: None,
            location_address_latitude_sur_latitude: None,
            location_address_latitude_sur_longitude: None,
            tbd_frm_cntynum: None,
            city_frm_pcity: None,
            from_primary_regulator: None,
            state_alpha_code_frm_pstalp: None,
            trust_power_frm_trust: None,
            numeric_code_frm_clcode: None,
            physical_street_address_frm_paddr: None,
            from_before_chartering_agency: None,
            tbd_frm_class: None,
            zip_code_frm_pzip5: None,
            zip_code_extension_frm_pziprest: None,
            institution_name_frm_instname: None,
            county_frm_cntyname: None,
            previous_fdic_certificate: None,
            county_frm_off_cntyname: None,
            tbd_frm_off_cntynum: None,
            physical_street_address_frm_off_paddr: None,
            city_frm_off_pcity: None,
            state_alpha_code_frm_off_pstalp: None,
            zip_code_frm_off_pzip5: None,
            zip_code_extension_frm_off_pziprest: None,
            service_type: None,
            service_type_description: None,
            office_state: None,
            office_name: None,
            branch_number: None,
            trust_power_frm_off_trust: None,
            numeric_code_frm_off_clcode: None,
            location_address_latitude_frm_off_latitude: None,
            location_address_latitude_frm_off_longitude: None,
            location_address_latitude_frm_latitude: None,
            location_address_latitude_frm_longitude: None,
            fdic_certificate_cert: None,
            institution_name_instname: None,
            chartering_agency: None,
            numeric_code_clcode: None,
            supervisory_region_number_fdicregion: None,
            supervisory_region_description_fdicregion_desc: None,
            county_cntyname: None,
            tbd_cntynum: None,
            insurance_fund_membership_insagent1: None,
            secondary_insurance_fund_insagent2: None,
            mailing_street_address_maddr: None,
            city_mcity: None,
            mailing_state_mstate: None,
            mailing_state_mstalp: None,
            zip_code_mzip5: None,
            zip_code_extension_mziprest: None,
            physical_street_address_paddr: None,
            zip_code_pzip5: None,
            state_alpha_code_pstalp: None,
            zip_code_extension_pziprest: None,
            city_pcity: None,
            physical_state: None,
            trust_power_trust: None,
            primary_regulator: None,
            service_type_servtype: None,
            service_type_description_servtype_desc: None,
            county_off_cntyname: None,
            branch_number_off_num: None,
            tbd_off_cntynum: None,
            physical_street_address_off_paddr: None,
            office_state_off_pstate: None,
            zip_code_off_pzip5: None,
            zip_code_extension_off_pziprest: None,
            office_name_off_name: None,
            state: None,
            city_off_pcity: None,
            service_type_off_servtype: None,
            location_address_latitude_off_latitude: None,
            location_address_latitude_off_longitude: None,
            service_type_description_off_servtype_desc: None,
            office_established_date: None,
            office_acquired_date: None,
            financial_institution_effective_date: None,
            fdic_s_unique_number_fi_uninum: None,
            organization_status_flag: None,
            location_address_latitude_latitude: None,
            location_address_latitude_longitude: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
