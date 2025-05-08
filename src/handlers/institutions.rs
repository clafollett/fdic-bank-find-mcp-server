//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/institutions` endpoint.

// Internal imports (std, crate)
use crate::common::*;
use crate::config::FdicApiConfig;

// External imports (alphabetized)
use rmcp::handler::server::tool::IntoCallToolResult;
use rmcp::model::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
use utoipa::ToSchema;

/// Auto-generated parameters struct for `/institutions` endpoint.
/// Spec: institution_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct InstitutionsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[schemars(description = r#"Flexible text search against institution records - currently only supporting name search. 
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by name
`NAME: Island`
* Search by name (fuzzy match)
`NAME: Iland`"#)]
    pub search: Option<String>,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for InstitutionsParameters {
    fn name() -> &'static str {
        "institutions"
    }
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

    #[allow(unused_variables)] // the `query` parameter is unused if there are no endpoint-specific parameters
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct InstitutionsProperties {
    #[schemars(description = r#"Institution Status - A number indicating the status of an institution. 1='Institutions that are currently open and insured by the FDIC'; 0='Institution closed or not insured by FDIC'"#)]
    pub active: Option<String>,

    #[schemars(description = r#"Street Address - The street address in which an institution or branch office is physically located."#)]
    pub address: Option<String>,

    #[schemars(description = r#"Total assets - The sum of all assets owned by the institution including cash, loans, securities, bank premises and other assets. This total does not include off-balance-sheet accounts."#)]
    pub asset: Option<f32>,

    #[schemars(description = r#"Institution Class - A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#)]
    pub bkclass: Option<String>,

    #[schemars(description = r#"Community Bank - FDIC community banks are identified based on criteria defined in the FDIC Community Banking Study. Using detailed balance sheet and geographic data, the study defines community banks in terms of their traditional relationship banking and limited geographic scope of operations"#)]
    pub cb: Option<String>,

    #[schemars(description = r#"Core Based Statistical Area Name - Name of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Name - Name of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_div: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Flag - A flag (1=Yes) indicating member of a Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#)]
    pub cbsa_div_flg: Option<String>,

    #[schemars(description = r#"Metropolitan Divisions Number - Numeric code of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget.s"#)]
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

    #[schemars(description = r#"FDIC Certificate # - A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#)]
    pub cert: Option<String>,

    #[schemars(description = r#"Directly owned by another bank (CERT) - FDIC certificate number of the parent bank or savings institution with which the reported institution’s financial data has been consolidated. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports.  Click on the certificate number to identify the parent bank or thrift."#)]
    pub certcons: Option<String>,

    #[schemars(description = r#"CFPB Effective Date - Date the institution began secondary supervision by CFPB"#)]
    pub cfpbeffdte: Option<String>,

    #[schemars(description = r#"CFPB End Date - Date the institution ended supervision by CFPB"#)]
    pub cfpbenddte: Option<String>,

    #[schemars(description = r#"CFPB Flag - Indicates secondary supervision by CFPB ('0' - not supervised by CFPB, '1'- secondarily supervised by CFPB)"#)]
    pub cfpbflag: Option<String>,

    #[schemars(description = r#"Previous Name 1 - Previous Name of Institution 1"#)]
    pub priorname1: Option<String>,

    #[schemars(description = r#"Previous Name 2 - Previous Name of Institution 2"#)]
    pub priorname2: Option<String>,

    #[schemars(description = r#"Previous Name 3 - Previous Name of Institution 3"#)]
    pub priorname3: Option<String>,

    #[schemars(description = r#"Previous Name 4 - Previous Name of Institution 4"#)]
    pub priorname4: Option<String>,

    #[schemars(description = r#"Previous Name 5 - Previous Name of Institution 5"#)]
    pub priorname5: Option<String>,

    #[schemars(description = r#"Previous Name 6 - Previous Name of Institution 6"#)]
    pub priorname6: Option<String>,

    #[schemars(description = r#"Previous Name 7 - Previous Name of Institution 7"#)]
    pub priorname7: Option<String>,

    #[schemars(description = r#"Previous Name 8 - Previous Name of Institution 8"#)]
    pub priorname8: Option<String>,

    #[schemars(description = r#"Previous Name 9 - Previous Name of Institution 9"#)]
    pub priorname9: Option<String>,

    #[schemars(description = r#"Previous Name 10 - Previous Name of Institution 10"#)]
    pub priorname10: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec1: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec2: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec3: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec4: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec5: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec6: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec7: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec8: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec9: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec10: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec11: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec12: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec13: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec14: Option<String>,

    #[schemars(description = r#"Change Code - FDIC code used to signify a structural event relating to an institution."#)]
    pub changec15: Option<String>,

    #[schemars(description = r#"OCC Charter Number - The charter number identifying a financial institution supervised by the Office of Comptroller of Currency."#)]
    pub charter: Option<String>,

    #[schemars(description = r#"Chartering Agency - All Chartering Agencies - State and Federal  Comptroller of the Currency - Chartering authority for nationally chartered commercial banks and for federally chartered savings associations (The Office of Thrift Supervision (OTS) before 7/21/11)  State (includes U.S. Territories) - Chartering authority for institutions that are not chartered by the OCC or OTS"#)]
    pub chrtagnt: Option<String>,

    #[schemars(description = r#"City - The city in which an institution or branch office is physically located."#)]
    pub city: Option<String>,

    #[schemars(description = r#"City of High Holder - City in which the headquarters of the institution's regulatory high holder are physically located."#)]
    pub cityhcr: Option<String>,

    #[schemars(description = r#"Numeric code - A number that sub-categorizes a major class of institutions. 3 = National bank, Federal Reserve System (FRS) member; 13 = State commercial bank, FRS member; 15 = State savings, co-op, or industrial bank, FRS member; 21 = State commercial bank, not FRS member; 23 = State savings, co-op, or industrial bank, not FRS member; 25 - State mutual commercial bank, not FRS member; 33 = Federal chartered stock savings bank; 34 = Federal chartered mutual saving bank; 35 = State chartered stock savings and loan association; 36 = State chartered mutual savings and loan association; 37 = Federal chartered stock savings and loan association; 38 = Federal chartered mutual savings and loan association; 41 = State chartered stock savings bank; 42 = State chartered mutual savings bank; 43 = Federal chartered stock savings bank (historical); 44 = Federal chartered mutual savings bank (historical); 52 = Insured domestic offices of foreign banks (International Banking Act(IBA)); 50 = OCC chartered nondeposit and/or noninsured trust companies; 51 = Noninsured commercial bank; 52 = Noninsured domestic offices of foreign bank (International Banking Act); 53 = Noninsured industrial bank; 54 = State chartered nondeposit and/or noninsured trust company, not FRS member; 55 = State chartered domestic branches of foreign banks; 56 = OCC chartered domestic branches of foreign banks; 57 = New York investment company; 58 = State chartered nondeposit and/or noninsured trust company, FRS member; 59 = OTS chartered nondeposit and/or noninsured trust company; 61 = Noninsured private bank; 62 = Noninsured loan workout bank, OCC chartered; 63 = Noninsured loan workout bank, state chartered, FRS member; 64 = Noninsured loan workout bank, state chartered, not FRS member; 65 = Other holding company; 71 = Transfer agent; 81 = Noninsured stock savings bank; 82 = Noninsured mutual savings bank; 85 = Noninsured stock savings and loan association; 86 = Noninsured mutual savings and loan association; 89 = Noninsured insurance company; 91 = State chartered credit unions; 92 = Federal chartered credit unions; 93 = Privately insured state credit union."#)]
    pub clcode: Option<f32>,

    #[schemars(description = r#"Consolidated Metropolitan Statistical Division Number - The numeric code given by the US Census Bureau office of Management and Budget that represents the CMSA prior to the year 2000 standards. 1=yes"#)]
    pub cmsa_no: Option<String>,

    #[schemars(description = r#"Consolidated Metropolitan Statistical Area - The Federal Information Processing Standards (FIPS) Consolidated Metropolitan Statistical Area (CMSA) code is a number representing the institution location. CMSA consists of two or more contiguous Metropolitan Statistical Areas (MSA) with a combined population of over 1 Million.  Note: If an institution is not located in a CMSA, the value of the field will be zeroes."#)]
    pub cmsa: Option<String>,

    #[schemars(description = r#"Conservatorship - A flag (1=yes;0=no) that indicates if an institution is being operated in government conservatorship."#)]
    pub conserve: Option<String>,

    #[schemars(description = r#"County - The county name in which an institution or branch office is physically located."#)]
    pub county: Option<String>,

    #[schemars(description = r#"Combined Statistical Area Name - Name of the Combined Statistical Area (CSA) as defined by the US Census Bureau."#)]
    pub csa: Option<String>,

    #[schemars(description = r#"Numeric Code for the Combined Statistical Area - The numeric code that the U.S. Census Bureau Office of Management and Budget assigns for the combined statistical area (CSA) per the 2000 standards. If an institution is not defined as a CSA, the value of the field will be zero.  For more information see: http://www.census.gov/population/www/estimates/metroarea.html ."#)]
    pub csa_no: Option<String>,

    #[schemars(description = r#"CSA Area Flag - A flag used to indicate whether an institution is in a Combined Statistical Area. 1=yes and 0=no."#)]
    pub csa_flg: Option<String>,

    #[schemars(description = r#"Last update - The date of the last data update."#)]
    pub dateupdt: Option<String>,

    #[schemars(description = r#"Denovo Institution - A flag used to indicate whether an institution is a new institution (not a recharter). This flag is set quarterly. For instance, if REPDTE is 3/31/98 and DENOVO equals 1, the institution was a denovo during the first quarter of 1998."#)]
    pub denovo: Option<String>,

    #[schemars(description = r#"Total deposits - The sum of all deposits including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices."#)]
    pub dep: Option<f32>,

    #[schemars(description = r#"Deposits held in domestic offices - The sum of all domestic office deposits, including demand deposits, money market deposits, other savings deposits and time deposits."#)]
    pub depdom: Option<f32>,

    #[schemars(description = r#"OTS Docket Number - An identification number assigned to institutions chartered by the Office of Thrift Supervision (OTS) or members of the Federal Housing  Finance Board (FHFB) and formerly by the Federal Home Loan Bank (FHLB) Board. The value is '00000' for institutions not members of the  FHFB."#)]
    pub docket: Option<String>,

    #[schemars(description = r#"Last Structure Change Effective Date - A date indicating when an institution's change/event is effective."#)]
    pub effdate: Option<String>,

    #[schemars(description = r#"End date - The date that ends or closes out the last structural event relating to an institution. For closed institutions, this date represents the day that the institution became inactive."#)]
    pub endefymd: Option<String>,

    #[schemars(description = r#"Equity capital - Total equity capital (includes preferred and common stock, surplus and undivided profits)."#)]
    pub eq: Option<String>,

    #[schemars(description = r#"Established Date - The date on which the institution began operations."#)]
    pub estymd: Option<String>,

    #[schemars(description = r#"FDIC Geographic Region - The FDIC Office assigned to the geographic area.  The eight FDIC Regions and their respective states are:    Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont  New York - Delaware, District of Columbia, Maryland, New Jersey, New York, Pennsylvania, Puerto Rico, U.S. Virgin Islands  Atlanta - Alabama, Florida, Georgia, North Carolina, South Carolina, Virginia, West Virginia  Memphis - Arkansas, Kentucky, Louisiana, Mississippi, Tennessee  Chicago - Illinois, Indiana, Michigan, Ohio, Wisconsin   Kansas City - Iowa, Kansas, Minnesota, Missouri, Nebraska, North Dakota, South Dakota  Dallas - Colorado, New Mexico, Oklahoma, Texas  San Francisco - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#)]
    pub fdicdbs: Option<String>,

    #[schemars(description = r#"FDIC Supervisory Region - The supervisory FDIC office assigned to the institution.  The eight FDIC Supervisory Regions and their respective states are:    Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont  New York - Delaware, District of Columbia, Maryland, New Jersey, New York, Pennsylvania, Puerto Rico, U.S. Virgin Islands  Atlanta - Alabama, Florida, Georgia, North Carolina, South Carolina, Virginia, West Virginia  Memphis - Arkansas, Kentucky, Louisiana, Mississippi, Tennessee  Chicago - Illinois, Indiana, Michigan, Ohio, Wisconsin   Kansas City - Iowa, Kansas, Minnesota, Missouri, Nebraska, North Dakota, South Dakota  Dallas - Colorado, New Mexico, Oklahoma, Texas  San Francisco - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#)]
    pub fdicregn: Option<String>,

    #[schemars(description = r#"Federal Reserve District - The supervisory FDIC office assigned to the institution. There are twelve Federal Reserve Districts, with two Districts serving one state in some instances. The list of Federal Reserve Districts and their respective states are as follows: Boston - Connecticut, Maine, Massachusetts, New Hampshire, Rhode Island, Vermont New York - Connecticut, New Jersey, New York, Puerto Rico U.S. Virgin Islands Philadelphia - Delaware, New Jersey, Pennsylvania Cleveland - Kentucky, Ohio, Pennsylvania, West Virginia Richmond - Maryland, North Carolina, South Carolina, Virginia, West Virginia Atlanta - Alabama, Florida, Georgia, Louisiana, Mississippi, Tennessee Chicago - Illinois, Indiana, Iowa, Michigan, Wisconsin St. Louis - Arkansas, Illinois, Indiana, Kentucky, Mississippi, Missouri, Tennessee Minneapolis - Michigan, Minnesota, Montana, North Dakota, South Dakota, Wisconsin Kansas City - Colorado, Kansas, Missouri, Nebraska, New Mexico, Oklahoma, Wyoming Dallas - Louisiana, New Mexico, Texas San Francisco> - Alaska, American Samoa, Arizona, California, Guam, Hawaii, Idaho, Nevada, Oregon, States of Micronesia, Utah, Washington"#)]
    pub fdicsupv: Option<String>,

    #[schemars(description = r#"Federal Reserve ID Number - A number used to identify the Federal Reserve district in which the institution is located. 01 – Boston, 02 - New York,03 – Philadelphia, 04 – Cleveland,05 – Richmond,06 – Atlanta,07 – Chicago,08 - St. Louis,09 – Minneapolis,10 - Kansas city,11 – Dallas,12 - San Francisco"#)]
    pub fed: Option<String>,

    #[schemars(description = r#"Federal Reserve ID Number - A unique number assigned by the Federal Reserve board as the entity's unique identifier"#)]
    pub fed_rssd: Option<String>,

    #[schemars(description = r#"Federal charter flag - A flag used to indicate whether the institution is chartered by an agent of the federal government."#)]
    pub fedchrtr: Option<String>,

    #[schemars(description = r#"FDIC Field Office - The FDIC Field Office where an institution is physically located."#)]
    pub fldoff: Option<String>,

    #[schemars(description = r#"FFIEC Call Report 31 Filer - A flag (1=yes,0=no) that indicates whether and institution filed an FFIEC 031 Call Report. Commercial banks with domestic and foreign offices are required to file such a report."#)]
    pub form31: Option<String>,

    #[schemars(description = r#"Bank Holding Company Type - A flag used to indicate whether an institution is a member of a multibank holding company 1=yes, 0=no"#)]
    pub hctmult: Option<String>,

    #[schemars(description = r#"Insured offices of foreign banks - Includes Bank Insurance Fund insured branches in the U.S. established by banks chartered and headquartered in foreign countries.  These institutions are regulated by one of the three Federal commercial bank regulators and submit financial data to the Federal Reserve."#)]
    pub iba: Option<String>,

    #[schemars(description = r#"Inactive - Institutions that are currently closed but were once insured by the FDIC."#)]
    pub inactive: Option<String>,

    #[schemars(description = r#"Primary Insurance Agency - Abbreviated primary insurance agency."#)]
    pub insagnt1: Option<String>,

    #[schemars(description = r#"Secondary Insurance Fund - As a result of the establishment of a single Deposit Insurance Fund (DIF) effective April 1, 2006, the Secondary Insurance fund is no longer applicable. previously both bif and saif bank insurance fund - institutions that are members of the bank insurance fund savings association insurance fund - Institutions that are members of the Savings Association Insurance Fund"#)]
    pub insagnt2: Option<String>,

    #[schemars(description = r#"Bank Insurance Fund - Institutions who are members of the Bank Insurance Fund. As of April 1, 2006 BIF was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF member institutions, that are still active or open, are now insured members of DIF."#)]
    pub insbif: Option<String>,

    #[schemars(description = r#"Insured commercial banks - Includes commercial banks insured by the FDIC.  These institutions are regulated by one of the three Federal commercial bank regulators (FDIC, Federal Reserve Board, or Office of the Comptroller of the Currency).  They submit financial reports to the Federal Reserve (state member banks) or the FDIC (state non-member banks and national banks)."#)]
    pub inscoml: Option<String>,

    #[schemars(description = r#"Date of Deposit Insurance - The date that an institution obtained federal deposit insurance."#)]
    pub insdate: Option<String>,

    #[schemars(description = r#"Date of Dropped Deposit Insurance - The date that an institution relinquished federal deposit insurance."#)]
    pub insdropdate_raw: Option<String>,

    #[schemars(description = r#"Date of Dropped Deposit Insurance - The date that an institution relinquished federal deposit insurance."#)]
    pub insdropdate: Option<String>,

    #[schemars(description = r#"Deposit Insurance Fund member - A flag used to indicate whether an institution is insured under the Deposit Insurance Fund (DIF).  As of April 1, 2006 the Bank Insurance Fund (BIF) was merged together with the Savings Institution Insurance Fund (SAIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured BIF and SAIF member institutions that are still active or open are now insured members of DIF.    0 = No, not DIF insured and 1 = Yes, DIF insured.  Note that institutions that became inactive prior to April 1006 will also have zero value."#)]
    pub insdif: Option<String>,

    #[schemars(description = r#"FDIC Insured - Includes institutions insured by the FDIC."#)]
    pub insfdic: Option<f32>,

    #[schemars(description = r#"SAIF Insured - Institutions who are members of the Savings Association Insurance Fund. As of April 1, 2006 SAIF was merged together with the Bank Insurance Fund (BIF) to create a single Deposit Insurance Fund (DIF).  All FDIC insured SAIF member institutions, that are still active or open, are now insured members of DIF."#)]
    pub inssaif: Option<String>,

    #[schemars(description = r#"Insured Savings Institution - Includes savings institutions insured by the FDIC that operate under state or federal banking codes applicable to thrift institutions.  These institutions are regulated by and submit financial reports to one of two Federal regulators (FDIC or Office of Thrift Supervision)."#)]
    pub inssave: Option<String>,

    #[schemars(description = r#"Agricultural lending institution indicator - An indicator specifying whether an institution is primarily an agricultural lending institution."#)]
    pub instag: Option<String>,

    #[schemars(description = r#"Credit Card Institutions - Institutions with total loans greater than 50% of total assets and credit card loans greater than 50% of total loans, including loans that have been securitized and sold."#)]
    pub instcrcd: Option<String>,

    #[schemars(description = r#"Location Address Latitude - The latitude of the physical address."#)]
    pub latitude: Option<f32>,

    #[schemars(description = r#"Law Sasser Flag - A flag, yes=1 and no=0 associated with OTS supervised savings associations that converted their charter to that of a commercial or savings bank.  Converted associations remain members of the SAIF, but they become subject to supervision by one of the three federal banking agencies. Not Applicable as of March 31, 2006."#)]
    pub law_sasser_flg: Option<String>,

    #[schemars(description = r#"Location Address Longitude - The longitude of the physical address."#)]
    pub longitude: Option<f32>,

    #[schemars(description = r#"Minority Status Code - A numeric flag used to indicate whether an institution is primarily a minority owned institution."#)]
    pub mdi_status_code: Option<String>,

    #[schemars(description = r#"Minority Status Description - A descriptive flag used to indicate type of minority owned institution."#)]
    pub mdi_status_desc: Option<String>,

    #[schemars(description = r#"Metropolitan Statistical Area (MSA) - The Metropolitan Statistical Areas based on Census Bureau data, as defined by the US Office of Management (OMB) prior to the year 2000."#)]
    pub msa: Option<String>,

    #[schemars(description = r#"Metropolitan Statistical Area Number - The Metropolitan Statistical Area Number (MSA_NO) in which the institution is physically located. The Office of Management and Budget defines MSAs in terms of entire counties surrounding central cities, except in the six New England states where they are defined in terms of cities and towns within counties. Before 200 standards"#)]
    pub msa_no: Option<String>,

    #[schemars(description = r#"Ownership Type - Banking institutions fall into one of two ownership types, stock or non-stock. An institution which sells stock to raise capital is called a stock institution. It is owned by the shareholders who benefit from profits earned by the institution. A non-stock institution, or mutual institution, is owned and controlled solely by its depositors. A mutual does not issue capital stock."#)]
    pub mutual: Option<String>,

    #[schemars(description = r#"Institution name (Search-Eligible) - The legal title or name of the institution. This field can be used for search and filtering."#)]
    pub name: Option<String>,

    #[schemars(description = r#"Bank Holding Company (Regulatory Top Holder) - Regulatory top holder is assigned by the Federal Reserve Board based on ownership and control percentages. Note: Information on bank holding companies is only as of quarter-end. Regulatory top holder is any company that directly or indirectly owns, controls or has power to vote 25 percent or more of a bank's or direct holding company's shares or  controls in any manner the election of a majority of the directors or trustees of a bank or direct holding company or  exercises a controlling influence over the management or policies of a bank or direct holding company.   Information on Thrift Holding Companies that own Savings Associations but do not own banks is not currently available in the ID System.  Source:  Federal Reserve Board National Information Center data base."#)]
    pub namehcr: Option<String>,

    #[schemars(description = r#"Net income - Net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#)]
    pub netinc: Option<f32>,

    #[schemars(description = r#"Net income - quarterly - Quarterly net interest income plus total noninterest income plus realized gains (losses) on securities and extraordinary items, less total noninterest expense, loan loss provisions and income taxes."#)]
    pub netincq: Option<f32>,

    #[schemars(description = r#"New certificate number - A new certificate number of an already existing FDIC-insured institution resulting from either a merger or an acquisition."#)]
    pub newcert: Option<String>,

    #[schemars(description = r#"Oakar Institutions - A member of one insurance fund that acquired deposits insured by the other fund, where that portion of the buyer's deposits remained insured by, and assessable by, the other fund."#)]
    pub oakar: Option<String>,

    #[schemars(description = r#"Office of the Comptroller - The Office of the Comptroller of the Currency (OCC) District in  which the institution is physically located. The six OCC Districts  and their respective states are: Northeast - Connecticut, Delaware,  District of Columbia, Maine, Maryland, Massachusetts, New Hampshire,  New Jersey, New York, Pennsylvania, Puerto Rico, Rhode Island,  Vermont, U.S. Virgin Islands Southeast - Alabama, Florida, Georgia,  Mississippi, North Carolina, South Carolina, Tennessee, Virginia,  West Virginia Central - Illinois, Indiana, Kentucky, Michigan,  Ohio, Wisconsin Midwest - Iowa, Kansas, Minnesota, Missouri,  Nebraska, North Dakota, South Dakota Southwest - Arkansas,  Louisiana, New Mexico, Oklahoma, Texas West - Alaska, American  Samoa, Arizona, California, Colorado, Guam, Hawaii, Idaho, Montana,  Nevada, Oregon, States of Micronesia, Utah, Washington, Wyoming"#)]
    pub occdist: Option<String>,

    #[schemars(description = r#"Number of Domestic Offices - The number of domestic offices (including headquarters) operated by active institutions in the 50 states of the U.S.A."#)]
    pub offdom: Option<f32>,

    #[schemars(description = r#"Number of Foreign Offices - The number of foreign offices (outside the U.S.) operated by the institution."#)]
    pub offfor: Option<f32>,

    #[schemars(description = r#"Office - A branch/office is any location, or facility, of a financial institution, including its main office, where deposit accounts are opened, deposits are accepted, checks paid, and loans granted. Some branches include, but are not limited to, brick and mortar locations, detached or attached drive-in facilities, seasonal offices, offices on military bases or government installations, paying/receiving stations or units, nondeposit offices, Internet and PhoneBanking locations where a customer can open accounts, make deposits and borrow money. A branch does not include Automated Teller Machines (ATM), Consumer Credit Offices, Contractual Offices, Customer Bank Communication Terminals (CBCT), Electronic Fund Transfer Units (EFTU), and Loan Production Offices Summary of Deposits information is required for each insured office located in any State, the District of Columbia, the Commonwealth of Puerto Rico or any U.S. territory or possession such as Guam or the U.S. Virgin Islands, without regard to the location of the main office."#)]
    pub offices: Option<f32>,

    #[schemars(description = r#"Number of US Offices - The number of offices operated by an FDIC-insured institution in all commonwealths and territories of the US, along with those in freely associated states under the Compact of Free Association"#)]
    pub offoa: Option<f32>,

    #[schemars(description = r#"OTS District - Office of Thrift Supervision (OTS) District - No longer used as of 7/21/11"#)]
    pub otsdist: Option<String>,

    #[schemars(description = r#"Office of Thrift Supervision Region - Prior to 7/21/11, the Office of Thrift Supervision (OTS) Region in  which the institution is physically located. The five OTS Regions  and their respective states are: Northeast - Connecticut, Delaware,  Maine, Massachusetts, New Hampshire, New Jersey, New York,  Pennsylvania, Rhode Island, Vermont, West Virginia Southeast -  Alabama, District of Columbia, Florida, Georgia, Maryland, North  Carolina, Puerto Rico, South Carolina, U.S. Virgin Islands, Virginia  Central - Illinois, Indiana, Kentucky, Michigan, Ohio, Tennessee,  Wisconsin Midwest - Arkansas, Colorado, Iowa, Kansas, Louisiana,  Minnesota, Mississippi, Missouri, Nebraska, New Mexico, North  Dakota, Oklahoma, South Dakota, Texas West - Alaska, American Samoa,  Arizona, California, Guam, Hawaii, Idaho, Montana, Nevada, States of  Micronesia, Oregon, Utah, Washington, Wyoming"#)]
    pub otsregnm: Option<String>,

    #[schemars(description = r#"Directly owned by another bank (CERT) - The PARCERT number identifies the subsidiary institutions parent certificate number. Beginning in March 1997, both the Thrift Financial Reports and Call Reports are completed on a fully consolidated basis.  Previously, the consolidation of subsidiary depository institutions was prohibited.  Now, parent institutions are required to file consolidated reports, while their subsidiary financial institutions are still required to file separate reports."#)]
    pub parcert: Option<String>,

    #[schemars(description = r#"Last Structure Change Process Date - A date indicating when an institution's change/event is processed."#)]
    pub procdate: Option<String>,

    #[schemars(description = r#"Quarterly Banking Profile Commercial Bank Region - The Quarterly Banking Profile (QBP) Commercial Bank Region in which the institution is physically located. Select from a drop down box. regional breakdown. group data by qbp region is only available for insured commercial banks and insured savings institutions and NOT All Insured Institutions, Insured Commercial Banks by asset size and Insured Savings Institutions by asset size."#)]
    pub qbprcoml: Option<String>,

    #[schemars(description = r#"Primary Regulator - A code indicating the federal regulatory agency that provides primary supervision over an institution. OCC=Office of the Comptroller of Currency; FDIC=Federal Deposit Insurance Corporation; FRB=Federal Reserve Board; NCUA=National Credit Union Association; OTS=Office of Thrift Supervision."#)]
    pub regagnt: Option<String>,

    #[schemars(description = r#"Secondary Regulator - A code indicating the federal regulatory agency that provides secondary supervision over an institution. CFPB = Consumer Financial Protection Bureau; OTS=Office of Thrift Supervision."#)]
    pub regagent2: Option<String>,

    #[schemars(description = r#"Report Date - The last day of the financial reporting period selected."#)]
    pub repdte: Option<String>,

    #[schemars(description = r#"Report Date - The financial reporting period selected in CCYYMM format."#)]
    pub risdate: Option<String>,

    #[schemars(description = r#"Return on assets (ROA) - Net income after taxes and extraordinary items (annualized) as a percent of average total assets."#)]
    pub roa: Option<f32>,

    #[schemars(description = r#"Pretax return on assets - Annualized pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#)]
    pub roaptx: Option<f32>,

    #[schemars(description = r#"Quarterly Pretax return on assets - Quarterly pre-tax net income as a percent of average assets. Note: Includes extraordinary items and other adjustments, net of taxes."#)]
    pub roaptxq: Option<f32>,

    #[schemars(description = r#"Quarterly return on assets - Quarterly net income after taxes and extraordinary items as a percent of average total assets."#)]
    pub roaq: Option<f32>,

    #[schemars(description = r#"Return on Equity (ROE) - Annualized net income as a percent of average equity on a consolidated basis.     Note: If retained earnings are  negative, the ratio is shown as NA."#)]
    pub roe: Option<f32>,

    #[schemars(description = r#"Quarterly return on equity - Quarterly net income (including gains or losses on securities and extraordinary items) as a percentage of average total equity capital."#)]
    pub roeq: Option<f32>,

    #[schemars(description = r#"RSSDID - High Regulatory Holder - The unique number assigned by the Federal Reserve Board to the regulatory high holding company of the institution."#)]
    pub rssdhcr: Option<String>,

    #[schemars(description = r#"Run Date - The day the institution information was updated."#)]
    pub rundate: Option<String>,

    #[schemars(description = r#"Sasser Institutions - OTS supervised savings associations that converted their charter to that of a commercial or savings bank.  Converted associations remain members of the SAIF, but they become subject to supervision by one of the three federal banking agencies. Not Applicable as of March 31, 2006."#)]
    pub sasser: Option<String>,

    #[schemars(description = r#"Asset Concentration Hierarchy - An indicator of an institution's primary specialization in terms of asset concentration"#)]
    pub specgrp: Option<f32>,

    #[schemars(description = r#"Specialization Group - Name associated with the numeric indicator (SPECGRP) of an institution's primary specialization in terms of asset concentration"#)]
    pub specgrpn: Option<String>,

    #[schemars(description = r#"State Alpha code - The state abbreviation of the location of the institution's main office."#)]
    pub stalp: Option<String>,

    #[schemars(description = r#"Regulatory holding company state location - State location of the regulatory high holding company (either direct or indirect owner)."#)]
    pub stalphcr: Option<String>,

    #[schemars(description = r#"State Charter - A flag (1=yes;0=no) that indicates if an institution is state chartered."#)]
    pub stchrtr: Option<String>,

    #[schemars(description = r#"State and county number - A five digit number representing the state and county in which the institution is physically located.  The first two digits represent the FIPS state numeric code and the last three digits represent the FIPS county numeric code."#)]
    pub stcnty: Option<String>,

    #[schemars(description = r#"State Name - State in which the the institution is physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#)]
    pub stname: Option<String>,

    #[schemars(description = r#"State Number - The Federal Information Processing Standard code used to identify states"#)]
    pub stnum: Option<String>,

    #[schemars(description = r#"Subchapter S Corporations - The Small Business Job Protection Act of 1996 changed the Internal Revenue Code to allow financial institutions to elect Subchapter S corporation status, beginning in 1997. Banks are required to indicate on the Call Report whether there is currently in effect an election to file under Subchapter S. Thrifts have a similar requirement as of March 1998.  The most important IRS requirements to elect and maintain Subchapter S status are: There can be no more than 75 eligible shareholders and no more than one class of stock. (In general, shareholders can only be individuals, estates, and certain types of trusts. Certain retirement plans and charitable organizations will be eligible in 1998.) All shareholders must consent.  Banks and thrifts converting to Subchapter S status must use the specific charge-off method for tax purposes rather than the reserve method of accounting for bad debts and recapture tax bad debt reserves over a period of six years, if the reserve method had been used prior to conversion. (Note: even though the specific charge-off method is required for tax purposes, an adequate allowance for loan and lease losses must still be maintained on the financial statements and Call Reports.) Banks and thrifts are subject to a built-in gains (BIG) tax, if the aggregate fair market value of assets is greater than their aggregate adjusted bases on the date of conversion to Subchapter S status.     [Banks are required to indicate separately on the Call Report in December of each year, the deferred portion of income taxes reported in net income. For Subchapter S banks, some or all of their deferred tax assets and liabilities may be eliminated upon conversion to Subchapter S status; however, deferred taxes related to the BIG tax and the recapture of bad debt reserves must be recognized.].   A Subchapter S corporation is treated as a pass-through entity, similar to a partnership, for federal income tax purposes. It is generally not subject to any federal income taxes at the corporate level. Its taxable income flows through to its shareholders in proportion to their stock ownership, and the shareholders generally pay federal income taxes on their share of this taxable income. This can have the effect of reducing institutions' reported income tax expense and increasing their after-tax earnings..   The election of Subchapter S status may result in an increase in shareholders' personal tax liabilities. Therefore, S corporations typically increase the amount of earnings distributed as dividends to compensate for higher personal taxes."#)]
    pub subchaps: Option<String>,

    #[schemars(description = r#"Supervisory Region Number - A two-digit number indicating the FDIC Supervisory Division or Region. 02 = New York; 05 = Atlanta; 09 = Chicago; 11 = Kansas City; 13 = Dallas; 14 = San Francisco; 16 = Office of Complex Financial Institutions (CFI)"#)]
    pub suprv_fd: Option<String>,

    #[schemars(description = r#"Web Site URL 01 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te01n528: Option<String>,

    #[schemars(description = r#"Web Site URL 02 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te02n528: Option<String>,

    #[schemars(description = r#"Web Site URL 03 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te03n528: Option<String>,

    #[schemars(description = r#"Web Site URL 04 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te04n528: Option<String>,

    #[schemars(description = r#"Web Site URL 05 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te05n528: Option<String>,

    #[schemars(description = r#"Web Site URL 06 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te06n528: Option<String>,

    #[schemars(description = r#"Web Site URL 07 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te07n528: Option<String>,

    #[schemars(description = r#"Web Site URL 08 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te08n528: Option<String>,

    #[schemars(description = r#"Web Site URL 09 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te09n528: Option<String>,

    #[schemars(description = r#"Web Site URL 10 - URL of other public-facing internet web site the reporting institution uses to accept or solicit deposits from the public"#)]
    pub te10n528: Option<String>,

    #[schemars(description = r#"Trade Name 01 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te01n529: Option<String>,

    #[schemars(description = r#"Trade Name 02 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te02n529: Option<String>,

    #[schemars(description = r#"Trade Name 03 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te03n529: Option<String>,

    #[schemars(description = r#"Trade Name 04 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te04n529: Option<String>,

    #[schemars(description = r#"Trade Name 05 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te05n529: Option<String>,

    #[schemars(description = r#"Trade Name 06 - Trade name other than the institution's legal name used to identify one of the institution's physical offices at which deposits are accepted or solicited from the public"#)]
    pub te06n529: Option<String>,

    #[schemars(description = r#" - Beyond having trust powers granted and exercised, institutions with fiduciary assets accounts, income, or other reportable fiduciary related service"#)]
    pub tract: Option<String>,

    #[schemars(description = r#"Trust Powers - A number corresponding to a valid type of trust power that an institution can possess and exercise. 00=Trust Powers Not Known; 10=Full Trust Powers Granted; 11=Full Trust Powers Granted, Exercised; 12=Full Trust Powers Granted, Not Exercised; 20=Limited Trust Powers Granted; 21=Limited Trust Powers Granted, Exercised; 30=Trust Powers Not Granted; 31=Trust Powers Not Granted, But Exercised; 40=Trust Powers Grandfathered"#)]
    pub trust: Option<String>,

    #[schemars(description = r#"Ultimate Cert - The cert number of the last successor or acquirer of the institution"#)]
    pub ultcert: Option<String>,

    #[schemars(description = r#"FDIC's unique number - FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#)]
    pub uninum: Option<String>,

    #[schemars(description = r#"Primary Internet Web Address - The primary internet web address is the public internet site obtained from the most recent FFIEC Call Report (CALL) for commercial banks or from the supplemental information for Thrift Financial Reporters (TFR). The primary internet web address is included only for those institutions reporting an address on the most recent FFIEC Call Report or Thrift Financial Report."#)]
    pub webaddr: Option<String>,

    #[schemars(description = r#"Zip Code - The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#)]
    pub zip: Option<String>,

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct InstitutionsResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
    pub totals: ResponseTotals,
}

