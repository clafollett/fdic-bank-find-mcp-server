//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/institutions` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/institutions` endpoint.
/// Spec: institution_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct InstitutionsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"Flexible text search against institution records - currently only supporting name search. 
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by name
`NAME: Island`
* Search by name (fuzzy match)
`NAME: Iland`"#]
    #[param(rename = "search")]
    pub search: Option<String>,
}

// Implement QueryParameters for generic handler
impl QueryParameters for InstitutionsParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ACTIVE",
        "ADDRESS",
        "ASSET",
        "BKCLASS",
        "CB",
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
        "CERTCONS",
        "CFPBEFFDTE",
        "CFPBENDDTE",
        "CFPBFLAG",
        "PRIORNAME1",
        "PRIORNAME2",
        "PRIORNAME3",
        "PRIORNAME4",
        "PRIORNAME5",
        "PRIORNAME6",
        "PRIORNAME7",
        "PRIORNAME8",
        "PRIORNAME9",
        "PRIORNAME10",
        "CHANGEC1",
        "CHANGEC2",
        "CHANGEC3",
        "CHANGEC4",
        "CHANGEC5",
        "CHANGEC6",
        "CHANGEC7",
        "CHANGEC8",
        "CHANGEC9",
        "CHANGEC10",
        "CHANGEC11",
        "CHANGEC12",
        "CHANGEC13",
        "CHANGEC14",
        "CHANGEC15",
        "CHARTER",
        "CHRTAGNT",
        "CITY",
        "CITYHCR",
        "CLCODE",
        "CMSA_NO",
        "CMSA",
        "CONSERVE",
        "COUNTY",
        "CSA",
        "CSA_NO",
        "CSA_FLG",
        "DATEUPDT",
        "DENOVO",
        "DEP",
        "DEPDOM",
        "DOCKET",
        "EFFDATE",
        "ENDEFYMD",
        "EQ",
        "ESTYMD",
        "FDICDBS",
        "FDICREGN",
        "FDICSUPV",
        "FED",
        "FED_RSSD",
        "FEDCHRTR",
        "FLDOFF",
        "FORM31",
        "HCTMULT",
        "IBA",
        "INACTIVE",
        "INSAGNT1",
        "INSAGNT2",
        "INSBIF",
        "INSCOML",
        "INSDATE",
        "INSDROPDATE_RAW",
        "INSDROPDATE",
        "INSDIF",
        "INSFDIC",
        "INSSAIF",
        "INSSAVE",
        "INSTAG",
        "INSTCRCD",
        "LATITUDE",
        "LAW_SASSER_FLG",
        "LONGITUDE",
        "MDI_STATUS_CODE",
        "MDI_STATUS_DESC",
        "MSA",
        "MSA_NO",
        "MUTUAL",
        "NAME",
        "NAMEHCR",
        "NETINC",
        "NETINCQ",
        "NEWCERT",
        "OAKAR",
        "OCCDIST",
        "OFFDOM",
        "OFFFOR",
        "OFFICES",
        "OFFOA",
        "OTSDIST",
        "OTSREGNM",
        "PARCERT",
        "PROCDATE",
        "QBPRCOML",
        "REGAGNT",
        "REGAGENT2",
        "REPDTE",
        "RISDATE",
        "ROA",
        "ROAPTX",
        "ROAPTXQ",
        "ROAQ",
        "ROE",
        "ROEQ",
        "RSSDHCR",
        "RUNDATE",
        "SASSER",
        "SPECGRP",
        "SPECGRPN",
        "STALP",
        "STALPHCR",
        "STCHRTR",
        "STCNTY",
        "STNAME",
        "STNUM",
        "SUBCHAPS",
        "SUPRV_FD",
        "TE01N528",
        "TE02N528",
        "TE03N528",
        "TE04N528",
        "TE05N528",
        "TE06N528",
        "TE07N528",
        "TE08N528",
        "TE09N528",
        "TE10N528",
        "TE01N529",
        "TE02N529",
        "TE03N529",
        "TE04N529",
        "TE05N529",
        "TE06N529",
        "TRACT",
        "TRUST",
        "ULTCERT",
        "UNINUM",
        "WEBADDR",
        "ZIP",
    ];

    #[allow(unused_variables)]
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        if let Some(val) = &self.search {
            query.insert("search".to_string(), val.to_string());
        }
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/institutions` endpoint.
/// Spec: institution_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct InstitutionsProperties {
    #[doc = r#"## FDIC Field:: `ACTIVE`
    Title: Institution Status
    Description: A number indicating the status of an institution. 1='Institutions that are currently open and insured by the FDIC'; 0='Institution closed or not insured by FDIC'"#]
    #[serde(rename="ACTIVE")]
    pub institution_status: Option<String>,

    #[doc = r#"## FDIC Field:: `ADDRESS`
    Title: Street Address
    Description: The street address in which an institution or branch office is physically located."#]
    #[serde(rename="ADDRESS")]
    pub street_address: Option<String>,

    #[doc = r#"## FDIC Field:: `ASSET`
    Title: Total assets
    Description: The sum of all assets owned by the institution including cash, loans, securities, bank premises and other assets. This total does not include off-balance-sheet accounts."#]
    #[serde(rename="ASSET")]
    pub total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `BKCLASS`
    Title: Institution Class
    Description: A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#]
    #[serde(rename="BKCLASS")]
    pub institution_class: Option<String>,

    #[doc = r#"## FDIC Field:: `CB`
    Title: Community Bank
    Description: FDIC community banks are identified based on criteria defined in the FDIC Community Banking Study. Using detailed balance sheet and geographic data, the study defines community banks in terms of their traditional relationship banking and limited geographic scope of operations"#]
    #[serde(rename="CB")]
    pub community_bank: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA`
    Title: Core Based Statistical Area Name
    Description: Name of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA")]
    pub core_based_statistical_area_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_DIV`
    Title: Metropolitan Divisions Name
    Description: Name of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_DIV")]
    pub metropolitan_divisions_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_DIV_FLG`
    Title: Metropolitan Divisions Flag
    Description: A flag (1=Yes) indicating member of a Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_DIV_FLG")]
    pub metropolitan_divisions_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_DIV_NO`
    Title: Metropolitan Divisions Number
    Description: Numeric code of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget.s"#]
    #[serde(rename="CBSA_DIV_NO")]
    pub metropolitan_divisions_number: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_METRO`
    Title: Metropolitan Division Number
    Description: Numeric code of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO")]
    pub metropolitan_division_number: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_METRO_FLG`
    Title: Metropolitan Division Flag
    Description: A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO_FLG")]
    pub metropolitan_division_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_METRO_NAME`
    Title: Metropolitan Division Name
    Description: Name of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO_NAME")]
    pub metropolitan_division_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_MICRO_FLG`
    Title: Micropolitan Division Flag
    Description: A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_MICRO_FLG")]
    pub micropolitan_division_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSA_NO`
    Title: Core Based Statistical Areas
    Description: Numeric code of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_NO")]
    pub core_based_statistical_areas: Option<String>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub fdic_certificate: Option<String>,

    #[doc = r#"## FDIC Field:: `CERTCONS`
    Title: Directly owned by another bank (CERT)
    Description: FDIC certificate number of the parent bank or savings institution with which the reported institution’s financial data has been consolidated. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports.  Click on the certificate number to identify the parent bank or thrift."#]
    #[serde(rename="CERTCONS")]
    pub directly_owned_by_another_bank_cert: Option<String>,

    #[doc = r#"## FDIC Field:: `CFPBEFFDTE`
    Title: CFPB Effective Date
    Description: Date the institution began secondary supervision by CFPB"#]
    #[serde(rename="CFPBEFFDTE")]
    pub cfpb_effective_date: Option<String>,

    #[doc = r#"## FDIC Field:: `CFPBENDDTE`
    Title: CFPB End Date
    Description: Date the institution ended supervision by CFPB"#]
    #[serde(rename="CFPBENDDTE")]
    pub cfpb_end_date: Option<String>,

    #[doc = r#"## FDIC Field:: `CFPBFLAG`
    Title: CFPB Flag
    Description: Indicates secondary supervision by CFPB ('0' - not supervised by CFPB, '1'- secondarily supervised by CFPB)"#]
    #[serde(rename="CFPBFLAG")]
    pub cfpb_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME1`
    Title: Previous Name 1
    Description: Previous Name of Institution 1"#]
    #[serde(rename="PRIORNAME1")]
    pub previous_name_1: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME2`
    Title: Previous Name 2
    Description: Previous Name of Institution 2"#]
    #[serde(rename="PRIORNAME2")]
    pub previous_name_2: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME3`
    Title: Previous Name 3
    Description: Previous Name of Institution 3"#]
    #[serde(rename="PRIORNAME3")]
    pub previous_name_3: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME4`
    Title: Previous Name 4
    Description: Previous Name of Institution 4"#]
    #[serde(rename="PRIORNAME4")]
    pub previous_name_4: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME5`
    Title: Previous Name 5
    Description: Previous Name of Institution 5"#]
    #[serde(rename="PRIORNAME5")]
    pub previous_name_5: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME6`
    Title: Previous Name 6
    Description: Previous Name of Institution 6"#]
    #[serde(rename="PRIORNAME6")]
    pub previous_name_6: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME7`
    Title: Previous Name 7
    Description: Previous Name of Institution 7"#]
    #[serde(rename="PRIORNAME7")]
    pub previous_name_7: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME8`
    Title: Previous Name 8
    Description: Previous Name of Institution 8"#]
    #[serde(rename="PRIORNAME8")]
    pub previous_name_8: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME9`
    Title: Previous Name 9
    Description: Previous Name of Institution 9"#]
    #[serde(rename="PRIORNAME9")]
    pub previous_name_9: Option<String>,

    #[doc = r#"## FDIC Field:: `PRIORNAME10`
    Title: Previous Name 10
    Description: Previous Name of Institution 10"#]
    #[serde(rename="PRIORNAME10")]
    pub previous_name_10: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC1`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC1")]
    pub change_code: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC2`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC2")]
    pub change_code_changec2: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC3`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC3")]
    pub change_code_changec3: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC4`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC4")]
    pub change_code_changec4: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC5`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC5")]
    pub change_code_changec5: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC6`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC6")]
    pub change_code_changec6: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC7`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC7")]
    pub change_code_changec7: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC8`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC8")]
    pub change_code_changec8: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC9`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC9")]
    pub change_code_changec9: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC10`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC10")]
    pub change_code_changec10: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC11`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC11")]
    pub change_code_changec11: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC12`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC12")]
    pub change_code_changec12: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC13`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC13")]
    pub change_code_changec13: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC14`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC14")]
    pub change_code_changec14: Option<String>,

    #[doc = r#"## FDIC Field:: `CHANGEC15`
    Title: Change Code
    Description: FDIC code used to signify a structural event relating to an institution."#]
    #[serde(rename="CHANGEC15")]
    pub change_code_changec15: Option<String>,

    #[doc = r#"## FDIC Field:: `CHARTER`
    Title: OCC Charter Number
    Description: The charter number identifying a financial institution supervised by the Office of Comptroller of Currency."#]
    #[serde(rename="CHARTER")]
    pub occ_charter_number: Option<String>,

    #[doc = r#"## FDIC Field:: `CHRTAGNT`
    Title: Chartering Agency
    Description: All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#]
    #[serde(rename="CHRTAGNT")]
    pub chartering_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `CITY`
    Title: City
    Description: The city in which an institution or branch office is physically located."#]
    #[serde(rename="CITY")]
    pub city: Option<String>,

    #[doc = r#"## FDIC Field:: `CITYHCR`
    Title: City of High Holder
    Description: City in which the headquarters of the institution's regulatory high holder are physically located."#]
    #[serde(rename="CITYHCR")]
    pub city_of_high_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `CLCODE`
    Title: Numeric code
    Description: A number that sub-categorizes a major class of institutions. 3 = National bank, Federal Reserve System (FRS) member; 13 = State commercial bank, FRS member; 15 = State savings, co-op, or industrial bank, FRS member; 21 = State commercial bank, not FRS member; 23 = State savings, co-op, or industrial bank, not FRS member; 25 - State mutual commercial bank, not FRS member; 33 = Federal chartered stock savings bank; 34 = Federal chartered mutual saving bank; 35 = State chartered stock savings and loan association; 36 = State chartered mutual savings and loan association; 37 = Federal chartered stock savings and loan association; 38 = Federal chartered mutual savings and loan association; 41 = State chartered stock savings bank; 42 = State chartered mutual savings bank; 43 = Federal chartered stock savings bank (historical); 44 = Federal chartered mutual savings bank (historical); 52 = Insured domestic offices of foreign banks (International Banking Act(IBA)); 50 = OCC chartered nondeposit and/or noninsured trust companies; 51 = Noninsured commercial bank; 52 = Noninsured domestic offices of foreign bank (International Banking Act); 53 = Noninsured industrial bank; 54 = State chartered nondeposit and/or noninsured trust company, not FRS member; 55 = State chartered domestic branches of foreign banks; 56 = OCC chartered domestic branches of foreign banks; 57 = New York investment company; 58 = State chartered nondeposit and/or noninsured trust company, FRS member; 59 = OTS chartered nondeposit and/or noninsured trust company; 61 = Noninsured private bank; 62 = Noninsured loan workout bank, OCC chartered; 63 = Noninsured loan workout bank, state chartered, FRS member; 64 = Noninsured loan workout bank, state chartered, not FRS member; 65 = Other holding company; 71 = Transfer agent; 81 = Noninsured stock savings bank; 82 = Noninsured mutual savings bank; 85 = Noninsured stock savings and loan association; 86 = Noninsured mutual savings and loan association; 89 = Noninsured insurance company; 91 = State chartered credit unions; 92 = Federal chartered credit unions; 93 = Privately insured state credit union."#]
    #[serde(rename="CLCODE")]
    pub numeric_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `CMSA_NO`
    Title: Consolidated Metropolitan Statistical Division Number
    Description: The numeric code given by the US Census Bureau office of Management and Budget that represents the CMSA prior to the year 2000 standards. 1=yes"#]
    #[serde(rename="CMSA_NO")]
    pub consolidated_metropolitan_statistical_division_number: Option<String>,

    #[doc = r#"## FDIC Field:: `CMSA`
    Title: Consolidated Metropolitan Statistical Area
    Description: The Federal Information Processing Standards (FIPS) Consolidated Metropolitan Statistical Area (CMSA) code is a number representing the institution location. CMSA consists of two or more contiguous Metropolitan Statistical Areas (MSA) with a combined population of over 1 Million.  Note: If an institution is not located in a CMSA, the value of the field will be zeroes."#]
    #[serde(rename="CMSA")]
    pub consolidated_metropolitan_statistical_area: Option<String>,

    #[doc = r#"## FDIC Field:: `CONSERVE`
    Title: Conservatorship
    Description: A flag (1=yes;0=no) that indicates if an institution is being operated in government conservatorship."#]
    #[serde(rename="CONSERVE")]
    pub conservatorship: Option<String>,

    #[doc = r#"## FDIC Field:: `COUNTY`
    Title: County
    Description: The county name in which an institution or branch office is physically located."#]
    #[serde(rename="COUNTY")]
    pub county: Option<String>,

    #[doc = r#"## FDIC Field:: `CSA`
    Title: Combined Statistical Area Name
    Description: Name of the Combined Statistical Area (CSA) as defined by the US Census Bureau."#]
    #[serde(rename="CSA")]
    pub combined_statistical_area_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CSA_NO`
    Title: Numeric Code for the Combined Statistical Area
    Description: The numeric code that the U.S. Census Bureau Office of Management and Budget assigns for the combined statistical area (CSA) per the 2000 standards. If an institution is not defined as a CSA, the value of the field will be zero.  For more information see: http://www.census.gov/population/www/estimates/metroarea.html ."#]
    #[serde(rename="CSA_NO")]
    pub numeric_code_for_the_combined_statistical_area: Option<String>,

    #[doc = r#"## FDIC Field:: `CSA_FLG`
    Title: CSA Area Flag
    Description: A flag used to indicate whether an institution is in a Combined Statistical Area. 1=yes and 0=no."#]
    #[serde(rename="CSA_FLG")]
    pub csa_area_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `DATEUPDT`
    Title: Last update
    Description: The date of the last data update."#]
    #[serde(rename="DATEUPDT")]
    pub last_update: Option<String>,

    #[doc = r#"## FDIC Field:: `DENOVO`
    Title: Denovo Institution
    Description: A flag used to indicate whether an institution is a new institution (not a recharter). This flag is set quarterly. For instance, if REPDTE is 3/31/98 and DENOVO equals 1, the institution was a denovo during the first quarter of 1998."#]
    #[serde(rename="DENOVO")]
    pub denovo_institution: Option<String>,

    #[doc = r#"## FDIC Field:: `DEP`
    Title: Total deposits
    Description: The sum of all deposits including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices."#]
    #[serde(rename="DEP")]
    pub total_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `DEPDOM`
    Title: Deposits held in domestic offices
    Description: The sum of all domestic office deposits, including demand deposits, money market deposits, other savings deposits and time deposits."#]
    #[serde(rename="DEPDOM")]
    pub deposits_held_in_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `DOCKET`
    Title: OTS Docket Number
    Description: An identification number assigned to institutions chartered by the Office of Thrift Supervision (OTS) or members of the Federal Housing  Finance Board (FHFB) and formerly by the Federal Home Loan Bank (FHLB) Board. The value is '00000' for institutions not members of the  FHFB."#]
    #[serde(rename="DOCKET")]
    pub ots_docket_number: Option<String>,

    #[doc = r#"## FDIC Field:: `EFFDATE`
    Title: Last Structure Change Effective Date
    Description: A date indicating when an institution's change/event is effective."#]
    #[serde(rename="EFFDATE")]
    pub last_structure_change_effective_date: Option<String>,

    #[doc = r#"## FDIC Field:: `ENDEFYMD`
    Title: End date
    Description: The date that ends or closes out the last structural event relating to an institution. For closed institutions, this date represents the day that the institution became inactive."#]
    #[serde(rename="ENDEFYMD")]
    pub end_date: Option<String>,

    #[doc = r#"## FDIC Field:: `EQ`
    Title: Equity capital
    Description: Total equity capital (includes preferred and common stock, surplus and undivided profits)."#]
    #[serde(rename="EQ")]
    pub equity_capital: Option<String>,

    #[doc = r#"## FDIC Field:: `ESTYMD`
    Title: Established Date
    Description: The date on which the institution began operations."#]
    #[serde(rename="ESTYMD")]
    pub established_date: Option<String>,

    #[doc = r#"## FDIC Field:: `FDICDBS`
    Title: FDIC Geographic Region
    Description: The FDIC Office assigned to the geographic area.  The eight FDIC Regions and their respective states are:    Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont  New York - Delaware, District of Columbia, Maryland, New Jersey, New York, Pennsylvania, Puerto Rico, U.S. Virgin Islands  Atlanta - Alabama, Florida, Georgia, North Carolina, South Carolina, Virginia, West Virginia  Memphis - Arkansas, Kentucky, Louisiana, Mississippi, Tennessee  Chicago - Illinois, Indiana, Michigan, Ohio, Wisconsin   Kansas City - Iowa, Kansas, Minnesota, Missouri, Nebraska, North Dakota, South Dakota  Dallas - Colorado, New Mexico, Oklahoma, Texas  San Francisco - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#]
    #[serde(rename="FDICDBS")]
    pub fdic_geographic_region: Option<String>,

    #[doc = r#"## FDIC Field:: `FDICREGN`
    Title: FDIC Supervisory Region
    Description: The supervisory FDIC office assigned to the institution.  The eight FDIC Supervisory Regions and their respective states are:    Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont  New York - Delaware, District of Columbia, Maryland, New Jersey, New York, Pennsylvania, Puerto Rico, U.S. Virgin Islands  Atlanta - Alabama, Florida, Georgia, North Carolina, South Carolina, Virginia, West Virginia  Memphis - Arkansas, Kentucky, Louisiana, Mississippi, Tennessee  Chicago - Illinois, Indiana, Michigan, Ohio, Wisconsin   Kansas City - Iowa, Kansas, Minnesota, Missouri, Nebraska, North Dakota, South Dakota  Dallas - Colorado, New Mexico, Oklahoma, Texas  San Francisco - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#]
    #[serde(rename="FDICREGN")]
    pub fdic_supervisory_region: Option<String>,

    #[doc = r#"## FDIC Field:: `FDICSUPV`
    Title: Federal Reserve District
    Description: The supervisory FDIC office assigned to the institution. There are twelve Federal Reserve Districts, with two Districts serving one state in some instances. The list of Federal Reserve Districts and their respective states are as follows: Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont New York - Connecticut, New Jersey, New York, Puerto Rico U.S. Virgin Islands Philadelphia - Delaware, New Jersey, Pennsylvania Cleveland - Kentucky, Ohio, Pennsylvania, West Virginia Richmond - Maryland, North Carolina, South Carolina, Virginia, West Virginia Atlanta - Alabama, Florida, Georgia, Louisiana, Mississippi, Tennessee Chicago - Illinois, Indiana, Iowa, Michigan, Wisconsin St. Louis - Arkansas, Illinois, Indiana, Kentucky, Mississippi, Missouri, Tennessee Minneapolis - Michigan, Minnesota, Montana, North Dakota, South Dakota, Wisconsin Kansas City - Colorado, Kansas, Missouri, Nebraska, New Mexico, Oklahoma, Wyoming Dallas - Louisiana, New Mexico, Texas San Francisco> - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Nevada, Oregon, States of Micronesia, Utah, Washington"#]
    #[serde(rename="FDICSUPV")]
    pub federal_reserve_district: Option<String>,

    #[doc = r#"## FDIC Field:: `FED`
    Title: Federal Reserve ID Number
    Description: A number used to identify the Federal Reserve district in which the institution is located. 01 – Boston, 02 - New York,03 – Philadelphia, 04 – Cleveland,05 – Richmond,06 – Atlanta,07 – Chicago,08 - St. Louis,09 – Minneapolis,10 - Kansas city,11 – Dallas,12 - San Francisco"#]
    #[serde(rename="FED")]
    pub federal_reserve_id_number: Option<String>,

    #[doc = r#"## FDIC Field:: `FED_RSSD`
    Title: Federal Reserve ID Number
    Description: A unique number assigned by the Federal Reserve board as the entity's unique identifier"#]
    #[serde(rename="FED_RSSD")]
    pub federal_reserve_id_number_fed_rssd: Option<String>,

    #[doc = r#"## FDIC Field:: `FEDCHRTR`
    Title: Federal charter flag
    Description: A flag used to indicate whether the institution is chartered by an agent of the federal government."#]
    #[serde(rename="FEDCHRTR")]
    pub federal_charter_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `FLDOFF`
    Title: FDIC Field Office
    Description: The FDIC Field Office where an institution is physically located."#]
    #[serde(rename="FLDOFF")]
    pub fdic_field_office: Option<String>,

    #[doc = r#"## FDIC Field:: `FORM31`
    Title: FFIEC Call Report 31 Filer
    Description: A flag (1=yes,0=no) that indicates whether and institution filed an FFIEC 031 Call Report. Commercial banks with domestic and foreign offices are required to file such a report."#]
    #[serde(rename="FORM31")]
    pub ffiec_call_report_31_filer: Option<String>,

    #[doc = r#"## FDIC Field:: `HCTMULT`
    Title: Bank Holding Company Type
    Description: A flag used to indicate whether an institution is a member of a multibank holding company 1=yes, 0=no"#]
    #[serde(rename="HCTMULT")]
    pub bank_holding_company_type: Option<String>,

    #[doc = r#"## FDIC Field:: `IBA`
    Title: Insured offices of foreign banks
    Description: Includes Bank Insurance Fund insured branches in the U.S. established by banks chartered and headquartered in foreign countries.  These institutions are regulated by one of the three Federal commercial bank regulators and submit financial data to the Federal Reserve."#]
    #[serde(rename="IBA")]
    pub insured_offices_of_foreign_banks: Option<String>,

    #[doc = r#"## FDIC Field:: `INACTIVE`
    Title: Inactive
    Description: Institutions that are currently closed but were once insured by the FDIC."#]
    #[serde(rename="INACTIVE")]
    pub inactive: Option<String>,

    #[doc = r#"## FDIC Field:: `INSAGNT1`
    Title: Primary Insurance Agency
    Description: Abbreviated primary insurance agency."#]
    #[serde(rename="INSAGNT1")]
    pub primary_insurance_agency: Option<String>,

    #[doc = r#"## FDIC Field:: `INSAGNT2`
    Title: Secondary Insurance Fund
    Description: As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#]
    #[serde(rename="INSAGNT2")]
    pub secondary_insurance_fund: Option<String>,

    #[doc = r#"## FDIC Field:: `INSBIF`
    Title: Bank Insurance Fund
    Description: Institutions who are members of the Bank Insurance Fund. As of April 1, 2006 BIF was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF member institutions, that are still active or open, are now insured members of DIF."#]
    #[serde(rename="INSBIF")]
    pub bank_insurance_fund: Option<String>,

    #[doc = r#"## FDIC Field:: `INSCOML`
    Title: Insured commercial banks
    Description: Includes commercial banks insured by the FDIC.  These institutions are regulated by one of the three Federal commercial bank regulators (FDIC, Federal Reserve Board, or Office of the Comptroller of the Currency).  They submit financial reports to the Federal Reserve (state member banks) or the FDIC (state non-member banks and national banks)."#]
    #[serde(rename="INSCOML")]
    pub insured_commercial_banks: Option<String>,

    #[doc = r#"## FDIC Field:: `INSDATE`
    Title: Date of Deposit Insurance
    Description: The date that an institution obtained federal deposit insurance."#]
    #[serde(rename="INSDATE")]
    pub date_of_deposit_insurance: Option<String>,

    #[doc = r#"## FDIC Field:: `INSDROPDATE_RAW`
    Title: Date of Dropped Deposit Insurance
    Description: The date that an institution relinquished federal deposit insurance."#]
    #[serde(rename="INSDROPDATE_RAW")]
    pub date_of_dropped_deposit_insurance: Option<String>,

    #[doc = r#"## FDIC Field:: `INSDROPDATE`
    Title: Date of Dropped Deposit Insurance
    Description: The date that an institution relinquished federal deposit insurance."#]
    #[serde(rename="INSDROPDATE")]
    pub date_of_dropped_deposit_insurance_insdropdate: Option<String>,

    #[doc = r#"## FDIC Field:: `INSDIF`
    Title: Deposit Insurance Fund member
    Description: A flag used to indicate whether an institution is insured under the Deposit Insurance Fund (DIF).  As of April 1, 2006 the Bank Insurance Fund (BIF) was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF and SAIF member institutions that are still active or open are now insured members of DIF.    0 = No, not DIF insured and 1 = Yes, DIF insured.  Note that institutions that became inactive prior to April 1006 will also have zero value."#]
    #[serde(rename="INSDIF")]
    pub deposit_insurance_fund_member: Option<String>,

    #[doc = r#"## FDIC Field:: `INSFDIC`
    Title: FDIC Insured
    Description: Includes institutions insured by the FDIC."#]
    #[serde(rename="INSFDIC")]
    pub fdic_insured: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSSAIF`
    Title: SAIF Insured
    Description: Institutions who are members of the Savings Association Insurance Fund. As of April 1, 2006 SAIF was merged together with the Bank Insurance Fund (BIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured SAIF member institutions, that are still active or open, are now insured members of DIF."#]
    #[serde(rename="INSSAIF")]
    pub saif_insured: Option<String>,

    #[doc = r#"## FDIC Field:: `INSSAVE`
    Title: Insured Savings Institution
    Description: Includes savings institutions insured by the FDIC that operate under state or federal banking codes applicable to thrift institutions.  These institutions are regulated by and submit financial reports to one of two Federal regulators (FDIC or Office of Thrift Supervision)."#]
    #[serde(rename="INSSAVE")]
    pub insured_savings_institution: Option<String>,

    #[doc = r#"## FDIC Field:: `INSTAG`
    Title: Agricultural lending institution indicator
    Description: An indicator specifying whether an institution is primarily an agricultural lending institution."#]
    #[serde(rename="INSTAG")]
    pub agricultural_lending_institution_indicator: Option<String>,

    #[doc = r#"## FDIC Field:: `INSTCRCD`
    Title: Credit Card Institutions
    Description: Institutions with total loans greater than 50% of total assets and credit card loans greater than 50% of total loans, including loans that have been securitized and sold."#]
    #[serde(rename="INSTCRCD")]
    pub credit_card_institutions: Option<String>,

    #[doc = r#"## FDIC Field:: `LATITUDE`
    Title: Location Address Latitude
    Description: The latitude of the physical address."#]
    #[serde(rename="LATITUDE")]
    pub location_address_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `LAW_SASSER_FLG`
    Title: Law Sasser Flag
    Description: A flag, yes=1 and no=0 associated with OTS supervised savings associations that converted their charter to that of a commercial or savings bank.  Converted associations remain members of the SAIF, but they become subject to supervision by one of the three federal banking agencies. Not Applicable as of March 31, 2006."#]
    #[serde(rename="LAW_SASSER_FLG")]
    pub law_sasser_flag: Option<String>,

    #[doc = r#"## FDIC Field:: `LONGITUDE`
    Title: Location Address Longitude
    Description: The longitude of the physical address."#]
    #[serde(rename="LONGITUDE")]
    pub location_address_longitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `MDI_STATUS_CODE`
    Title: Minority Status Code
    Description: A numeric flag used to indicate whether an institution is primarily a minority owned institution."#]
    #[serde(rename="MDI_STATUS_CODE")]
    pub minority_status_code: Option<String>,

    #[doc = r#"## FDIC Field:: `MDI_STATUS_DESC`
    Title: Minority Status Description
    Description: A descriptive flag used to indicate type of minority owned institution."#]
    #[serde(rename="MDI_STATUS_DESC")]
    pub minority_status_description: Option<String>,

    #[doc = r#"## FDIC Field:: `MSA`
    Title: Metropolitan Statistical Area (MSA)
    Description: The Metropolitan Statistical Areas based on Census Bureau data, as defined by the US Office of Management (OMB) prior to the year 2000."#]
    #[serde(rename="MSA")]
    pub metropolitan_statistical_area_msa: Option<String>,

    #[doc = r#"## FDIC Field:: `MSA_NO`
    Title: Metropolitan Statistical Area Number
    Description: The Metropolitan Statistical Area Number (MSA_NO) in which the institution is physically located. The Office of Management and Budget defines MSAs in terms of entire counties surrounding central cities, except in the six New England states where they are defined in terms of cities and towns within counties. Before 200 standards"#]
    #[serde(rename="MSA_NO")]
    pub metropolitan_statistical_area_number: Option<String>,

    #[doc = r#"## FDIC Field:: `MUTUAL`
    Title: Ownership Type
    Description: Banking institutions fall into one of two ownership types, stock or non-stock. An institution which sells stock to raise capital is called a stock institution. It is owned by the shareholders who benefit from profits earned by the institution. A non-stock institution, or mutual institution, is owned and controlled solely by its depositors. A mutual does not issue capital stock."#]
    #[serde(rename="MUTUAL")]
    pub ownership_type: Option<String>,

    #[doc = r#"## FDIC Field:: `NAME`
    Title: Institution name
    Description: The legal title or name of the institution."#]
    #[serde(rename="NAME")]
    pub institution_name: Option<String>,

    #[doc = r#"## FDIC Field:: `NAMEHCR`
    Title: Bank Holding Company (Regulatory Top Holder)
    Description: Regulatory top holder is assigned by the Federal Reserve Board based on ownership and control percentages. Note: Information on bank holding companies is only as of quarter-end. Regulatory top holder is any company that directly or indirectly owns, controls or has power to vote 25 percent or more of a bank's or direct holding company's shares or  controls in any manner the election of a majority of the directors or trustees of a bank or direct holding company or  exercises a controlling influence over the management or policies of a bank or direct holding company.   Information on Thrift Holding Companies that own Savings Associations but do not own banks is not currently available in the ID System.  Source:  Federal Reserve Board National Information Center data base."#]
    #[serde(rename="NAMEHCR")]
    pub bank_holding_company_regulatory_top_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `NETINC`
    Title: Net income
    Description: Net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    #[serde(rename="NETINC")]
    pub net_income: Option<f64>,

    #[doc = r#"## FDIC Field:: `NETINCQ`
    Title: Net income - quarterly
    Description: Quarterly net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#]
    #[serde(rename="NETINCQ")]
    pub net_income_quarterly: Option<f64>,

    #[doc = r#"## FDIC Field:: `NEWCERT`
    Title: New certificate number
    Description: A new certificate number of an already existing FDIC-insured institution resulting from either a merger or an acquisition."#]
    #[serde(rename="NEWCERT")]
    pub new_certificate_number: Option<String>,

    #[doc = r#"## FDIC Field:: `OAKAR`
    Title: Oakar Institutions
    Description: A member of one insurance fund that acquired deposits insured by the other fund, where that portion of the buyer's deposits remained insured by, and assessable by, the other fund."#]
    #[serde(rename="OAKAR")]
    pub oakar_institutions: Option<String>,

    #[doc = r#"## FDIC Field:: `OCCDIST`
    Title: Office of the Comptroller
    Description: The Office of the Comptroller of the Currency (OCC) District in  which the institution is physically located. The six OCC Districts  and their respective states are: Northeast - Connecticut, Delaware,  District of Columbia, Maine, Maryland, Massachusetts, New Hampshire,  New Jersey, New York, Pennsylvania, Puerto Rico, Rhode Island,  Vermont, U.S. Virgin Islands Southeast - Alabama, Florida, Georgia,  Mississippi, North Carolina, South Carolina, Tennessee, Virginia,  West Virginia Central - Illinois, Indiana, Kentucky, Michigan,  Ohio, Wisconsin Midwest - Iowa, Kansas, Minnesota, Missouri,  Nebraska, North Dakota, South Dakota Southwest - Arkansas,  Louisiana, New Mexico, Oklahoma, Texas West - Alaska, American  Samoa, Arizona, California, Colorado, Guam, Hawaii, Idaho, Montana,  Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#]
    #[serde(rename="OCCDIST")]
    pub office_of_the_comptroller: Option<String>,

    #[doc = r#"## FDIC Field:: `OFFDOM`
    Title: Number of Domestic Offices
    Description: The number of domestic offices (including headquarters) operated by active institutions in the 50 states of the U.S.A."#]
    #[serde(rename="OFFDOM")]
    pub number_of_domestic_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFFOR`
    Title: Number of Foreign Offices
    Description: The number of foreign offices (outside the U.S.) operated by the institution."#]
    #[serde(rename="OFFFOR")]
    pub number_of_foreign_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFICES`
    Title: Office
    Description: A branch/office is any location, or facility, of a financial institution, including its main office, where deposit accounts are opened, deposits are accepted, checks paid, and loans granted. Some branches include, but are not limited to, brick and mortar locations, detached or attached drive-in facilities, seasonal offices, offices on military bases or government installations, paying/receiving stations or units, nondeposit offices, Internet and PhoneBanking locations where a customer can open accounts, make deposits and borrow money. A branch does not include Automated Teller Machines (ATM), Consumer Credit Offices, Contractual Offices, Customer Bank Communication Terminals (CBCT), Electronic Fund Transfer Units (EFTU), and Loan Production Offices Summary of Deposits information is required for each insured office located in any State, the District of Columbia, the Commonwealth of Puerto Rico or any U.S. territory or possession such as Guam or the U.S. Virgin Islands, without regard to the location of the main office."#]
    #[serde(rename="OFFICES")]
    pub office: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFOA`
    Title: Number of US Offices
    Description: The number of offices operated by an FDIC-insured institution in all commonwealths and territories of the US, along with those in freely associated states under the Compact of Free Association"#]
    #[serde(rename="OFFOA")]
    pub number_of_us_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSDIST`
    Title: OTS District
    Description: Office of Thrift Supervision (OTS) District - No longer used as of 7/21/11"#]
    #[serde(rename="OTSDIST")]
    pub ots_district: Option<String>,

    #[doc = r#"## FDIC Field:: `OTSREGNM`
    Title: Office of Thrift Supervision Region
    Description: Prior to 7/21/11, the Office of Thrift Supervision (OTS) Region in  which the institution is physically located. The five OTS Regions  and their respective states are: Northeast - Connecticut, Delaware,  Maine, Massachusetts, New Hampshire, New Jersey, New York,  Pennsylvania, Rhode Island, Vermont, West Virginia Southeast -  Alabama, District of Columbia, Florida, Georgia, Maryland, North  Carolina, Puerto Rico, South Carolina, U.S. Virgin Islands, Virginia  Central - Illinois, Indiana, Kentucky, Michigan, Ohio, Tennessee,  Wisconsin Midwest - Arkansas, Colorado, Iowa, Kansas, Louisiana,  Minnesota, Mississippi, Missouri, Nebraska, New Mexico, North  Dakota, Oklahoma, South Dakota, Texas West - Alaska, American Samoa,  Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, States of  Micronesia, Oregon, Utah, Washington, Wyoming"#]
    #[serde(rename="OTSREGNM")]
    pub office_of_thrift_supervision_region: Option<String>,

    #[doc = r#"## FDIC Field:: `PARCERT`
    Title: Directly owned by another bank (CERT)
    Description: The PARCERT number identifies the subsidiary institutions parent certificate number. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports."#]
    #[serde(rename="PARCERT")]
    pub directly_owned_by_another_bank_cert_parcert: Option<String>,

    #[doc = r#"## FDIC Field:: `PROCDATE`
    Title: Last Structure Change Process Date
    Description: A date indicating when an institution's change/event is processed."#]
    #[serde(rename="PROCDATE")]
    pub last_structure_change_process_date: Option<String>,

    #[doc = r#"## FDIC Field:: `QBPRCOML`
    Title: Quarterly Banking Profile Commercial Bank Region
    Description: The Quarterly Banking Profile (QBP) Commercial Bank Region in which the institution is physically located. Select from a drop down box. regional breakdown. group data by qbp region is only available for insured commercial banks and insured savings institutions and NOT All Insured Institutions, Insured Commercial Banks by asset size and Insured Savings Institutions by asset size."#]
    #[serde(rename="QBPRCOML")]
    pub quarterly_banking_profile_commercial_bank_region: Option<String>,

    #[doc = r#"## FDIC Field:: `REGAGNT`
    Title: Primary Regulator
    Description: A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#]
    #[serde(rename="REGAGNT")]
    pub primary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `REGAGENT2`
    Title: Secondary Regulator
    Description: A code indicating the federal regulatory agency that provides secondary supervision over an institution. CFPB = Consumer Financial Protection Bureau; OTS=Office of Thrift Supervision."#]
    #[serde(rename="REGAGENT2")]
    pub secondary_regulator: Option<String>,

    #[doc = r#"## FDIC Field:: `REPDTE`
    Title: Report Date
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="REPDTE")]
    pub report_date: Option<String>,

    #[doc = r#"## FDIC Field:: `RISDATE`
    Title: Report Date
    Description: The financial reporting period selected in CCYYMM format."#]
    #[serde(rename="RISDATE")]
    pub report_date_risdate: Option<String>,

    #[doc = r#"## FDIC Field:: `ROA`
    Title: Return on assets (ROA)
    Description: Net income after taxes and extraordinary items (annualized) as a percent of average total assets."#]
    #[serde(rename="ROA")]
    pub return_on_assets_roa: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAPTX`
    Title: Pretax return on assets
    Description: Annualized pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    #[serde(rename="ROAPTX")]
    pub pretax_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAPTXQ`
    Title: Quarterly Pretax return on assets
    Description: Quarterly pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#]
    #[serde(rename="ROAPTXQ")]
    pub quarterly_pretax_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROAQ`
    Title: Quarterly return on assets
    Description: Quarterly net income after taxes and extraordinary items as a percent of average total assets."#]
    #[serde(rename="ROAQ")]
    pub quarterly_return_on_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROE`
    Title: Return on Equity (ROE)
    Description: Annualized net income as a percent of average equity on a consolidated basis.     Note: If retained earnings are  negative, the ratio is shown as NA."#]
    #[serde(rename="ROE")]
    pub return_on_equity_roe: Option<f64>,

    #[doc = r#"## FDIC Field:: `ROEQ`
    Title: Quarterly return on equity
    Description: Quarterly net income (including gains or losses on securities and extraordinary items) as a percentage of average total equity capital."#]
    #[serde(rename="ROEQ")]
    pub quarterly_return_on_equity: Option<f64>,

    #[doc = r#"## FDIC Field:: `RSSDHCR`
    Title: RSSDID - High Regulatory Holder
    Description: The unique number assigned by the Federal Reserve Board to the regulatory high holding company of the institution."#]
    #[serde(rename="RSSDHCR")]
    pub rssdid_high_regulatory_holder: Option<String>,

    #[doc = r#"## FDIC Field:: `RUNDATE`
    Title: Run Date
    Description: The day the institution information was updated."#]
    #[serde(rename="RUNDATE")]
    pub run_date: Option<String>,

    #[doc = r#"## FDIC Field:: `SASSER`
    Title: Sasser Institutions
    Description: OTS supervised savings associations that converted their charter to that of a commercial or savings bank.  Converted associations remain members of the SAIF, but they become subject to supervision by one of the three federal banking agencies. Not Applicable as of March 31, 2006."#]
    #[serde(rename="SASSER")]
    pub sasser_institutions: Option<String>,

    #[doc = r#"## FDIC Field:: `SPECGRP`
    Title: Asset Concentration Hierarchy
    Description: An indicator of an institution's primary specialization in terms of asset concentration"#]
    #[serde(rename="SPECGRP")]
    pub asset_concentration_hierarchy: Option<f64>,

    #[doc = r#"## FDIC Field:: `SPECGRPN`
    Title: Specialization Group
    Description: Name associated with the numeric indicator (SPECGRP) of an institution's primary specialization in terms of asset concentration"#]
    #[serde(rename="SPECGRPN")]
    pub specialization_group: Option<String>,

    #[doc = r#"## FDIC Field:: `STALP`
    Title: State Alpha code
    Description: The state abbreviation of the location of the institution's main office."#]
    #[serde(rename="STALP")]
    pub state_alpha_code: Option<String>,

    #[doc = r#"## FDIC Field:: `STALPHCR`
    Title: Regulatory holding company state location
    Description: State location of the regulatory high holding company (either direct or indirect owner)."#]
    #[serde(rename="STALPHCR")]
    pub regulatory_holding_company_state_location: Option<String>,

    #[doc = r#"## FDIC Field:: `STCHRTR`
    Title: State Charter
    Description: A flag (1=yes;0=no) that indicates if an institution is state chartered."#]
    #[serde(rename="STCHRTR")]
    pub state_charter: Option<String>,

    #[doc = r#"## FDIC Field:: `STCNTY`
    Title: State and county number
    Description: A five digit number representing the state and county in which the institution is physically located.  The first two digits represent the FIPS state numeric code and the last three digits represent the FIPS county numeric code."#]
    #[serde(rename="STCNTY")]
    pub state_and_county_number: Option<String>,

    #[doc = r#"## FDIC Field:: `STNAME`
    Title: State Name
    Description: State in which the the institution is physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="STNAME")]
    pub state_name: Option<String>,

    #[doc = r#"## FDIC Field:: `STNUM`
    Title: State Number
    Description: The Federal Information Processing Standard code used to identify states"#]
    #[serde(rename="STNUM")]
    pub state_number: Option<String>,

    #[doc = r#"## FDIC Field:: `SUBCHAPS`
    Title: Subchapter S Corporations
    Description: The Small Business Job Protection Act of 1996 changed the Internal Revenue Code to allow financial institutions to elect Subchapter S corporation status, beginning in 1997. Banks are required to indicate on the Call Report whether there is currently in effect an election to file under Subchapter S. Thrifts have a similar requirement as of March 1998.  The most important IRS requirements to elect and maintain Subchapter S status are: There can be no more than 75 eligible shareholders and no more than one class of stock. (In general, shareholders can only be individuals, estates, and certain types of trusts. Certain retirement plans and charitable organizations will be eligible in 1998.) All shareholders must consent.  Banks and thrifts converting to Subchapter S status must use the specific charge-off method for tax purposes rather than the reserve method of accounting for bad debts and recapture tax bad debt reserves over a period of six years, if the reserve method had been used prior to conversion. (Note: even though the specific charge-off method is required for tax purposes, an adequate allowance for loan and lease losses must still be maintained on the financial statements and Call Reports.) Banks and thrifts are subject to a built-in gains (BIG) tax, if the aggregate fair market value of assets is greater than their aggregate adjusted bases on the date of conversion to Subchapter S status.     [Banks are required to indicate separately on the Call Report in December of each year, the deferred portion of income taxes reported in net income. For Subchapter S banks, some or all of their deferred tax assets and liabilities may be eliminated upon conversion to Subchapter S status; however, deferred taxes related to the BIG tax and the recapture of bad debt reserves must be recognized.].   A Subchapter S corporation is treated as a pass-through entity, similar to a partnership, for federal income tax purposes. It is generally not subject to any federal income taxes at the corporate level. Its taxable income flows through to its shareholders in proportion to their stock ownership, and the shareholders generally pay federal income taxes on their share of this taxable income. This can have the effect of reducing institutions' reported income tax expense and increasing their after-tax earnings..   The election of Subchapter S status may result in an increase in shareholders' personal tax liabilities. Therefore, S corporations typically increase the amount of earnings distributed as dividends to compensate for higher personal taxes."#]
    #[serde(rename="SUBCHAPS")]
    pub subchapter_s_corporations: Option<String>,

    #[doc = r#"## FDIC Field:: `SUPRV_FD`
    Title: Supervisory Region Number
    Description: A two-digit number indicating the FDIC Supervisory Division or Region. 02 = New York; 05 = Atlanta; 09 = Chicago; 11 = Kansas City; 13 = Dallas; 14 = San Francisco; 16 = Office of Complex Financial Institutions (CFI)"#]
    #[serde(rename="SUPRV_FD")]
    pub supervisory_region_number: Option<String>,

    #[doc = r#"## FDIC Field:: `TE01N528`
    Title: Web Site URL 01
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE01N528")]
    pub web_site_url_01: Option<String>,

    #[doc = r#"## FDIC Field:: `TE02N528`
    Title: Web Site URL 02
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE02N528")]
    pub web_site_url_02: Option<String>,

    #[doc = r#"## FDIC Field:: `TE03N528`
    Title: Web Site URL 03
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE03N528")]
    pub web_site_url_03: Option<String>,

    #[doc = r#"## FDIC Field:: `TE04N528`
    Title: Web Site URL 04
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE04N528")]
    pub web_site_url_04: Option<String>,

    #[doc = r#"## FDIC Field:: `TE05N528`
    Title: Web Site URL 05
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE05N528")]
    pub web_site_url_05: Option<String>,

    #[doc = r#"## FDIC Field:: `TE06N528`
    Title: Web Site URL 06
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE06N528")]
    pub web_site_url_06: Option<String>,

    #[doc = r#"## FDIC Field:: `TE07N528`
    Title: Web Site URL 07
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE07N528")]
    pub web_site_url_07: Option<String>,

    #[doc = r#"## FDIC Field:: `TE08N528`
    Title: Web Site URL 08
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE08N528")]
    pub web_site_url_08: Option<String>,

    #[doc = r#"## FDIC Field:: `TE09N528`
    Title: Web Site URL 09
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE09N528")]
    pub web_site_url_09: Option<String>,

    #[doc = r#"## FDIC Field:: `TE10N528`
    Title: Web Site URL 10
    Description: URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#]
    #[serde(rename="TE10N528")]
    pub web_site_url_10: Option<String>,

    #[doc = r#"## FDIC Field:: `TE01N529`
    Title: Trade Name 01
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE01N529")]
    pub trade_name_01: Option<String>,

    #[doc = r#"## FDIC Field:: `TE02N529`
    Title: Trade Name 02
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE02N529")]
    pub trade_name_02: Option<String>,

    #[doc = r#"## FDIC Field:: `TE03N529`
    Title: Trade Name 03
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE03N529")]
    pub trade_name_03: Option<String>,

    #[doc = r#"## FDIC Field:: `TE04N529`
    Title: Trade Name 04
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE04N529")]
    pub trade_name_04: Option<String>,

    #[doc = r#"## FDIC Field:: `TE05N529`
    Title: Trade Name 05
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE05N529")]
    pub trade_name_05: Option<String>,

    #[doc = r#"## FDIC Field:: `TE06N529`
    Title: Trade Name 06
    Description: Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#]
    #[serde(rename="TE06N529")]
    pub trade_name_06: Option<String>,

    #[doc = r#"## FDIC Field:: `TRACT`
    Title: 
    Description: Beyond having trust powers granted and exercised, institutions with fiduciary assets accounts, income, or other reportable fiduciary related service"#]
    #[serde(rename="TRACT")]
    pub tract: Option<String>,

    #[doc = r#"## FDIC Field:: `TRUST`
    Title: Trust Powers
    Description: A number corresponding to a valid type of trust power that an institution can possess and exercise. 00=Trust Powers Not Known; 10=Full Trust Powers Granted; 11=Full Trust Powers Granted, Exercised; 12=Full Trust Powers Granted, Not Exercised; 20=Limited Trust Powers Granted; 21=Limited Trust Powers Granted, Exercised; 30=Trust Powers Not Granted; 31=Trust Powers Not Granted, But Exercised; 40=Trust Powers Grandfathered"#]
    #[serde(rename="TRUST")]
    pub trust_powers: Option<String>,

    #[doc = r#"## FDIC Field:: `ULTCERT`
    Title: Ultimate Cert
    Description: The cert number of the last successor or acquirer of the institution"#]
    #[serde(rename="ULTCERT")]
    pub ultimate_cert: Option<String>,

    #[doc = r#"## FDIC Field:: `UNINUM`
    Title: FDIC's unique number
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    #[serde(rename="UNINUM")]
    pub fdic_s_unique_number: Option<String>,

    #[doc = r#"## FDIC Field:: `WEBADDR`
    Title: Primary Internet Web Address
    Description: The primary internet web address is the public internet site obtained from the most recent FFIEC Call Report (CALL) for commercial banks or from the supplemental information for Thrift Financial Reporters (TFR). The primary internet web address is included only for those institutions reporting an address on the most recent FFIEC Call Report or Thrift Financial Report."#]
    #[serde(rename="WEBADDR")]
    pub primary_internet_web_address: Option<String>,

    #[doc = r#"## FDIC Field:: `ZIP`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#]
    #[serde(rename="ZIP")]
    pub zip_code: Option<String>,

}

/// FDIC BankFind API `/institutions` endpoint handler
/// Get Financial Institutions
/// Returns a list of financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter for the bank search. All values must be entered in UPPERCASE.
Examples:
* Filter by State name  
`STNAME:\"West Virginia\"`    
* Filter for any one of multiple State names  
`STNAME:(\"West Virginia\",\"Delaware\")`
* Filter all but a specified value  
`!(STNAME:\"Virginia\")`  
* Filter by last updated within an inclusive date range  
`DATEUPDT:&#91;\"2010-01-01\" TO \"2010-12-31\"&#93;`
* Filter for deposits over 50,000,000 (50000 thousands of dollars)  
`DEP:&#91;50000 TO *&#93;`
    Example: STALP:IA AND ACTIVE:1
 - `search` (String, optional): Flexible text search against institution records - currently only supporting name search. 
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by name
`NAME: Island`
* Search by name (fuzzy match)
`NAME: Iland`
 - `fields` (String, optional): Comma delimited list of fields to search. All values must be entered in UPPERCASE.
    Example: ZIP,OFFDOM,CITY,COUNTY,STNAME,STALP,NAME,ACTIVE,CERT,CBSA,ASSET,NETINC,DEP,DEPDOM,ROE,ROA,DATEUPDT,OFFICES
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: OFFICES
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/institutions",
    params(InstitutionsParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<InstitutionsProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Structure"
)]
pub async fn institutions_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<InstitutionsParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "institutions",
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = InstitutionsParameters {
            common: CommonParameters::default(),
            search: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }
    #[test]
    fn test_properties_struct_serialization() {
        let props = InstitutionsProperties {
            
            institution_status: None,
            street_address: None,
            total_assets: None,
            institution_class: None,
            community_bank: None,
            core_based_statistical_area_name: None,
            metropolitan_divisions_name: None,
            metropolitan_divisions_flag: None,
            metropolitan_divisions_number: None,
            metropolitan_division_number: None,
            metropolitan_division_flag: None,
            metropolitan_division_name: None,
            micropolitan_division_flag: None,
            core_based_statistical_areas: None,
            fdic_certificate: None,
            directly_owned_by_another_bank_cert: None,
            cfpb_effective_date: None,
            cfpb_end_date: None,
            cfpb_flag: None,
            previous_name_1: None,
            previous_name_2: None,
            previous_name_3: None,
            previous_name_4: None,
            previous_name_5: None,
            previous_name_6: None,
            previous_name_7: None,
            previous_name_8: None,
            previous_name_9: None,
            previous_name_10: None,
            change_code: None,
            change_code_changec2: None,
            change_code_changec3: None,
            change_code_changec4: None,
            change_code_changec5: None,
            change_code_changec6: None,
            change_code_changec7: None,
            change_code_changec8: None,
            change_code_changec9: None,
            change_code_changec10: None,
            change_code_changec11: None,
            change_code_changec12: None,
            change_code_changec13: None,
            change_code_changec14: None,
            change_code_changec15: None,
            occ_charter_number: None,
            chartering_agency: None,
            city: None,
            city_of_high_holder: None,
            numeric_code: None,
            consolidated_metropolitan_statistical_division_number: None,
            consolidated_metropolitan_statistical_area: None,
            conservatorship: None,
            county: None,
            combined_statistical_area_name: None,
            numeric_code_for_the_combined_statistical_area: None,
            csa_area_flag: None,
            last_update: None,
            denovo_institution: None,
            total_deposits: None,
            deposits_held_in_domestic_offices: None,
            ots_docket_number: None,
            last_structure_change_effective_date: None,
            end_date: None,
            equity_capital: None,
            established_date: None,
            fdic_geographic_region: None,
            fdic_supervisory_region: None,
            federal_reserve_district: None,
            federal_reserve_id_number: None,
            federal_reserve_id_number_fed_rssd: None,
            federal_charter_flag: None,
            fdic_field_office: None,
            ffiec_call_report_31_filer: None,
            bank_holding_company_type: None,
            insured_offices_of_foreign_banks: None,
            inactive: None,
            primary_insurance_agency: None,
            secondary_insurance_fund: None,
            bank_insurance_fund: None,
            insured_commercial_banks: None,
            date_of_deposit_insurance: None,
            date_of_dropped_deposit_insurance: None,
            date_of_dropped_deposit_insurance_insdropdate: None,
            deposit_insurance_fund_member: None,
            fdic_insured: None,
            saif_insured: None,
            insured_savings_institution: None,
            agricultural_lending_institution_indicator: None,
            credit_card_institutions: None,
            location_address_latitude: None,
            law_sasser_flag: None,
            location_address_longitude: None,
            minority_status_code: None,
            minority_status_description: None,
            metropolitan_statistical_area_msa: None,
            metropolitan_statistical_area_number: None,
            ownership_type: None,
            institution_name: None,
            bank_holding_company_regulatory_top_holder: None,
            net_income: None,
            net_income_quarterly: None,
            new_certificate_number: None,
            oakar_institutions: None,
            office_of_the_comptroller: None,
            number_of_domestic_offices: None,
            number_of_foreign_offices: None,
            office: None,
            number_of_us_offices: None,
            ots_district: None,
            office_of_thrift_supervision_region: None,
            directly_owned_by_another_bank_cert_parcert: None,
            last_structure_change_process_date: None,
            quarterly_banking_profile_commercial_bank_region: None,
            primary_regulator: None,
            secondary_regulator: None,
            report_date: None,
            report_date_risdate: None,
            return_on_assets_roa: None,
            pretax_return_on_assets: None,
            quarterly_pretax_return_on_assets: None,
            quarterly_return_on_assets: None,
            return_on_equity_roe: None,
            quarterly_return_on_equity: None,
            rssdid_high_regulatory_holder: None,
            run_date: None,
            sasser_institutions: None,
            asset_concentration_hierarchy: None,
            specialization_group: None,
            state_alpha_code: None,
            regulatory_holding_company_state_location: None,
            state_charter: None,
            state_and_county_number: None,
            state_name: None,
            state_number: None,
            subchapter_s_corporations: None,
            supervisory_region_number: None,
            web_site_url_01: None,
            web_site_url_02: None,
            web_site_url_03: None,
            web_site_url_04: None,
            web_site_url_05: None,
            web_site_url_06: None,
            web_site_url_07: None,
            web_site_url_08: None,
            web_site_url_09: None,
            web_site_url_10: None,
            trade_name_01: None,
            trade_name_02: None,
            trade_name_03: None,
            trade_name_04: None,
            trade_name_05: None,
            trade_name_06: None,
            tract: None,
            trust_powers: None,
            ultimate_cert: None,
            fdic_s_unique_number: None,
            primary_internet_web_address: None,
            zip_code: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
