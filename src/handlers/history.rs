//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/history` endpoint.

// Internal imports (std, crate)
use crate::common::{CommonParameters, FdicEndpoint, QueryParameters, get_fdic_bank_find_mcp_response};
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::model::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Auto-generated parameters struct for `/history` endpoint.
/// Spec: history_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
pub struct HistoryParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[schemars(description = r#"Flexible text search against institution records
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by Name
`NAME: Island`
* Search by Name (fuzzy match)
`NAME: Iland`
* Search by State
`STATE: VA`"#)]
    pub search: Option<String>,
    #[schemars(description = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_by: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#)]
    pub agg_term_fields: Option<String>,
    #[schemars(description = r#"The limit on how many aggregated results will be displayed"#)]
    pub agg_limit: Option<i32>,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for HistoryParameters {
    fn name() -> &'static str {
        "history"
    }
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct HistoryProperties {
    #[doc = r#"Title: System Transaction Number"#]
    #[doc = r#"Description: System Transaction Number"#]
    pub transnum: Option<f32>,

    #[doc = r#"Title: Activity Event Code"#]
    #[doc = r#"Description: Activity Event Code"#]
    pub changecode: Option<f32>,

    #[doc = r#"Title: Activity Event Code Description (Search-Eligible)"#]
    #[doc = r#"Description: Activity Event Code Description This field can be used for search and filtering."#]
    pub changecode_desc: Option<String>,

    #[doc = r#"Title: Process Date"#]
    #[doc = r#"Description: A date indicating when an institution's change/event is processed."#]
    pub procdate: Option<String>,

    #[doc = r#"Title: Effective Date"#]
    #[doc = r#"Description: Effective Date"#]
    pub effdate: Option<String>,

    #[doc = r#"Title: Effective Date"#]
    #[doc = r#"Description: Effective Date"#]
    pub enddate: Option<String>,

    #[doc = r#"Title: FDIC's unique number"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    pub uninum: Option<f32>,

    #[doc = r#"Title: FDIC's unique number of who is Acquiring"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps to the main office for  the acquiring Institution in a merger, acquisition, etc."#]
    pub acq_uninum: Option<f32>,

    #[doc = r#"Title: FDIC's unique number of who is Divesting"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps to the main office for  the divesting Institution in a merger, acquisition, etc."#]
    pub out_uninum: Option<f32>,

    #[doc = r#"Title: Organization Role Code"#]
    #[doc = r#"Description: Codes include FI (Financial Institution), BR (Branch), and PA"#]
    pub org_role_cde: Option<String>,

    #[doc = r#"Title: Report Type"#]
    #[doc = r#"Description: Type of Report"#]
    pub report_type: Option<f32>,

    #[doc = r#"Title: TBD (Search-Eligible)"#]
    #[doc = r#"Description: TBD This field can be used for search and filtering."#]
    pub class: Option<String>,

    #[doc = r#"Title: Bank Insurance Status"#]
    #[doc = r#"Description: Bank Insurance Status"#]
    pub bank_insured: Option<String>,

    #[doc = r#"Title: Activity Event Code"#]
    #[doc = r#"Description: Activity Event Code"#]
    pub acq_changecode: Option<f32>,

    #[doc = r#"Title: Effective Date"#]
    #[doc = r#"Description: Acquiring Institution's Effective Date"#]
    pub acq_org_eff_dte: Option<String>,

    #[doc = r#"Title: Institution name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the institution. This field can be used for search and filtering."#]
    pub acq_instname: Option<String>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub acq_cert: Option<f32>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub acq_clcode: Option<f32>,

    #[doc = r#"Title: OCC Charter Number"#]
    #[doc = r#"Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    pub acq_charter: Option<f32>,

    #[doc = r#"Title: Acquiring Chartering Agency"#]
    #[doc = r#"Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    pub acq_chartagent: Option<String>,

    #[doc = r#"Title: Supervisory Region Number"#]
    #[doc = r#"Description: A numeric value associated with the name of an FDIC supervisory region"#]
    pub acq_fdicregion: Option<f32>,

    #[doc = r#"Title: Supervisory Region Description"#]
    #[doc = r#"Description: A description associated with the name of an FDIC supervisory region"#]
    pub acq_fdicregion_desc: Option<String>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub acq_paddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub acq_pcity: Option<String>,

    #[doc = r#"Title: State Alpha code (Search-Eligible)"#]
    #[doc = r#"Description: State in which the  acquiring institution's main office or one if its branches are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands. This field can be used for search and filtering."#]
    pub acq_pstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub acq_pzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub acq_pziprest: Option<String>,

    #[doc = r#"Title: Mailing Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches receives mail."#]
    pub acq_maddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub acq_mcity: Option<String>,

    #[doc = r#"Title: Mailing State"#]
    #[doc = r#"Description: Mailing State"#]
    pub acq_mstate: Option<String>,

    #[doc = r#"Title: Mailing State Abbbreviation"#]
    #[doc = r#"Description: Mailing State Abbbreviation"#]
    pub acq_mstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub acq_mzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub acq_mziprest: Option<String>,

    #[doc = r#"Title: TBD (Search-Eligible)"#]
    #[doc = r#"Description: TBD This field can be used for search and filtering."#]
    pub acq_class: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub acq_cntyname: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub acq_cntynum: Option<f32>,

    #[doc = r#"Title: Insurance Fund Membership"#]
    #[doc = r#"Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    pub acq_insagent1: Option<String>,

    #[doc = r#"Title: Secondary Insurance Fund"#]
    #[doc = r#"Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    pub acq_insagent2: Option<String>,

    #[doc = r#"Title: Acquiring Primary Regulator (Search-Eligible)"#]
    #[doc = r#"Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision. This field can be used for search and filtering."#]
    pub acq_regagent: Option<String>,

    #[doc = r#"Title: Trust Power"#]
    #[doc = r#"Description: Trust Power"#]
    pub acq_trust: Option<String>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Surviving Location Address Latitude"#]
    pub acq_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Surviving Location Address Latitude"#]
    pub acq_longitude: Option<f32>,

    #[doc = r#"Title: Institution name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the institution. This field can be used for search and filtering."#]
    pub out_instname: Option<String>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub out_cert: Option<f32>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub out_clcode: Option<f32>,

    #[doc = r#"Title: OCC Charter Number"#]
    #[doc = r#"Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    pub out_charter: Option<f32>,

    #[doc = r#"Title: Outgoing Chartering Agency"#]
    #[doc = r#"Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    pub out_chartagent: Option<String>,

    #[doc = r#"Title: Supervisory Region Number"#]
    #[doc = r#"Description: A numeric value associated with the name of an FDIC supervisory region"#]
    pub out_fdicregion: Option<f32>,

    #[doc = r#"Title: Supervisory Region Description"#]
    #[doc = r#"Description: A description associated with the name of an FDIC supervisory region"#]
    pub out_fdicregion_desc: Option<String>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub out_paddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub out_pcity: Option<String>,

    #[doc = r#"Title: State Alpha code"#]
    #[doc = r#"Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    pub out_pstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub out_pzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub out_pziprest: Option<String>,

    #[doc = r#"Title: Mailing Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches receives mail."#]
    pub out_maddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub out_mcity: Option<String>,

    #[doc = r#"Title: Mailing State"#]
    #[doc = r#"Description: Mailing State"#]
    pub out_mstate: Option<String>,

    #[doc = r#"Title: Mailing State Abbbreviation"#]
    #[doc = r#"Description: Mailing State Abbbreviation"#]
    pub out_mstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub out_mzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub out_mziprest: Option<String>,

    #[doc = r#"Title: TBD (Search-Eligible)"#]
    #[doc = r#"Description: TBD This field can be used for search and filtering."#]
    pub out_class: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub out_cntyname: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub out_cntynum: Option<f32>,

    #[doc = r#"Title: Insurance Fund Membership"#]
    #[doc = r#"Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    pub out_insagent1: Option<String>,

    #[doc = r#"Title: Secondary Insurance Fund"#]
    #[doc = r#"Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    pub out_insagent2: Option<String>,

    #[doc = r#"Title: Outgoing Primary Regulator (Search-Eligible)"#]
    #[doc = r#"Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision. This field can be used for search and filtering."#]
    pub out_regagent: Option<String>,

    #[doc = r#"Title: Trust Power"#]
    #[doc = r#"Description: Trust Power"#]
    pub out_trust: Option<String>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub out_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub out_longitude: Option<f32>,

    #[doc = r#"Title: Activity Event Code"#]
    #[doc = r#"Description: Activity Event Code"#]
    pub sur_changecode: Option<f32>,

    #[doc = r#"Title: Activity Event Code Description (Search-Eligible)"#]
    #[doc = r#"Description: Activity Event Code Description This field can be used for search and filtering."#]
    pub sur_changecode_desc: Option<String>,

    #[doc = r#"Title: Institution name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the institution. This field can be used for search and filtering."#]
    pub sur_instname: Option<String>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub sur_cert: Option<f32>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub sur_clcode: Option<f32>,

    #[doc = r#"Title: OCC Charter Number"#]
    #[doc = r#"Description: A unique number assigned by the Office of the Comptroller of the Currency (OCC) used to identify institutions that it has chartered and regulates (i.e. national  banks)."#]
    pub sur_charter: Option<f32>,

    #[doc = r#"Title: Surviving Chartering Agency"#]
    #[doc = r#"Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    pub sur_chartagent: Option<String>,

    #[doc = r#"Title: Supervisory Region Number"#]
    #[doc = r#"Description: A numeric value associated with the name of an FDIC supervisory region"#]
    pub sur_fdicregion: Option<f32>,

    #[doc = r#"Title: Supervisory Region Description"#]
    #[doc = r#"Description: A description associated with the name of an FDIC supervisory region"#]
    pub sur_fdicregion_desc: Option<String>,

    #[doc = r#"Title: Mailing Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches receives mail."#]
    pub sur_maddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub sur_mcity: Option<String>,

    #[doc = r#"Title: Mailing State"#]
    #[doc = r#"Description: Mailing State"#]
    pub sur_mstate: Option<String>,

    #[doc = r#"Title: Mailing State Abbreviation"#]
    #[doc = r#"Description: Mailing State Abbreviation"#]
    pub sur_mstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub sur_mzip5: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub sur_pzip5: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub sur_class: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub sur_cntyname: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub sur_cntynum: Option<f32>,

    #[doc = r#"Title: Insurance Fund Membership"#]
    #[doc = r#"Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    pub sur_insagent1: Option<String>,

    #[doc = r#"Title: Secondary Insurance Fund"#]
    #[doc = r#"Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    pub sur_insagent2: Option<String>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub sur_paddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub sur_pcity: Option<String>,

    #[doc = r#"Title: State Alpha code (Search-Eligible)"#]
    #[doc = r#"Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands. This field can be used for search and filtering."#]
    pub sur_pstalp: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub sur_pziprest: Option<String>,

    #[doc = r#"Title: Surviving Primary Regulator"#]
    #[doc = r#"Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    pub sur_regagent: Option<String>,

    #[doc = r#"Title: Trust Power"#]
    #[doc = r#"Description: Trust Power"#]
    pub sur_trust: Option<String>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Surviving Location Address Latitude"#]
    pub sur_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Surviving Location Address Latitude"#]
    pub sur_longitude: Option<f32>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub frm_cntynum: Option<f32>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub frm_pcity: Option<String>,

    #[doc = r#"Title: From Primary Regulator (Search-Eligible)"#]
    #[doc = r#"Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision. This field can be used for search and filtering."#]
    pub frm_regagent: Option<String>,

    #[doc = r#"Title: State Alpha code"#]
    #[doc = r#"Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    pub frm_pstalp: Option<String>,

    #[doc = r#"Title: Trust Power (Search-Eligible)"#]
    #[doc = r#"Description: Trust Power This field can be used for search and filtering."#]
    pub frm_trust: Option<String>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub frm_clcode: Option<f32>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub frm_paddr: Option<String>,

    #[doc = r#"Title: From/Before Chartering Agency (Search-Eligible)"#]
    #[doc = r#"Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS This field can be used for search and filtering."#]
    pub frm_chartagent: Option<String>,

    #[doc = r#"Title: TBD (Search-Eligible)"#]
    #[doc = r#"Description: TBD This field can be used for search and filtering."#]
    pub frm_class: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub frm_pzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub frm_pziprest: Option<String>,

    #[doc = r#"Title: Institution name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the institution. This field can be used for search and filtering."#]
    pub frm_instname: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub frm_cntyname: Option<String>,

    #[doc = r#"Title: Previous FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub frm_cert: Option<f32>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub frm_off_cntyname: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub frm_off_cntynum: Option<f32>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub frm_off_paddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub frm_off_pcity: Option<String>,

    #[doc = r#"Title: State Alpha code"#]
    #[doc = r#"Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    pub frm_off_pstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub frm_off_pzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub frm_off_pziprest: Option<String>,

    #[doc = r#"Title: Service Type"#]
    #[doc = r#"Description: Service Type"#]
    pub frm_off_servtype: Option<f32>,

    #[doc = r#"Title: Service Type Description"#]
    #[doc = r#"Description: Service Type Description"#]
    pub frm_off_servtype_desc: Option<String>,

    #[doc = r#"Title: Office State"#]
    #[doc = r#"Description: Office State"#]
    pub frm_off_state: Option<String>,

    #[doc = r#"Title: Office Name (Search-Eligible)"#]
    #[doc = r#"Description: Name of the branch. This field can be used for search and filtering."#]
    pub frm_off_name: Option<String>,

    #[doc = r#"Title: Branch Number"#]
    #[doc = r#"Description: The branch's corresponding office number."#]
    pub frm_off_num: Option<String>,

    #[doc = r#"Title: Trust Power"#]
    #[doc = r#"Description: Trust Power"#]
    pub frm_off_trust: Option<String>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub frm_off_clcode: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub frm_off_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub frm_off_longitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub frm_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub frm_longitude: Option<f32>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    pub cert: Option<f32>,

    #[doc = r#"Title: Institution name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the institution. This field can be used for search and filtering."#]
    pub instname: Option<String>,

    #[doc = r#"Title: Chartering Agency (Search-Eligible)"#]
    #[doc = r#"Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS This field can be used for search and filtering."#]
    pub chartagent: Option<String>,

    #[doc = r#"Title: Numeric code"#]
    #[doc = r#"Description: Numeric code which identifies the major and minor categories of an institution."#]
    pub clcode: Option<f32>,

    #[doc = r#"Title: Supervisory Region Number"#]
    #[doc = r#"Description: A numeric value associated with the name of an FDIC supervisory region"#]
    pub fdicregion: Option<f32>,

    #[doc = r#"Title: Supervisory Region Description"#]
    #[doc = r#"Description: A description associated with the name of an FDIC supervisory region"#]
    pub fdicregion_desc: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub cntyname: Option<String>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub cntynum: Option<f32>,

    #[doc = r#"Title: Insurance Fund Membership"#]
    #[doc = r#"Description: Deposit Insurance Fund (DIF), Bank Insurance Fund (BIF), Savings Association Insurance Fund (SAIF)"#]
    pub insagent1: Option<String>,

    #[doc = r#"Title: Secondary Insurance Fund"#]
    #[doc = r#"Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    pub insagent2: Option<String>,

    #[doc = r#"Title: Mailing Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches receives mail."#]
    pub maddr: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub mcity: Option<String>,

    #[doc = r#"Title: Mailing State"#]
    #[doc = r#"Description: Mailing State"#]
    pub mstate: Option<String>,

    #[doc = r#"Title: Mailing State"#]
    #[doc = r#"Description: Mailing State"#]
    pub mstalp: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub mzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub mziprest: Option<String>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub paddr: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub pzip5: Option<String>,

    #[doc = r#"Title: State Alpha code (Search-Eligible)"#]
    #[doc = r#"Description: State in which the the headquarters are physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands. This field can be used for search and filtering."#]
    pub pstalp: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub pziprest: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub pcity: Option<String>,

    #[doc = r#"Title: Physical State"#]
    #[doc = r#"Description: State in which the institution or one of its branches is physically located."#]
    pub state: Option<String>,

    #[doc = r#"Title: Trust Power (Search-Eligible)"#]
    #[doc = r#"Description: Trust Power This field can be used for search and filtering."#]
    pub trust: Option<String>,

    #[doc = r#"Title: Primary Regulator (Search-Eligible)"#]
    #[doc = r#"Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision. This field can be used for search and filtering."#]
    pub regagent: Option<String>,

    #[doc = r#"Title: Service Type"#]
    #[doc = r#"Description: Service Type"#]
    pub servtype: Option<f32>,

    #[doc = r#"Title: Service Type Description"#]
    #[doc = r#"Description: Service Type Description"#]
    pub servtype_desc: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: County where the institution is physically located (abbreviated if the county name exceeds 16 characters)."#]
    pub off_cntyname: Option<String>,

    #[doc = r#"Title: Branch Number"#]
    #[doc = r#"Description: The branch's corresponding office number."#]
    pub off_num: Option<f32>,

    #[doc = r#"Title: TBD"#]
    #[doc = r#"Description: TBD"#]
    pub off_cntynum: Option<f32>,

    #[doc = r#"Title: Physical Street Address"#]
    #[doc = r#"Description: Street address at which the institution or one of its branches is physically located."#]
    pub off_paddr: Option<String>,

    #[doc = r#"Title: Office State"#]
    #[doc = r#"Description: Office State"#]
    pub off_pstate: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or its branch office."#]
    pub off_pzip5: Option<String>,

    #[doc = r#"Title: Zip Code Extension"#]
    #[doc = r#"Description: Zip Code Extension"#]
    pub off_pziprest: Option<String>,

    #[doc = r#"Title: Office name (Search-Eligible)"#]
    #[doc = r#"Description: The legal name of the office. This field can be used for search and filtering."#]
    pub off_name: Option<String>,

    #[doc = r#"Title: State (Search-Eligible)"#]
    #[doc = r#"Description: State in which the institution or one of its branches is physically located. This field can be used for search and filtering."#]
    pub off_pstalp: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: City in which an institution's headquarters or one of its branches is physically located. Either the entire name or part of the name of a specific city may be entered to produce an Institution List."#]
    pub off_pcity: Option<String>,

    #[doc = r#"Title: Service Type"#]
    #[doc = r#"Description: Service Type"#]
    pub off_servtype: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub off_latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: Location Address Latitude"#]
    pub off_longitude: Option<f32>,

    #[doc = r#"Title: Service Type Description (Search-Eligible)"#]
    #[doc = r#"Description: Service Type Description This field can be used for search and filtering."#]
    pub off_servtype_desc: Option<String>,

    #[doc = r#"Title: Office Established Date"#]
    #[doc = r#"Description: Office Established Date"#]
    pub estdate: Option<String>,

    #[doc = r#"Title: Office Acquired Date"#]
    #[doc = r#"Description: Office Acquired Date"#]
    pub acqdate: Option<String>,

    #[doc = r#"Title: Financial Institution Effective Date"#]
    #[doc = r#"Description: Financial Institution Effective Date"#]
    pub fi_effdate: Option<String>,

    #[doc = r#"Title: FDIC's unique number"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    pub fi_uninum: Option<f32>,

    #[doc = r#"Title: Organization Status Flag"#]
    #[doc = r#"Description: Organization Status Flag"#]
    pub org_stat_flg: Option<String>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: The latitude of the physical address."#]
    pub latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: The longitude of the physical address."#]
    pub longitude: Option<f32>,

}

/// Auto-generated response envelope struct for `/history` endpoint.
/// Spec: history_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryResponse {
    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    pub data: Option<String>,

}

/// FDIC BankFind API `/history` endpoint handler
/// Get Detail on Structure Change Events
/// Returns details on structure change events
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter criteria that refines the records returned. All values must be entered in UPPERCASE.
Examples:
* Filter by State
`STATE:\"VIRGINIA\"`  
* Filter all but a specified value  
`!(STATE:\"VIRGINIA\")`  
* Filter by Date Range
`PROCDATE:&#91;2020-01-01 TO 2020-02-01&#93;`
STATE:\"VIRGINIA\" - `search` (String, optional): Flexible text search against institution records
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by Name
`NAME: Island`
* Search by Name (fuzzy match)
`NAME: Iland`
* Search by State
`STATE: VA` - `fields` (String, optional): Comma delimited list of fields of failed financial institutions to return. All values must be entered in UPPERCASE.
INSTNAME,CERT,PCITY,PSTALP,PZIP5 - `sort_by` (String, optional): Field name by which to sort returned data
PROCDATE - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE. - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE. - `agg_limit` (i32, optional): The limit on how many aggregated results will be displayed - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /history
Parameters: HistoryParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: History"#]
pub async fn history_handler(
    config: &FdicApiConfig,
    params: &HistoryParameters,
) -> Result<CallToolResult, rmcp::Error> {
   
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "history",
        method = "GET",
        path = "/history",
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
            transnum: None,
            changecode: None,
            changecode_desc: None,
            procdate: None,
            effdate: None,
            enddate: None,
            uninum: None,
            acq_uninum: None,
            out_uninum: None,
            org_role_cde: None,
            report_type: None,
            class: None,
            bank_insured: None,
            acq_changecode: None,
            acq_org_eff_dte: None,
            acq_instname: None,
            acq_cert: None,
            acq_clcode: None,
            acq_charter: None,
            acq_chartagent: None,
            acq_fdicregion: None,
            acq_fdicregion_desc: None,
            acq_paddr: None,
            acq_pcity: None,
            acq_pstalp: None,
            acq_pzip5: None,
            acq_pziprest: None,
            acq_maddr: None,
            acq_mcity: None,
            acq_mstate: None,
            acq_mstalp: None,
            acq_mzip5: None,
            acq_mziprest: None,
            acq_class: None,
            acq_cntyname: None,
            acq_cntynum: None,
            acq_insagent1: None,
            acq_insagent2: None,
            acq_regagent: None,
            acq_trust: None,
            acq_latitude: None,
            acq_longitude: None,
            out_instname: None,
            out_cert: None,
            out_clcode: None,
            out_charter: None,
            out_chartagent: None,
            out_fdicregion: None,
            out_fdicregion_desc: None,
            out_paddr: None,
            out_pcity: None,
            out_pstalp: None,
            out_pzip5: None,
            out_pziprest: None,
            out_maddr: None,
            out_mcity: None,
            out_mstate: None,
            out_mstalp: None,
            out_mzip5: None,
            out_mziprest: None,
            out_class: None,
            out_cntyname: None,
            out_cntynum: None,
            out_insagent1: None,
            out_insagent2: None,
            out_regagent: None,
            out_trust: None,
            out_latitude: None,
            out_longitude: None,
            sur_changecode: None,
            sur_changecode_desc: None,
            sur_instname: None,
            sur_cert: None,
            sur_clcode: None,
            sur_charter: None,
            sur_chartagent: None,
            sur_fdicregion: None,
            sur_fdicregion_desc: None,
            sur_maddr: None,
            sur_mcity: None,
            sur_mstate: None,
            sur_mstalp: None,
            sur_mzip5: None,
            sur_pzip5: None,
            sur_class: None,
            sur_cntyname: None,
            sur_cntynum: None,
            sur_insagent1: None,
            sur_insagent2: None,
            sur_paddr: None,
            sur_pcity: None,
            sur_pstalp: None,
            sur_pziprest: None,
            sur_regagent: None,
            sur_trust: None,
            sur_latitude: None,
            sur_longitude: None,
            frm_cntynum: None,
            frm_pcity: None,
            frm_regagent: None,
            frm_pstalp: None,
            frm_trust: None,
            frm_clcode: None,
            frm_paddr: None,
            frm_chartagent: None,
            frm_class: None,
            frm_pzip5: None,
            frm_pziprest: None,
            frm_instname: None,
            frm_cntyname: None,
            frm_cert: None,
            frm_off_cntyname: None,
            frm_off_cntynum: None,
            frm_off_paddr: None,
            frm_off_pcity: None,
            frm_off_pstalp: None,
            frm_off_pzip5: None,
            frm_off_pziprest: None,
            frm_off_servtype: None,
            frm_off_servtype_desc: None,
            frm_off_state: None,
            frm_off_name: None,
            frm_off_num: None,
            frm_off_trust: None,
            frm_off_clcode: None,
            frm_off_latitude: None,
            frm_off_longitude: None,
            frm_latitude: None,
            frm_longitude: None,
            cert: None,
            instname: None,
            chartagent: None,
            clcode: None,
            fdicregion: None,
            fdicregion_desc: None,
            cntyname: None,
            cntynum: None,
            insagent1: None,
            insagent2: None,
            maddr: None,
            mcity: None,
            mstate: None,
            mstalp: None,
            mzip5: None,
            mziprest: None,
            paddr: None,
            pzip5: None,
            pstalp: None,
            pziprest: None,
            pcity: None,
            state: None,
            trust: None,
            regagent: None,
            servtype: None,
            servtype_desc: None,
            off_cntyname: None,
            off_num: None,
            off_cntynum: None,
            off_paddr: None,
            off_pstate: None,
            off_pzip5: None,
            off_pziprest: None,
            off_name: None,
            off_pstalp: None,
            off_pcity: None,
            off_servtype: None,
            off_latitude: None,
            off_longitude: None,
            off_servtype_desc: None,
            estdate: None,
            acqdate: None,
            fi_effdate: None,
            fi_uninum: None,
            org_stat_flg: None,
            latitude: None,
            longitude: None,
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