impl IntoContents for InstitutionsResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize InstitutionsResponse to Content")]
    }
}

/// FDIC BankFind API `/institutions` endpoint handler
/// Get Financial Institutions
/// Returns a list of financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter for the bank search. All values must be entered in UPPERCASE.
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
STALP:IA AND ACTIVE:1 - `search` (String, optional): Flexible text search against institution records - currently only supporting name search. 
Search supports text search and fuzzy matching, as opposed to filters that are exact matches. All values must be entered in UPPERCASE.
Examples:
* Search by name
`NAME: Island`
* Search by name (fuzzy match)
`NAME: Iland` - `fields` (String, optional): Comma delimited list of fields to search. All values must be entered in UPPERCASE.
ZIP,OFFDOM,CITY,COUNTY,STNAME,STALP,NAME,ACTIVE,CERT,CBSA,ASSET,NETINC,DEP,DEPDOM,ROE,ROA,DATEUPDT,OFFICES - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
OFFICES - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /institutions
Parameters: InstitutionsParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Structure"#]
pub async fn institutions_handler(config: &FdicApiConfig, params: &InstitutionsParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "institutions",
        method = "GET",
        path = "/institutions",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response::<_, InstitutionsResponse>(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp.and_then(|r| r.into_call_tool_result())
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
            active: None,
            address: None,
            asset: None,
            bkclass: None,
            cb: None,
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
            certcons: None,
            cfpbeffdte: None,
            cfpbenddte: None,
            cfpbflag: None,
            priorname1: None,
            priorname2: None,
            priorname3: None,
            priorname4: None,
            priorname5: None,
            priorname6: None,
            priorname7: None,
            priorname8: None,
            priorname9: None,
            priorname10: None,
            changec1: None,
            changec2: None,
            changec3: None,
            changec4: None,
            changec5: None,
            changec6: None,
            changec7: None,
            changec8: None,
            changec9: None,
            changec10: None,
            changec11: None,
            changec12: None,
            changec13: None,
            changec14: None,
            changec15: None,
            charter: None,
            chrtagnt: None,
            city: None,
            cityhcr: None,
            clcode: None,
            cmsa_no: None,
            cmsa: None,
            conserve: None,
            county: None,
            csa: None,
            csa_no: None,
            csa_flg: None,
            dateupdt: None,
            denovo: None,
            dep: None,
            depdom: None,
            docket: None,
            effdate: None,
            endefymd: None,
            eq: None,
            estymd: None,
            fdicdbs: None,
            fdicregn: None,
            fdicsupv: None,
            fed: None,
            fed_rssd: None,
            fedchrtr: None,
            fldoff: None,
            form31: None,
            hctmult: None,
            iba: None,
            inactive: None,
            insagnt1: None,
            insagnt2: None,
            insbif: None,
            inscoml: None,
            insdate: None,
            insdropdate_raw: None,
            insdropdate: None,
            insdif: None,
            insfdic: None,
            inssaif: None,
            inssave: None,
            instag: None,
            instcrcd: None,
            latitude: None,
            law_sasser_flg: None,
            longitude: None,
            mdi_status_code: None,
            mdi_status_desc: None,
            msa: None,
            msa_no: None,
            mutual: None,
            name: None,
            namehcr: None,
            netinc: None,
            netincq: None,
            newcert: None,
            oakar: None,
            occdist: None,
            offdom: None,
            offfor: None,
            offices: None,
            offoa: None,
            otsdist: None,
            otsregnm: None,
            parcert: None,
            procdate: None,
            qbprcoml: None,
            regagnt: None,
            regagent2: None,
            repdte: None,
            risdate: None,
            roa: None,
            roaptx: None,
            roaptxq: None,
            roaq: None,
            roe: None,
            roeq: None,
            rssdhcr: None,
            rundate: None,
            sasser: None,
            specgrp: None,
            specgrpn: None,
            stalp: None,
            stalphcr: None,
            stchrtr: None,
            stcnty: None,
            stname: None,
            stnum: None,
            subchaps: None,
            suprv_fd: None,
            te01n528: None,
            te02n528: None,
            te03n528: None,
            te04n528: None,
            te05n528: None,
            te06n528: None,
            te07n528: None,
            te08n528: None,
            te09n528: None,
            te10n528: None,
            te01n529: None,
            te02n529: None,
            te03n529: None,
            te04n529: None,
            te05n529: None,
            te06n529: None,
            tract: None,
            trust: None,
            ultcert: None,
            uninum: None,
            webaddr: None,
            zip: None,
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
