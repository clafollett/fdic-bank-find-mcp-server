//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/summary` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// Auto-generated parameters struct for `/summary` endpoint.
/// Spec: summary_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#]
    pub agg_by: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#]
    pub agg_term_fields: Option<String>,
    #[doc = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#]
    pub agg_sum_fields: Option<String>,
    #[doc = r#"The limit on how many aggregated results will be displayed."#]
    pub agg_limit: Option<i32>,
    #[doc = r#"The field by which the max value is desired."#]
    pub max_value: Option<String>,
    #[doc = r#"The field that will be used to determine unique records, similar to a primary key (i.e. CERT). All values must be entered in UPPERCASE."#]
    pub max_value_by: Option<String>,
}

// Implement QueryParameters for generic handler
impl QueryParameters for SummaryParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ALLOTHER",
        "ALSONEW",
        "ASSET",
        "BANKS",
        "BKPREM",
        "BRANCHES",
        "BRANCHIN",
        "BRO",
        "BRWDMONY",
        "CB_SI",
        "CHARTOTH",
        "CHBAL",
        "CHBALI",
        "CHRTREST",
        "COMBOASS",
        "COMBOS",
        "CONS",
        "CORPBNDS",
        "COUNT",
        "CRLNLS",
        "DDT",
        "DEP",
        "DEPDOM",
        "DEPFOR",
        "DEPI",
        "DEPIFOR",
        "DEPNI",
        "DEPNIFOR",
        "DRLNLS",
        "EAMINTAN",
        "EDEP",
        "EDEPDOM",
        "EDEPFOR",
        "EEREPP",
        "EFHLBADV",
        "EFREPP",
        "EINTEXP",
        "EINTEXP2",
        "ELNATR",
        "EOTHNINT",
        "EPREMAGG",
        "EQ",
        "EQCDIV",
        "EQCDIVC",
        "EQCDIVP",
        "EQCS",
        "EQDIV",
        "EQNM",
        "EQNWCERT",
        "EQOTHCC",
        "EQPP",
        "EQSUR",
        "EQUPTOT",
        "ESAL",
        "ESUBND",
        "EXTRA",
        "FD_BIF",
        "FD_SAIF",
        "FREPO",
        "FREPP",
        "ICHBAL",
        "IFEE",
        "IFREPO",
        "IGLSEC",
        "ILNDOM",
        "ILNFOR",
        "ILNLS",
        "ILNS",
        "ILS",
        "INTAN",
        "INTBAST",
        "INTBLIB",
        "INTINC",
        "INTINC2",
        "IRAKEOGH",
        "ISC",
        "ISERCHG",
        "ITAX",
        "ITAXR",
        "ITRADE",
        "LIAB",
        "LIABEQ",
        "LIQASSTD",
        "LIQUNASS",
        "LNAG",
        "LNALLOTH",
        "LNATRES",
        "LNAUTO",
        "LNCI",
        "LNCON",
        "LNCONOT1",
        "LNCONOTH",
        "LNCRCD",
        "LNDEP",
        "LNLS",
        "LNLSGR",
        "LNLSNET",
        "LNMOBILE",
        "LNMUNI",
        "LNRE",
        "LNREAG",
        "LNRECONS",
        "LNREDOM",
        "LNREFOR",
        "LNRELOC",
        "LNREMULT",
        "LNRENRES",
        "LNRERES",
        "LNRESRE",
        "LNSP",
        "LS",
        "MERGERS",
        "MISSADJ",
        "MTGLS",
        "NALNLS",
        "NCHGREC",
        "NCLNLS",
        "NETIMIN",
        "NETINC",
        "NEWCOUNT",
        "NEW_CHAR",
        "NEW6_1",
        "NEW9_1",
        "NEW10_1",
        "NEW10_2",
        "NEW10_3",
        "NEW11_1",
        "NEW14_1",
        "NEW14_2",
        "NEW14_3",
        "NEW14_4",
        "NEW15_1",
        "NEW15_2",
        "NEW15_3",
        "NEW15_4",
        "NEW15_5",
        "NEW15_7",
        "NEW16_1",
        "NEW16_2",
        "NIM",
        "NONII",
        "NONIX",
        "NTLNLS",
        "NTR",
        "NTRTIME",
        "NTRTMLG",
        "NUMEMP",
        "OEA",
        "OFFICES",
        "OINTBOR",
        "OINTEXP",
        "OINTINC",
        "OONONII",
        "ORE",
        "ORET",
        "OT_BIF",
        "OT_SAIF",
        "OTHASST",
        "OTHBFHLB",
        "OTHBORR",
        "OTHEQ",
        "OTHER",
        "OTHLIAB",
        "OTHNBORR",
        "OTLNCNTA",
        "PAID_OFF",
        "P3LNLS",
        "P9LNLS",
        "PTXNOINC",
        "REL_CO",
        "SAVINGS",
        "SC",
        "SCAGE",
        "SCEQ",
        "SCMTGBK",
        "SCMUNI",
        "SCMV",
        "SCRES",
        "SCUS",
        "SCUSA",
        "SCUST",
        "STNAME",
        "STNUM",
        "SUBLLPF",
        "SUBND",
        "TINTINC",
        "TOCHRT",
        "TOFAIL",
        "TOINTEXP",
        "TOMERG",
        "TORTC",
        "TOTAL",
        "TOT_FDIC",
        "TOT_OTS",
        "TOT_SAVE",
        "TPD",
        "TRADE",
        "TRADES",
        "TRN",
        "UNASSIST",
        "UNINC",
        "UNIT",
        "YEAR",
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
        if let Some(val) = &self.max_value {
            query.insert("max_value".to_string(), val.to_string());
        }
        if let Some(val) = &self.max_value_by {
            query.insert("max_value_by".to_string(), val.to_string());
        }
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/summary` endpoint.
/// Spec: summary_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryProperties {
    #[doc = r#"Title: All Other Loans"#]
    #[doc = r#"Description: All Other Loans"#]
    #[serde(rename="ALLOTHER")]
    pub allother: Option<i32>,

    #[doc = r#"Title: New Charters to Absorb Another Charter"#]
    #[doc = r#"Description: New savings institution charter created to absorb any other type of charter in its first quarter of operation."#]
    #[serde(rename="alsonew")]
    pub alsonew: Option<i32>,

    #[doc = r#"Title: Total Assets"#]
    #[doc = r#"Description: Total Assets On A Consolidated Basis
Note: For Banks With Foreign Operations Data For March &
September Of 1973 Through 1975 Are Reported On A
Domestic Basis"#]
    #[serde(rename="ASSET")]
    pub asset: Option<i32>,

    #[doc = r#"Title: Total Commercial Banks (Filing Y/E Call)"#]
    #[doc = r#"Description: Total Insured Commercial Banks filing 12/31 fincncial report  (See Notes to User for definition of commercial bank)"#]
    #[serde(rename="BANKS")]
    pub banks: Option<i32>,

    #[doc = r#"Title: Bank Premises and Equipment"#]
    #[doc = r#"Description: Premises and Fixed Assets
Note:
(1) Premises and Fixed Assets (Including Capitalized Leases) On A Consolidated Basis
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="BKPREM")]
    pub bkprem: Option<i32>,

    #[doc = r#"Title: Total Branches"#]
    #[doc = r#"Description: Branches include all offices of a bank, other than its head office, at which deposits are received, checks paid or money lent. Banking facilities separate from a banking house, banking facilities at government installations, offices, agencies, paying or receiving stations, drive-in facilities and other facilities operated for limited purposes are defined as branches under the FDI Act (see Notes to User)"#]
    #[serde(rename="BRANCHES")]
    pub branches: Option<i32>,

    #[doc = r#"Title: Banks with Branches"#]
    #[doc = r#"Description: Banks with branches are institutions that operate one or more offices at which deposits are received or other banking business conducted in addition to the main or head office."#]
    #[serde(rename="BRANCHIN")]
    pub branchin: Option<i32>,

    #[doc = r#"Title: Memo: Brokered Deposits"#]
    #[doc = r#"Description: Borrowed Deposits (Represents funds which the reporting bank obtains, directly or indirectly, by or through any deposit broker for deposit into one or more deposit accounts. Includes both those in which the entire beneficial interest in a given bank deposit account or investment is held by a single depositor and those in which deposit broker sells participation in a given bank deposit account or instrument to one or more investors)."#]
    #[serde(rename="BRO")]
    pub bro: Option<i32>,

    #[doc = r#"Title: Borrowed Funds"#]
    #[doc = r#"Description: Borrowed Funds - (1969-Present -- Represents Federal Funds purchase. securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premsises.)"#]
    #[serde(rename="BRWDMONY")]
    pub brwdmony: Option<i32>,

    #[doc = r#"Title: Commercial Banks (CB) vs. Savings Institution (SI) (Search-Eligible)"#]
    #[doc = r#"Description: Differentiates the summarised data between the Commercial Banks and the Savings Institutions This field can be used for search and filtering."#]
    #[serde(rename="CB_SI")]
    pub cb_si: Option<String>,

    #[doc = r#"Title: Charter Transfers from Commercial Banks"#]
    #[doc = r#"Description: Represents the transfer of a commercial bank to a savings institution charter that meets the definition of a thrift (see Notes to Table SI-1) and has applied for and received FDIC insurance (BIF or SAIF)."#]
    #[serde(rename="chartoth")]
    pub chartoth: Option<i32>,

    #[doc = r#"Title: Cash & Due from Depository Institutions"#]
    #[doc = r#"Description: Total Cash and Balances Due From Depository Institutions Which Include Both Noninterest-Bearing and Interest-Bearing Deposits On A Consolidated Basis
Note:
(1): Additional Detail Can Be Found On Schedule Rc-A
(2) For Banks With Foreign Operations Data For March and September 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="CHBAL")]
    pub chbal: Option<i32>,

    #[doc = r#"Title: Interest Earning Balances"#]
    #[doc = r#"Description: Interest-Bearing Balances Due From Depository Institutions On A Consolidated Basis
Note: Additional Detail Can Be Found On Schedule Rc-A"#]
    #[serde(rename="CHBALI")]
    pub chbali: Option<i32>,

    #[doc = r#"Title: Non-insured Becoming insured"#]
    #[doc = r#"Description: Represents the transfer of an existing institution that does not have deposit insurance to a savings institution charter with FDIC insurance from BIF or SAIF. Examples of such institutions include Trust Banks and savings institutions with state deposit insurance that apply for and receive FDIC insurance"#]
    #[serde(rename="chrtrest")]
    pub chrtrest: Option<i32>,

    #[doc = r#"Title: Assisted Mergers with Thrifts"#]
    #[doc = r#"Description: Represents the absorption of a failing savings institution by another savings institution with assistance from either the BIF or SAIF. (Included are RTC Accelerated Resolution Program (ARP) assisted mergers. These institutions were not placed in RTC conservatorship.)"#]
    #[serde(rename="comboass")]
    pub comboass: Option<i32>,

    #[doc = r#"Title: Unassisted Mergers/Consolidations of Thrifts"#]
    #[doc = r#"Description: Represents the absorption of a savings institution charter by another savings institution without assistance. Both institutions may be owned by the same holding company in a consolidation of affiliates."#]
    #[serde(rename="combos")]
    pub combos: Option<i32>,

    #[doc = r#"Title: RTC Conservatorships"#]
    #[doc = r#"Description: Institutions in RTC Conservatorship"#]
    #[serde(rename="CONS")]
    pub cons: Option<i32>,

    #[doc = r#"Title: Other Debt Securities"#]
    #[doc = r#"Description: Other Debt Securities"#]
    #[serde(rename="CORPBNDS")]
    pub corpbnds: Option<i32>,

    #[doc = r#"Title: Total Savings Institutions (Filing Y/E Call)"#]
    #[doc = r#"Description: All FDIC Insured Savings Institutions filing a 12/31 financial report"#]
    #[serde(rename="COUNT")]
    pub count: Option<i32>,

    #[doc = r#"Title: Loan & Lease Recoveries"#]
    #[doc = r#"Description: Total Recoveries Of Loans and Lease Financing Receivables Credited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="CRLNLS")]
    pub crlnls: Option<i32>,

    #[doc = r#"Title: Deposits - Domestic Demand"#]
    #[doc = r#"Description: Total Demand Deposits Included In Total Transaction Accounts Held In Domestic Offices
Note: For Tfr Filers Between June 1989 Through March 1990 Includes Non-interest Bearing Deposits"#]
    #[serde(rename="DDT")]
    pub ddt: Option<i32>,

    #[doc = r#"Title: Total Deposits"#]
    #[doc = r#"Description: Total Deposits On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="DEP")]
    pub dep: Option<i32>,

    #[doc = r#"Title: Total Domestic Deposits"#]
    #[doc = r#"Description: Represents the sum of total deposits, domestic offices only"#]
    #[serde(rename="DEPDOM")]
    pub depdom: Option<i32>,

    #[doc = r#"Title: Total Foreign Deposits"#]
    #[doc = r#"Description: Represents the sum of total deposits in foreign offices"#]
    #[serde(rename="DEPFOR")]
    pub depfor: Option<i32>,

    #[doc = r#"Title: Interest Bearing Deposits"#]
    #[doc = r#"Description: Interest-Bearing Consolidated Office Deposits

Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) Tfr Filers With Less Than $300 Million In Assets and Risk-Based Capital Ratios In Excess Of 12 Percent Are Not Required To File Schedule Cmr Beginning March 1993, However, When Cmr Data Is Either Incorrect Or Not Filed Fts Assumes That All Deposits Are Interest-Bearing
(3) Prior To Receipt Of The 75-Day Tfr Tape All Tfr Filers Deposits Are Assumed To Be Interest-Bearing
(4) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="DEPI")]
    pub depi: Option<i32>,

    #[doc = r#"Title: Foreign Deposits - Interest Bearing"#]
    #[doc = r#"Description: Represents any deposit in foreign offices, whether demand, savings or time, on which the bank pays or accrues interest"#]
    #[serde(rename="DEPIFOR")]
    pub depifor: Option<i32>,

    #[doc = r#"Title: Memo: Deposits - Non-interest Bearing"#]
    #[doc = r#"Description: Represents any deposit on which the bank does not pay or accrue interest"#]
    #[serde(rename="DEPNI")]
    pub depni: Option<i32>,

    #[doc = r#"Title: Foreign Deposits - Non-interest Bearing"#]
    #[doc = r#"Description: Represents any deposit in foreign offices on which the bank does not pay or accrue interest"#]
    #[serde(rename="DEPNIFOR")]
    pub depnifor: Option<i32>,

    #[doc = r#"Title: Loan & Lease Charge-offs"#]
    #[doc = r#"Description: Total Charged-Off Loans and Lease Financing Receivables Debited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="DRLNLS")]
    pub drlnls: Option<i32>,

    #[doc = r#"Title: Memo: Amortization of Intangibles"#]
    #[doc = r#"Description: Goodwill Impairment Losses and Amortization Expense and Impairment Loss For Other Intangible Assets On A Consolidated Basis

Note:
(1) Prior To March 2001, Listed As Memoranda Only and Is Included In All Other Non-interest Expense
(2) Includes Only Amortization Of Goodwill For Tfr Filers"#]
    #[serde(rename="EAMINTAN")]
    pub eamintan: Option<i32>,

    #[doc = r#"Title: Int Exp - Total Deposits"#]
    #[doc = r#"Description: Interest Expense On Total Deposits (Domestic and Foreign) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EDEP")]
    pub edep: Option<i32>,

    #[doc = r#"Title: Int Exp - Deposit in Domestic Offices"#]
    #[doc = r#"Description: Interest Expense On Total Deposits Held In Domestic Offices
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EDEPDOM")]
    pub edepdom: Option<i32>,

    #[doc = r#"Title: Int Exp - Deposits in Foreign Offices"#]
    #[doc = r#"Description: Deposit Interest Expense-For (1976-Present -- Represents all interests on all liabilities reportable as deposits in foreign offices. -- 1934-1975 -- Interest on foreign office deposits is not available. Reports of income were submitted on a domestic only basis.)"#]
    #[serde(rename="EDEPFOR")]
    pub edepfor: Option<i32>,

    #[doc = r#"Title: Fed Funds Purchased/Securities Sold"#]
    #[doc = r#"Description: Represents the gross expenses of all liabilities reportable under this category"#]
    #[serde(rename="EEREPP")]
    pub eerepp: Option<i32>,

    #[doc = r#"Title: Int Exp Oth - Advances From FHLB"#]
    #[doc = r#"Description: Interest Expense and The Amortization Of Any Related Yield Adjustments On Fhlbank Advances
Note: Only Reported By Tfr Filers"#]
    #[serde(rename="EFHLBADV")]
    pub efhlbadv: Option<i32>,

    #[doc = r#"Title: Int Exp - Fed Funds Purchased/Securities Sold"#]
    #[doc = r#"Description: Interest Expense On Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis (Prior To March 1997 Was On A Consolidated Basis In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S)
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EFREPP")]
    pub efrepp: Option<i32>,

    #[doc = r#"Title: Total Interest Expense"#]
    #[doc = r#"Description: Total Interest Expense On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EINTEXP")]
    pub eintexp: Option<i32>,

    #[doc = r#"Title: Eintexp2"#]
    #[doc = r#"Description: Eintexp2"#]
    #[serde(rename="EINTEXP2")]
    pub eintexp2: Option<i32>,

    #[doc = r#"Title: Provision for Loan & Lease Losses"#]
    #[doc = r#"Description: Provision For Loan & Lease Losses On A Consolidated Basis

Note:
(1) Beginning March 2003, Includes The Provision For Allocated Transfer Risk Related To Loans
(2) From March 1997 To December 2000, Defined As The Provision For Credit Losses & Allocated Transfer Risk Reserve Which Includes The Provision For Off-Balance Sheet Credit Losses For Call Report Filers
(3) Prior To March 1997, Defined As The Provision For Loan and Lease Losses & Allocated Transfer Risk
(4) For Tfr Filers, Consists Of The Provision For Loan and Lease Losses
(5) Reflects Net Provision For Losses On Interest-Bearing Assets For Tfr Filers
(6) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ELNATR")]
    pub elnatr: Option<i32>,

    #[doc = r#"Title: All Other Non-interest Expenses"#]
    #[doc = r#"Description: All Other Non-interest Expense On A Consolidated Basis

Note:
(1) Prior To March 2001, Included The Amortization Of Intangible Assets For Call Reporters
(2) Greater Detail Is Provided In Subsequent Data Fields For All Items In Excess Of 10% Of This Item All Other Non-interest Expense On A Consolidated Basis
(3) Does Not Include Losses On Asset Sales For Tfr Filers Beginning June 1996, Such Gains (Losses) Are Included Net In Non-interest Income
(4) Includes Loss On Sale Of Securities Held For Investments For Tfr Filers Between March 1984 Through December 1986
(5) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EOTHNINT")]
    pub eothnint: Option<i32>,

    #[doc = r#"Title: Occupancy Expense"#]
    #[doc = r#"Description: Expenses Of Premises and Fixed Assets (Net Of Rental Income and Excluding Salaries and Employee Benefits and Mortgage Interest) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EPREMAGG")]
    pub epremagg: Option<i32>,

    #[doc = r#"Title: Total Equity"#]
    #[doc = r#"Description: Represents the sum of all capital accounts"#]
    #[serde(rename="EQ")]
    pub eq: Option<i32>,

    #[doc = r#"Title: Total Cash Dividends Declared"#]
    #[doc = r#"Description: Cash Dividends Declared On Common and Preferred Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIV")]
    pub eqcdiv: Option<i32>,

    #[doc = r#"Title: Cash Dividends Declared (Common)"#]
    #[doc = r#"Description: Cash Dividends Declared On Common Stock On A Consolidated Basis

Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIVC")]
    pub eqcdivc: Option<i32>,

    #[doc = r#"Title: Cash Dividends Declared (Preferred)"#]
    #[doc = r#"Description: Cash Dividends Declared On Preferred Stock On A Consolidated Basis
Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIVP")]
    pub eqcdivp: Option<i32>,

    #[doc = r#"Title: Common Stock"#]
    #[doc = r#"Description: Common Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQCS")]
    pub eqcs: Option<i32>,

    #[doc = r#"Title: Total Cash Divident Declared"#]
    #[doc = r#"Description: The total of cash dividends declared on all preferred and common stock during the calendar year, regardless of when payable"#]
    #[serde(rename="EQDIV")]
    pub eqdiv: Option<i32>,

    #[doc = r#"Title: Total Equity Capital"#]
    #[doc = r#"Description: Total Capital (Represents the total of all capital components, including FDIC net worth certificates.)"#]
    #[serde(rename="EQNM")]
    pub eqnm: Option<i32>,

    #[doc = r#"Title: FDIC Net Worth Certificates"#]
    #[doc = r#"Description: Net Worth Certificates Represents The Outstanding Balances Issued To The Fdic In Exchange For Promissory Notes Received From The Fdic On A Consolidated Basis"#]
    #[serde(rename="EQNWCERT")]
    pub eqnwcert: Option<i32>,

    #[doc = r#"Title: Other Capital"#]
    #[doc = r#"Description: Other Capital"#]
    #[serde(rename="EQOTHCC")]
    pub eqothcc: Option<i32>,

    #[doc = r#"Title: Perpetual Preferred Stock"#]
    #[doc = r#"Description: Perpetual Preferred Stock and Related Surplus On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQPP")]
    pub eqpp: Option<i32>,

    #[doc = r#"Title: Surplus"#]
    #[doc = r#"Description: Surplus (Excludes All Surplus Related To Preferred Stock) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis
"#]
    #[serde(rename="EQSUR")]
    pub eqsur: Option<i32>,

    #[doc = r#"Title: Undivided Profits"#]
    #[doc = r#"Description: Undivided Profits, Capital Reserves, Net Unrealized Holding Gains (Losses) On Available-For-Sale Securities and Other Equity Capital Components and/Or
Accumulated Gains (Losses) On Cash Flow Hedges On A Consolidated Basis

Note:
(1) Prior To March 1999 Included Undivided Profits, Capital Reserves and Net Unrealized Gains (Losses) On Available-For-Sale Securities
(2) Prior To March 1994 Included Undivided Profits and Capital Reserves Less Net Unrealized Loss On Marketable Equity Securities
(3) This Item Includes Net Worth Certificates For Bif Thrifts
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQUPTOT")]
    pub equptot: Option<i32>,

    #[doc = r#"Title: Employee Salaries and Benefits"#]
    #[doc = r#"Description: Salaries and Employee Benefits On A Consolidated Basis Note: For Banks With Foreign Operations, Data For December 72 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ESAL")]
    pub esal: Option<i32>,

    #[doc = r#"Title: Int Exp - Subordinated Notes"#]
    #[doc = r#"Description: Interest Expense On Subordinated Notes and Debentures On A Consolidated Basis
Note:
1. This Item Is Not Reported By Form 51 Filers
2. For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ESUBND")]
    pub esubnd: Option<i32>,

    #[doc = r#"Title: Net Extraordinary Items"#]
    #[doc = r#"Description: Discontinued Operations, Net Of Applicable Income Taxes On A Consolidated Basis

Note:
(1) Prior To March 2016, Defined As Extraordinary Items and and Other Adjustments, Net Of Taxes On A Consolidated Basis
(2) This Item Does Not Include The Tax Effects Related To Securities Gains and Losses and Extraordinary Items From June 1984 Through December 1985 For Bif Thrifts (Refer To Applicable Income Taxes)
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EXTRA")]
    pub extra: Option<i32>,

    #[doc = r#"Title: FDIC Supervised, BIF Insured Institutions"#]
    #[doc = r#"Description: FDIC Supervised, BIF Insured Institutions"#]
    #[serde(rename="FD_BIF")]
    pub fd_bif: Option<i32>,

    #[doc = r#"Title: FDIC supervised, SAIF Insured Institutions"#]
    #[doc = r#"Description: FDIC supervised SAIF insured institutions"#]
    #[serde(rename="FD_SAIF")]
    pub fd_saif: Option<i32>,

    #[doc = r#"Title: Federal Funds Sold"#]
    #[doc = r#"Description: Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis

Note:
(1) Prior To March 1997, Includes Only Federal Funds Sold and Securities Purchased Under Agreements To Resell In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S
(2) Prior To March 1998, Includes Only Federal Funds Sold For Tfr Filers
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="FREPO")]
    pub frepo: Option<i32>,

    #[doc = r#"Title: Fed Funds & Repos Purchased"#]
    #[doc = r#"Description: Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis 
Note:
(1) Prior To March 1998, Includes Only Reverse Repurchase Agreements For Tfr Filers
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="FREPP")]
    pub frepp: Option<i32>,

    #[doc = r#"Title: Int Inc - Balances Due"#]
    #[doc = r#"Description: Total Interest Income On Balances Due From Depository Institutions On A Consolidated Basis"#]
    #[serde(rename="ICHBAL")]
    pub ichbal: Option<i32>,

    #[doc = r#"Title: Fee Income"#]
    #[doc = r#"Description: Fee Income (Represents service charges on deposit accounts such as maintenance fees, activity charges, administrative charges, overdraft charges and check certification charges; mortgage loans servicing fees plus other fees and charges, including prepayment loan fees, late charges, assumption fees, and amortization of commitment fees.)"#]
    #[serde(rename="IFEE")]
    pub ifee: Option<i32>,

    #[doc = r#"Title: Int Inc - Fed Funds Sold/Securities Purchased"#]
    #[doc = r#"Description: Interest Income On Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis
Note:
(1) Prior To March 1997 Included Only Income From Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibfs On A Consolidated Basis
(2) For Banks With Foreign Operations Data For December 1972 Through 1975 Are Domestic Only"#]
    #[serde(rename="IFREPO")]
    pub ifrepo: Option<i32>,

    #[doc = r#"Title: Securities Gains and Losses"#]
    #[doc = r#"Description: Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Debt Securities and Unrealized Holding Gains (Losses) On Equity Securities Not Held For Trading Before Adjustments For Income Taxes On A Consolidated Basis (Also Includes Realized Gains On Equity Securities Until The Institution Adopts Asu 2016-01)
Note:
1. Prior To March 2018, Defined As Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Securities Before Adjustments For Income Taxes On A Consolidated Basis
2. Beginning In The 2018 Reporting Year, Includes Unrealized Gains (Losses) On Equity Securities For Institutions That Adopted Asu2016-01 and Includes Realized Gains (Losses) On Equity Securities For Institutions That Have Not Yet Adopted Asu2016-01
3. Prior To March 1994 Defined As Gains (Losses) On Securities Not Held In Trading Accounts 
4. From March 1990 Through March 2009, Includes Gains (Losses) On Assets Held For Sale For Tfr Filers
5. Includes Gains (Losses) On Loans Held For Investment From March 1984 Through December 1989 For Tfr Filers
6. Tfr Filers Report Only Gains From March 1984 Through December 1986
7. For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="IGLSEC")]
    pub iglsec: Option<i32>,

    #[doc = r#"Title: Int Inc - Domestic Office Loans"#]
    #[doc = r#"Description: Total Interest and Fees On Loans Held In Domestic Offices
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ILNDOM")]
    pub ilndom: Option<i32>,

    #[doc = r#"Title: Int Inc - Foreign Office Loans"#]
    #[doc = r#"Description: Total Interest and Fees On Loans Held In Foreign Offices, Edge and Agreement Subsidiaries, and Ibf'S"#]
    #[serde(rename="ILNFOR")]
    pub ilnfor: Option<i32>,

    #[doc = r#"Title: Int Inc - Total Loans & Leases"#]
    #[doc = r#"Description: Interest and Fees On Loans and Lease Financing Receivables On A Consolidated Basis
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ILNLS")]
    pub ilnls: Option<i32>,

    #[doc = r#"Title: Int Inc - Loans"#]
    #[doc = r#"Description: Loans (Represents all interest, fees and similar charges levied against or associated with all assets reportable as loans. Includes interest, yield related fees, commitment fees, service charges on loans and discount accretion. (One savings bank with an office in Canada has been reporting on the Domestic & Foreign Consolidated Call Report form (FFIEC 031). It does not, however, indicate any income or expenses related to foreign operations.))"#]
    #[serde(rename="ILNS")]
    pub ilns: Option<i32>,

    #[doc = r#"Title: Int Inc - Leases"#]
    #[doc = r#"Description: Total Interest Income From Lease Financing Receivables On A
Consolidated Basis"#]
    #[serde(rename="ILS")]
    pub ils: Option<i32>,

    #[doc = r#"Title: Intangible Assets"#]
    #[doc = r#"Description: Intangible Assets On A Consolidated Basis"#]
    #[serde(rename="INTAN")]
    pub intan: Option<i32>,

    #[doc = r#"Title: Total Interest Earning Assets"#]
    #[doc = r#"Description: Total Interest Earning Assets (Derived See Si-19) - Sc"#]
    #[serde(rename="INTBAST")]
    pub intbast: Option<i32>,

    #[doc = r#"Title: Total Interest Bearing Liabilities"#]
    #[doc = r#"Description: Total Interest Bearing Liabilities (Derived See Si-19) - Sc"#]
    #[serde(rename="INTBLIB")]
    pub intblib: Option<i32>,

    #[doc = r#"Title: Total Interest Income"#]
    #[doc = r#"Description: Total Interest Income On A Consolidated Basis
Note: For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="INTINC")]
    pub intinc: Option<i32>,

    #[doc = r#"Title: INTINC2"#]
    #[doc = r#"Description: INTINC2"#]
    #[serde(rename="INTINC2")]
    pub intinc2: Option<i32>,

    #[doc = r#"Title: Memo: IRA's and Keogh Plan-Deposits"#]
    #[doc = r#"Description: Individual Retirement Accounts (Ira'S) and Keogh Plan
Accounts Held In Domestic Offices
Note: Listed As Memoranda Only"#]
    #[serde(rename="IRAKEOGH")]
    pub irakeogh: Option<i32>,

    #[doc = r#"Title: Int Inc - Investment Securities"#]
    #[doc = r#"Description: Total Interest and Dividend Income On: U.S. Treasury Securities, U.S. Government Agency and Corporation Obligations, Securities Issued By States and Political
Subdivision In The U.S., Other Domestic Debt Securities, Foreign Debt Securities, and Equity Securities (Including Investments In Mutual Funds) On A Consolidated Basis
Note:
(1) This Item Includes Interest Income On Deposits For Tfr Filers
(2) Includes Interest Income On Assets Held In Trading Accounts For Tfr Filers For Two Distinct Periods: (A) March 1984 Through December 1989 and (B) June 1996
And Following Quarters.
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ISC")]
    pub isc: Option<i32>,

    #[doc = r#"Title: Service Charges on Deposit Accounts"#]
    #[doc = r#"Description: Represents service charges on deposit accounts in domestic offices such as maintenance fees, activity charges, administrative charges, overdraft charges, and check certification charges"#]
    #[serde(rename="ISERCHG")]
    pub iserchg: Option<i32>,

    #[doc = r#"Title: Applicable Income Taxes"#]
    #[doc = r#"Description: Represents Federal, state and local taxes on income. It does not include taxes relating to securities transactions or extraordinary items"#]
    #[serde(rename="ITAX")]
    pub itax: Option<i32>,

    #[doc = r#"Title: Pre-Tax Net Operating Income"#]
    #[doc = r#"Description: Pre-Tax Net Operating Income (Represents Net Interest Income plus Total Non-interest Income less Total Non-interest Expense and the Provision for Loan & Lease Losses.)"#]
    #[serde(rename="ITAXR")]
    pub itaxr: Option<i32>,

    #[doc = r#"Title: Int Inc - Trading Account Assets"#]
    #[doc = r#"Description: Interest Income From Assets Held In Trading Accounts On A Consolidated Basis 
Note:
Beginning March 2017, Reported As An Individual Income Category For Form 031 Filers Only and Is Included As A Component Of Other Interest Income For All Other Report Forms"#]
    #[serde(rename="ITRADE")]
    pub itrade: Option<i32>,

    #[doc = r#"Title: Total Liabilities"#]
    #[doc = r#"Description: Total Liabilities Including Subordinated Notes and Debentures and Limited Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: Prior To March 2009, This Item Included Noncontrolling (Minority) Interests In Consolidated Subsidiaries For Call Report and Tfr Filers"#]
    #[serde(rename="LIAB")]
    pub liab: Option<i32>,

    #[doc = r#"Title: Total Liabilities and Equity Capital"#]
    #[doc = r#"Description: Total Liabilities, Limited-Life Preferred Stock, and Equity Capital On A Consolidated Basis Note: For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="LIABEQ")]
    pub liabeq: Option<i32>,

    #[doc = r#"Title: Assisted Payouts"#]
    #[doc = r#"Description: Represents all assisted payouts of FDIC-insured savings institutions that are not in RTC conservatorship."#]
    #[serde(rename="liqasstd")]
    pub liqasstd: Option<i32>,

    #[doc = r#"Title: Voluntary Liquidations"#]
    #[doc = r#"Description: Represents all instances where the owners of a thrift voluntarily surrender their charter with all liabilities including deposits paid down and all assets sold."#]
    #[serde(rename="liqunass")]
    pub liqunass: Option<i32>,

    #[doc = r#"Title: Agricultural Loans"#]
    #[doc = r#"Description: Loans To Finance Agricultural Production and Other Loans To Farmers On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods Form December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNAG")]
    pub lnag: Option<i32>,

    #[doc = r#"Title: All Other Loans to Individuals"#]
    #[doc = r#"Description: All Other Loans (1969-Present -- Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premises.)"#]
    #[serde(rename="LNALLOTH")]
    pub lnalloth: Option<i32>,

    #[doc = r#"Title: Allowance for Losses Loans and Leases"#]
    #[doc = r#"Description: Allowance For Loan and Lease Financing Receivable Losses and Allocated Transfer Risk On A Consolidated Basis
Note:
(1) From March 2001 To Dec 2002 Allocated Transfer Riskis Netted From Loans & Not Included As Part Of The Reserve
(2) Additional Detail Can Be Found On Schedule Ri-B
(3) For Tfr Filers Between March 1984 Through December 1989 Includes Allowance For Mortgage Pool Securities
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="LNATRES")]
    pub lnatres: Option<i32>,

    #[doc = r#"Title: Memo: Loans to Individuals - Auto"#]
    #[doc = r#"Description: Represents installment loans to purchase private passenger automobiles, both direct loans and purchased paper"#]
    #[serde(rename="LNAUTO")]
    pub lnauto: Option<i32>,

    #[doc = r#"Title: Commercial and Industrial Loans"#]
    #[doc = r#"Description: Commercial and Industrial Loans On A Consolidated Basis Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCI")]
    pub lnci: Option<i32>,

    #[doc = r#"Title: Total Loans to Individuals"#]
    #[doc = r#"Description: Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) On A Consolidated Basis
Note:
(1) For Tfr Filers Includes Revolving Loans Secured By 1-4 Family Dwelling Units From March 1984 Through March 1988
(1) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCON")]
    pub lncon: Option<i32>,

    #[doc = r#"Title: Loans to Individuals - Home Improvement"#]
    #[doc = r#"Description: Installment Loans To Individuals To Repair and Modernize
Residential Property Held In Domestic Offices"#]
    #[serde(rename="LNCONOT1")]
    pub lnconot1: Option<i32>,

    #[doc = r#"Title: Loans to Individuals - All Others"#]
    #[doc = r#"Description: Represents all other loans to individuals for household, family and other personal expenditures. It includes auto loans, both direct and indirect, mobile homes (unless secured by a real estate mortgage), education loans, other installment loans both secured by personal property or unsecured, and single payment loans (time or demand, secured or unsecured)"#]
    #[serde(rename="LNCONOTH")]
    pub lnconoth: Option<i32>,

    #[doc = r#"Title: Loans to Individuals - Credit Card Plans"#]
    #[doc = r#"Description: Credit Cards Related Plans On A Consolidated Basis

Note:
(1)Prior To March 2001 Includes Credit Cards Related Plans-Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) Includes Check Credit and Other Revolving Credit Plans
(2) For Tfr Filers Between March 1984 Through March 1988 This Figure Includes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(3) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCRCD")]
    pub lncrcd: Option<i32>,

    #[doc = r#"Title: Loans to Deposit Institutions"#]
    #[doc = r#"Description: Loans To Depository Institutions On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only
Note:(1) Beginning March 2001 Includes Acceptances Of Other Banks
(2) Beginning March 2001, Includes Acceptances Of Other Banks For Ibas"#]
    #[serde(rename="LNDEP")]
    pub lndep: Option<i32>,

    #[doc = r#"Title: Gross Loans and Leases"#]
    #[doc = r#"Description: Represents the sum of all components of loans"#]
    #[serde(rename="LNLS")]
    pub lnls: Option<i32>,

    #[doc = r#"Title: Total Loans and Leases"#]
    #[doc = r#"Description: Loans and Lease Financing Receivables, Net Of Unearned Income, On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-C
(2) For Tfr Filers This Item Is Net Of Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through December 1989
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="LNLSGR")]
    pub lnlsgr: Option<i32>,

    #[doc = r#"Title: Net Loans and Leases"#]
    #[doc = r#"Description: Loans and Lease Financing Receivables, Net Of Unearned Income, Allowance, and Reserve On A Consolidated Basis
Note:
(1) For Tfr Filers This Item Is Net Of Valuation Allowances and Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through
(2) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="LNLSNET")]
    pub lnlsnet: Option<i32>,

    #[doc = r#"Title: Memo: Loans to Individuals - Mobile Homes"#]
    #[doc = r#"Description: Represents loans to individuals to purchase mobile homes. (If the bank's security interest in the loan was represented by a mortgage or deed of trust, the loan should be included in real estate loans)"#]
    #[serde(rename="LNMOBILE")]
    pub lnmobile: Option<i32>,

    #[doc = r#"Title: Loans to States and Politicial Sub-divisions"#]
    #[doc = r#"Description: Obligations (Other Than Securities and Leases) Of States and Political Subdivisions In The U.S. (Including Nonrated Industrial Development Obligations) On A Consolidated Basis"#]
    #[serde(rename="LNMUNI")]
    pub lnmuni: Option<i32>,

    #[doc = r#"Title: Total Real Estate Loans"#]
    #[doc = r#"Description: Loans Secured By Real Estate On A Consolidated Basis
Note:
(1) For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(2) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNRE")]
    pub lnre: Option<i32>,

    #[doc = r#"Title: R/E Loan - Farmland"#]
    #[doc = r#"Description: Represents loans secured by farmland, including improvements, and other land known to be used or usable for agricultural purposes, as evidenced by mortgages or other liens. It includes loans secured by farmland that are guaranteed by the Farmers Home Administration (FHA) or by the Small Business Administration"#]
    #[serde(rename="LNREAG")]
    pub lnreag: Option<i32>,

    #[doc = r#"Title: R/E Loan - Construction & Land Develop"#]
    #[doc = r#"Description: Construction and Land Development Loans Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers Portions Of Lnrecons Were Included In Other Real Estate Loan Categories Prior To March 30, 1986"#]
    #[serde(rename="LNRECONS")]
    pub lnrecons: Option<i32>,

    #[doc = r#"Title: Total R/E Loans in Domestic Offices"#]
    #[doc = r#"Description: Represents the total of all loans secured by real estate in domestic offices (U.S. and other areas)"#]
    #[serde(rename="LNREDOM")]
    pub lnredom: Option<i32>,

    #[doc = r#"Title: Total R/E Loans in Foreign Offices"#]
    #[doc = r#"Description: Represents all loans secured by real estate in foreign offices"#]
    #[serde(rename="LNREFOR")]
    pub lnrefor: Option<i32>,

    #[doc = r#"Title: Memo: Home Equity Loans"#]
    #[doc = r#"Description: Revolving, Open-End Loans Secured By 1-4 Family Residential Properties and Extended Under Lines Of Credit Held In Domestic Offices"#]
    #[serde(rename="LNRELOC")]
    pub lnreloc: Option<i32>,

    #[doc = r#"Title: R/E Loans -  Multifamily"#]
    #[doc = r#"Description: Multifamily (5 Or More) Residential Properties Secured By Real Estate Held In Domestic Offices"#]
    #[serde(rename="LNREMULT")]
    pub lnremult: Option<i32>,

    #[doc = r#"Title: R/E Loan -  Non-farm/Non-residential Prop"#]
    #[doc = r#"Description: Nonfarm Nonresidential Properties Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers This Figure Includes Mortgages On Properties That Are Used For Farming"#]
    #[serde(rename="LNRENRES")]
    pub lnrenres: Option<i32>,

    #[doc = r#"Title: R/E Loan - 1-4 Family"#]
    #[doc = r#"Description: Total Loans Secured By 1-4 Family Residential Properties Held In Domestic Offices
Note: For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower"#]
    #[serde(rename="LNRERES")]
    pub lnreres: Option<i32>,

    #[doc = r#"Title: Memo: Contra Account"#]
    #[doc = r#"Description: Allowance For Loan Losses On Real Estate Loans On A Consolidated Basis
Note: For Tfr Filers Includes Allowance For Mortgage Pool Securities Between March 1984 Through December 1989"#]
    #[serde(rename="LNRESRE")]
    pub lnresre: Option<i32>,

    #[doc = r#"Title: Memo: Loans to Individuals - Single Payment"#]
    #[doc = r#"Description: All loans both time or demand, secured or unsecured, to individuals for personal, family or other household expenditures"#]
    #[serde(rename="LNSP")]
    pub lnsp: Option<i32>,

    #[doc = r#"Title: Leases"#]
    #[doc = r#"Description: Lease Financing Receivables (Net Of Unearned Income) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Call Dates Are Domestic Only Through December 1975"#]
    #[serde(rename="LS")]
    pub ls: Option<i32>,

    #[doc = r#"Title: Failures: Assisted Merger"#]
    #[doc = r#"Description: Mergers, consolidations or absorptions entered into as a result of supervisory actions. The transaction may or may not have required FDIC assistance."#]
    #[serde(rename="MERGERS")]
    pub mergers: Option<i32>,

    #[doc = r#"Title: Other Misc. Adjustments"#]
    #[doc = r#"Description: Represents any FDIC-insured savings institution that did not file a financial report during the year in which the charter was added or deleted."#]
    #[serde(rename="MISSADJ")]
    pub missadj: Option<i32>,

    #[doc = r#"Title: Mortgage and Other Borrowings"#]
    #[doc = r#"Description: Represents mortgage indebtedness and liabilities under capitalized leases"#]
    #[serde(rename="MTGLS")]
    pub mtgls: Option<i32>,

    #[doc = r#"Title: Non-accrual Loans & Leases"#]
    #[doc = r#"Description: Total Nonaccrual Loans and Lease Financing Receivables On A Consolidated Basis"#]
    #[serde(rename="NALNLS")]
    pub nalnls: Option<i32>,

    #[doc = r#"Title: Net Loans and Leases Charge-offs"#]
    #[doc = r#"Description: Net Loans and Leases Charge Offs (-- 1984-1989 -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries. Not collected by TFR filers. -- 1990-Present -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries.)"#]
    #[serde(rename="NCHGREC")]
    pub nchgrec: Option<i32>,

    #[doc = r#"Title: Total Non-current Loans & Leases"#]
    #[doc = r#"Description: Total Loans and Lease Financing Receivables 90 Days Or More Past Due and Nonaccrual On A Consolidated Basis
Note: Includes Delinquent Loans (60 Or More Days Overdue) and Past Due Loans (One Or More Payments Missed) For Tfr Filers Prior To March 1990"#]
    #[serde(rename="NCLNLS")]
    pub nclnls: Option<i32>,

    #[doc = r#"Title: Net Income Attributable to Noncontrolling Interests"#]
    #[doc = r#"Description: Net income (loss) attributable to noncontrolling (minority) interests on a consolidated basis."#]
    #[serde(rename="NETIMIN")]
    pub netimin: Option<i32>,

    #[doc = r#"Title: Net Income"#]
    #[doc = r#"Description: Net Income Attributable To The Bank On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NETINC")]
    pub netinc: Option<i32>,

    #[doc = r#"Title: New Charters"#]
    #[doc = r#"Description: Institutions newly chartered by federal or state banking authorities including authorities in the U. S. Territories or possessions."#]
    #[serde(rename="newcount")]
    pub newcount: Option<i32>,

    #[doc = r#"Title: New Charters"#]
    #[doc = r#"Description: Institutions newly licensed or chartered by the Office of the Comptroller of the Currency (national banks) or by state banking authorities, including banking authorities in the U. S. territories or possessions. Includes de novo institutions as well as charters issued to take over a failing institution."#]
    #[serde(rename="New_Char")]
    pub new_char: Option<i32>,

    #[doc = r#"Title: Int Exp - Borrowed Money"#]
    #[doc = r#"Description: Represents interest expense related to demand notes issued to the U. S. Treasury, mortgage indebtedness, obligations under capitalized leases, and other borrowed money"#]
    #[serde(rename="NEW6_1")]
    pub new6_1: Option<i32>,

    #[doc = r#"Title: All Other Assets"#]
    #[doc = r#"Description: Represents all other assets not included in previously mentioned captions. Includes, for the most part, customers' liabilities on acceptances outstanding, income earned not collected as well as any other asset not included above"#]
    #[serde(rename="NEW9_1")]
    pub new9_1: Option<i32>,

    #[doc = r#"Title: Corporate Bonds and Other Securities"#]
    #[doc = r#"Description: Represents all securities, bonds, notes and debentures of domestic and foreign corporations. Also includes privately issued or guaranteed mortgage backed securities and certain detached U.S. Government security coupons held as a result of either their purchase or the bank's stripping them (CATS, TIGRs, COUGARs, LIONs and ETRs)."#]
    #[serde(rename="NEW10_1")]
    pub new10_1: Option<i32>,

    #[doc = r#"Title: Trading Account Securities"#]
    #[doc = r#"Description: Securities within the scope of ASC Topic 320, Investments – Debt Securities, that a bank has elected to report at fair value under a fair value option with changes in fair value reported in current earnings should be classified as trading securities. (https://www.fdic.gov/regulations/resources/call/crinst/2018-06/031-041-618rc-d-063018.pdf)"#]
    #[serde(rename="NEW10_2")]
    pub new10_2: Option<i32>,

    #[doc = r#"Title: Memo: Valuation Reserves"#]
    #[doc = r#"Description: For all years except 1969-1973, investment securities are reflected net of general valuation reserves. Specific reserves are deducted from each security so reserved"#]
    #[serde(rename="NEW10_3")]
    pub new10_3: Option<i32>,

    #[doc = r#"Title: All Other Loans"#]
    #[doc = r#"Description: Represents unplanned overdrafts and loans to: brokers and dealers in securities, any borrower for the purpose of purchasing and carrying securities, nonprofit institutions and organizations, individuals for investment purposes, real estate investment trusts, mortgage companies holding companies of depository institutions, insurance companies, finance companies, factors and other financial intermediaries, federally sponsored lending agencies, investment banks, the bank's own trust department, Small Business Investment Companies, foreign governments and official institutions, and any other loan not included in one of the above categories"#]
    #[serde(rename="NEW11_1")]
    pub new11_1: Option<i32>,

    #[doc = r#"Title: Borrowed Funds"#]
    #[doc = r#"Description: Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money"#]
    #[serde(rename="NEW14_1")]
    pub new14_1: Option<i32>,

    #[doc = r#"Title: Other Liabilities"#]
    #[doc = r#"Description: Includes all liabilities not included above and limited life preferred stock"#]
    #[serde(rename="NEW14_2")]
    pub new14_2: Option<i32>,

    #[doc = r#"Title: Total Liabilities"#]
    #[doc = r#"Description: Represents the total of all components of liabilities"#]
    #[serde(rename="NEW14_3")]
    pub new14_3: Option<i32>,

    #[doc = r#"Title: Undivided Profits"#]
    #[doc = r#"Description: Represents undivided profits and related accounts"#]
    #[serde(rename="NEW14_4")]
    pub new14_4: Option<i32>,

    #[doc = r#"Title: Deposits - Individuals, Partnerships and Corporations"#]
    #[doc = r#"Description: Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#]
    #[serde(rename="NEW15_1")]
    pub new15_1: Option<i32>,

    #[doc = r#"Title: Deposits - U.S. Government"#]
    #[doc = r#"Description: Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#]
    #[serde(rename="NEW15_2")]
    pub new15_2: Option<i32>,

    #[doc = r#"Title: Deposits - States and Political Subdivisions"#]
    #[doc = r#"Description: Represents all deposits of states, counties and municipalities in domestic offices. Such deposits, if any, in foreign offices are not separately reported"#]
    #[serde(rename="NEW15_3")]
    pub new15_3: Option<i32>,

    #[doc = r#"Title: Deposits - All Other"#]
    #[doc = r#"Description: Represents all other deposits. Includes deposits of financial institutions, both domestic and foreign, deposits of foreign governments and official institutions and certified and official checks. Also includes deposits in foreign offices other than those of individuals, partnerships and corporations"#]
    #[serde(rename="NEW15_4")]
    pub new15_4: Option<i32>,

    #[doc = r#"Title: Deposits - Domestic Savings"#]
    #[doc = r#"Description: Represents all savings deposits in domestic offices"#]
    #[serde(rename="NEW15_5")]
    pub new15_5: Option<i32>,

    #[doc = r#"Title: Total Domestic Deposits"#]
    #[doc = r#"Description: Total Domestic Deposits"#]
    #[serde(rename="NEW15_7")]
    pub new15_7: Option<i32>,

    #[doc = r#"Title: Demand Notes and Other Liabilities"#]
    #[doc = r#"Description: Represents demand notes issued to the U.S. Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers"#]
    #[serde(rename="NEW16_1")]
    pub new16_1: Option<i32>,

    #[doc = r#"Title: Interest Bearing Deposits"#]
    #[doc = r#"Description: Represents any deposit in domestic and foreign offices on which the banks pays or accrues interest"#]
    #[serde(rename="NEW16_2")]
    pub new16_2: Option<i32>,

    #[doc = r#"Title: Net Interest Income"#]
    #[doc = r#"Description: Net Interest Income (Total Interest Income Minus Total Interest Expense) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NIM")]
    pub nim: Option<i32>,

    #[doc = r#"Title: Total Non-interest Income"#]
    #[doc = r#"Description: Total Non-interest Income On A Consolidated Basis
Note:
(1) From March 1990 Through March 2009, Excludes Gains (Losses) On Assets Held For Sale For Tfr Filers, See Tfr Instructions For So430
(2) Excludes Gains On The Sale Of Loans Held For Investments From March 1984 Through December 1989 For Tfr Filers
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NONII")]
    pub nonii: Option<i32>,

    #[doc = r#"Title: Total Non-interest Expense"#]
    #[doc = r#"Description: Total Non-interest Expense On A Consolidated Basis
Note:
(1) Excludes Losses On Asset Sales For Tfr Filers Beginning June 1996
(2) Includes Loss On Sale Of Mortgage Pool and Other Securities Held For Investment For Tfr Filers From March 1984 Through December 1986
(3) Excludes Losses On Loans Held For Investment For Tfr Filers From March 1987 Through December 1989
(4) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NONIX")]
    pub nonix: Option<i32>,

    #[doc = r#"Title: Net Loan & Lease Charge-offs"#]
    #[doc = r#"Description: Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in parentheses indicates net recoveries. Not collected by TFR filers"#]
    #[serde(rename="NTLNLS")]
    pub ntlnls: Option<i32>,

    #[doc = r#"Title: Memo: Domestic Deposits Non-Transaction"#]
    #[doc = r#"Description: Represents deposits that are not included in the definition of transaction accounts above or that do not satisfy the criteria necessary to be defined as a transaction account. MMDA's are specifically defined as nontransaction accounts"#]
    #[serde(rename="NTR")]
    pub ntr: Option<i32>,

    #[doc = r#"Title: Deposits - Domestic Time"#]
    #[doc = r#"Description: Represents all time certificates of deposit, time open accounts and similar deposits in domestic offices"#]
    #[serde(rename="NTRTIME")]
    pub ntrtime: Option<i32>,

    #[doc = r#"Title: Memo: Time Deposits (Over $100K)"#]
    #[doc = r#"Description: Time Deposits Over $100,000 Or More Held In Domestic Offices
Note:
(1) Listed As Memoranda Only and Is Included In Total Nontransaction Accounts
(2) Prior To March 2007, Includes All Deposits (Not Just Time) Greater Than $100,000 For Tfr Filers. Except For December 2006, Includes All Nonretirement Deposits Over
$100,000 and All Retirement Deposits Over $250,000 For Tfr Filers
(3) Includes Time Deposits Of $100,000 Or More"#]
    #[serde(rename="NTRTMLG")]
    pub ntrtmlg: Option<i32>,

    #[doc = r#"Title: Number of Full Time Employees"#]
    #[doc = r#"Description: Number Of Full Time-Equivalent Employees On Payroll At The End Of The Current Period
Note:
(1) Listed As Memoranda Only
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="NUMEMP")]
    pub numemp: Option<i32>,

    #[doc = r#"Title: Other Earning Assests"#]
    #[doc = r#"Description: Other Earning Assests (-- 1984-1989 -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Items not separately reported by TFR filers. They are included in Secruties. -- 1990-Present -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Includes only Federal funds sold for TFR filers. Repurchase agreements are included in Securities.)"#]
    #[serde(rename="OEA")]
    pub oea: Option<i32>,

    #[doc = r#"Title: Offices"#]
    #[doc = r#"Description: Offices include: Multiple service offices, Military facilities, Drive-in facilities, Loan production offices, Consumer credit offices, Seasonal offices, Administrative offices, Messenger service offices, Supermarket banking offices, and Other offices."#]
    #[serde(rename="OFFICES")]
    pub offices: Option<i32>,

    #[doc = r#"Title: Demand Notes and Other Borrowings"#]
    #[doc = r#"Description: Demand Notes and Other Borrowings (Represents demand notes issued to US Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers.)"#]
    #[serde(rename="OINTBOR")]
    pub ointbor: Option<i32>,

    #[doc = r#"Title: Total Other Interest Expenses"#]
    #[doc = r#"Description: Total Other Interest Expenses (Federal Funds Purchased and Securities Sold -- Represents the gross expense of all liabilities reportable under this category. This item is not reported separately by TFR filers. It is included in Borrowed Money)."#]
    #[serde(rename="OINTEXP")]
    pub ointexp: Option<i32>,

    #[doc = r#"Title: Int Inc - Total Other"#]
    #[doc = r#"Description: Total Other Interest Income (Represents the total of all Other Interest Income components)."#]
    #[serde(rename="OINTINC")]
    pub ointinc: Option<i32>,

    #[doc = r#"Title: Other Non-interest Income"#]
    #[doc = r#"Description: Other Non Interest Income (1984-1989 -- Same as Total Other Interest Income except gains on the sale of loans held for investment are excluded for TFR filers. -- 1990- Present -- Represents income derived from the sale of assets held for sale; office building operations; real estate held for investment; REO operations; LOCOM adjustments made to assets held for sale; net income (loss) from investements in service corporations/subsidiaries (other than operating or finance subsidiaires); leasing operations; realized and unrealized gains (losses) on trading assets; gains on the sale of REO real estate held for investment, and loans held for investment; and the amoritization of deferred gains (losses) on asset hedges.)"#]
    #[serde(rename="OONONII")]
    pub oononii: Option<i32>,

    #[doc = r#"Title: Other Real Estate Owned"#]
    #[doc = r#"Description: Other Real Estate Owned On A Consolidated Basis
Note:
(1) Prior To June 2009, Includes Direct and Indirect Investments In Real Estate
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="ORE")]
    pub ore: Option<i32>,

    #[doc = r#"Title: Other Real Estate"#]
    #[doc = r#"Description: Other Real Estate (Represents other real estate owned net of reserves for losses). Not available for 1997. For 1986 through 1988 ORET = ORE + INVSUORE; for all other years ORET = ORE"#]
    #[serde(rename="ORET")]
    pub oret: Option<i32>,

    #[doc = r#"Title: Non FDIC Supervised BIF Insured Institutions"#]
    #[doc = r#"Description: Non FDIC supervised BIF insured institutions"#]
    #[serde(rename="OT_BIF")]
    pub ot_bif: Option<i32>,

    #[doc = r#"Title: Non FDIC Supervised SAIF Insured Institutions"#]
    #[doc = r#"Description: Non FDIC supervised SAIF insured institutions"#]
    #[serde(rename="OT_SAIF")]
    pub ot_saif: Option<i32>,

    #[doc = r#"Title: All Other Assets"#]
    #[doc = r#"Description: All Other Assets (Same as Other Real Estate except that investment in service corporations/subsidiaries is reported gross of valuation allowances by TFR filers, and assets held in trading accounts are included in Securities for TFR filers. -- 1990-Present -- Represents all associations assets not previously mentioned. Includes all non real estate repossessed property, investment in service corporations/subsidiaries, property leased to others, income earned but not yet collected, assets held in the trading accounts, and miscellaneous assets) For 2009- present OTHASST = SUM (INVSUB + INVSUORE + CUSLI + OA)"#]
    #[serde(rename="OTHASST")]
    pub othasst: Option<i32>,

    #[doc = r#"Title: Advances from FHLB"#]
    #[doc = r#"Description: Other Liabilities From The Fhlb
Note:Prior To March 2001 Only Reported On Tfrs"#]
    #[serde(rename="OTHBFHLB")]
    pub othbfhlb: Option<i32>,

    #[doc = r#"Title: Int Exp Oth - Borrowed Money"#]
    #[doc = r#"Description: Borrowed Money (Represents interest expense related to demand notes issued the US Treasury, mortage indebtedness, obligations under capitalized leases and on other borrowed money."#]
    #[serde(rename="OTHBORR")]
    pub othborr: Option<i32>,

    #[doc = r#"Title: Other Equity"#]
    #[doc = r#"Description: Represents all equity securities not held for trading: investment in mutual funds, common stock of FNMA, Student Loan Marketing Association, Federal Home Loan Mortgage Corporation, Federal Reserve Bank stock, Federal Home Loan Bank stock, minority interests not meeting the definition of associated companies, "restricted" stock, and other equity securities in both domestic and foreign corporations
"#]
    #[serde(rename="OTHEQ")]
    pub otheq: Option<i32>,

    #[doc = r#"Title: Other"#]
    #[doc = r#"Description: Withdrawals from FDIC insurance, voluntary liquidations, or conversions to institutions that are not considered commercial banks. Also includes relocation of banks from one state to another."#]
    #[serde(rename="OTHER")]
    pub other: Option<i32>,

    #[doc = r#"Title: Other Liabilities"#]
    #[doc = r#"Description: Other Liabilities (Includes all liabilities not included above and limited life preferred stock. 2001- present -- Includes OTHER LIAB & MINOR IN SUBS)."#]
    #[serde(rename="OTHLIAB")]
    pub othliab: Option<i32>,

    #[doc = r#"Title: Borrowed Funds"#]
    #[doc = r#"Description: Borrowed Funds (Includes federal funds purchased, securities sold under agreements to repurchase (reverse repurchase agreements), demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. Includes only reverse purchase agreements (securities sold under agreements to repurchase) and FSLIC net worth certificates for TFR filers)"#]
    #[serde(rename="OTHNBORR")]
    pub othnborr: Option<i32>,

    #[doc = r#"Title: Less: Other Contra Accounts"#]
    #[doc = r#"Description: Other Contracts (Represents amount reported by savings institutions that file on the Thrift Financial Report. Contra accounts include accrued interest receivable, unamortized yield adjustments and valuation allowances. Negative amounts reflect unamortized premiums and deferred direct costs exceeding unamortized discounts and deferred loan fees)."#]
    #[serde(rename="OTLNCNTA")]
    pub otlncnta: Option<i32>,

    #[doc = r#"Title: Failures: Paid Off"#]
    #[doc = r#"Description: Institutions that were declared insolvent, the insured deposits of which were paid by the FDIC."#]
    #[serde(rename="PAID_OFF")]
    pub paid_off: Option<i32>,

    #[doc = r#"Title: Loans & Leases P/D 30-89 Days"#]
    #[doc = r#"Description: Total Loans and Lease Financing Receivables Past Due 30 Through 89 Days and Still Accruing Interest On A Consolidated Basis
Note:
(1) Prior To March 2001,This Information On An Institution Level Is Considered Confidential By The Ffiec"#]
    #[serde(rename="P3LNLS")]
    pub p3lnls: Option<i32>,

    #[doc = r#"Title: Loans & Leases P/D 90+ Days"#]
    #[doc = r#"Description: Total Loans and Lease Financing Receivables Past Due 90 Or More Days and Still Accruing Interest On A Consolidated Basis"#]
    #[serde(rename="P9LNLS")]
    pub p9lnls: Option<i32>,

    #[doc = r#"Title: Pre-Tax Net Operating Income"#]
    #[doc = r#"Description: Pre-Tax Net Operating Income"#]
    #[serde(rename="PTXNOINC")]
    pub ptxnoinc: Option<i32>,

    #[doc = r#"Title: Conversions"#]
    #[doc = r#"Description: Conversions of existing institutions of any type that meet the definition of commercial banks (see Definition of Total Commercial Banks and have applied for and received FDIC insurance. Also includes bank relocations from one state to another."#]
    #[serde(rename="REL_CO")]
    pub rel_co: Option<i32>,

    #[doc = r#"Title: Total Savings Institutions (Total Insured)"#]
    #[doc = r#"Description: Total Insured Savings Institutions including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)."#]
    #[serde(rename="SAVINGS")]
    pub savings: Option<i32>,

    #[doc = r#"Title: Total Investment Securities (Book Value)"#]
    #[doc = r#"Description: Total Securities: The Sum Of Held-To-Maturity Securities At Amortized Cost, Available-For-Sale Securities At Fair Value and Equity Securities With Readily Determinable Fair Values Not Held For Trading On A Consolidated Basis
Note:
1. Prior To March 2018, Defined As Total Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities (Excludes Assets Held In Trading Accounts) On A Consolidated Basis
2. Beginning In 2018, Includes Equity Securities For Institutions That Have Adopted Asu2016-01 and Those Institutions That Have Not Yet Adopted This Accounting
Standard
3. Prior To March 1994 Item Defined As Book Value
4. Additional Detail Can Be Found On Schedule Rc-B
5. For Tfr Filers Between March 1984 Through December 1989 Includes Interest-Earning Deposits In Fhlbs, Other Interest-Earning Deposits, Federal Funds Sold and Assets Held In Trading Accounts
6. For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SC")]
    pub sc: Option<i32>,

    #[doc = r#"Title: U.S. Agencies and Corporation Securities"#]
    #[doc = r#"Description: Total U.S. Government Agency and Corporation Obligations On A Consolidated Basis
Note:
1) From June 2009 Through December 2010, This Item Excluded Other Commercial
Mortgage-Backed Securities
2) Prior To June 2009, This Item Included Other Commercial Mortgage-Backed Securities
3) Beginning March 1994 Consists Of Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities
4) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
5) Includes U.S. Treasury Securities For Tfr Filers Between March 1984 Through December 1989 and After March 1996
6) Does Not Include Mortgage Derivative Securities For Tfr Filers Between March 1984 Through December 1986
7) For Banks With Foreign Operations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCAGE")]
    pub scage: Option<i32>,

    #[doc = r#"Title: Equity Securities"#]
    #[doc = r#"Description: Total Equity Securities Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning March 2018 Does Not Include Equity Securities For Institutions That Have Adopted Asu 2016-01 See Sceqfv
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers"#]
    #[serde(rename="SCEQ")]
    pub sceq: Option<i32>,

    #[doc = r#"Title: Memo: Mortgage Backed Securities"#]
    #[doc = r#"Description: Mortgage Backed Securities On A Consolidated Basis
Includes:
(1) U.S. Government Agency and Corporation Obligations Issued Or Guaranteed Certificates Of Participation In Pools Of Residential Mortgages,
(2) U.S. Government Agency and Corporation Obligations Collateralized Mortgage Obligations Issued By Fnma and Fhlmc (Including Remics)
(3) Other Domestic Debt Securities - Private (I.E., Non-Government-Issued-Or-Guaranteed) Certificates Of Participations In Pools Of Residential Mortgages, and
(4) Other Domestic Debt Securities - Privately-Issued Collateralized Mortgage Obligations (Including Remics)"#]
    #[serde(rename="SCMTGBK")]
    pub scmtgbk: Option<i32>,

    #[doc = r#"Title: States and Political Subdivisions Securities"#]
    #[doc = r#"Description: Total Securities Issued By States and Political Subdivisions Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Prior To March 1994 Item Was Defined As Book Value
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(3) For Banks With Foreign Opeations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCMUNI")]
    pub scmuni: Option<i32>,

    #[doc = r#"Title: Market Values"#]
    #[doc = r#"Description: Represents the market (fair) value of all investment securities"#]
    #[serde(rename="SCMV")]
    pub scmv: Option<i32>,

    #[doc = r#"Title: Less: Contra Accounts"#]
    #[doc = r#"Description: Contra-Assets To Securities (Reserves)
Note: For Tfr Filers Only"#]
    #[serde(rename="SCRES")]
    pub scres: Option<i32>,

    #[doc = r#"Title: U.S. Treasury & Agency"#]
    #[doc = r#"Description: Total U.S. Treasury Securities and U.S. Government Agency and Corporation Obligations On A Consolidated Basis
Note:
1) From June 2009 Through December 2010 This Item Excluded Commercial Mortgage Backed Securities
2) Prior To June 2009, This Item Included Commercial Mortgage Backed Securities
3) Beginning March 1994 Consists Of Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities
4) Does Not Include Mortgage Derivative Securities From March 1984 Through December 1986 For Tfr Filers
5) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
6) For Banks With Foreign Operations Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCUS")]
    pub scus: Option<i32>,

    #[doc = r#"Title: Securities Of Us Agencies"#]
    #[doc = r#"Description: Securities Of Us Agencies"#]
    #[serde(rename="SCUSA")]
    pub scusa: Option<i32>,

    #[doc = r#"Title: U.S. Treasury Securities"#]
    #[doc = r#"Description: U.S. Treasury Securities Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning June 1996, Tfr Filers No Longer Report U.S. Treasury Securities Separately
(2) Prior To March 1994 Item Was Defined As Book Value
(3) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(4) For Banks With Foreign Operations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCUST")]
    pub scust: Option<i32>,

    #[doc = r#"Title: Locations (Search-Eligible)"#]
    #[doc = r#"Description: Locations This field can be used for search and filtering."#]
    #[serde(rename="STNAME")]
    pub stname: Option<String>,

    #[doc = r#"Title: State Number (Search-Eligible)"#]
    #[doc = r#"Description: State Number This field can be used for search and filtering."#]
    #[serde(rename="STNUM")]
    pub stnum: Option<String>,

    #[doc = r#"Title: Subordinated Notes"#]
    #[doc = r#"Description: Subordinated Notes and Debentures and Limited-Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: (1) Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domesitc Basis"#]
    #[serde(rename="SUBLLPF")]
    pub subllpf: Option<i32>,

    #[doc = r#"Title: Subordinated Notes/Debentures"#]
    #[doc = r#"Description: Represents all notes and debentures subordinated to deposits and all capital notes and debentures"#]
    #[serde(rename="SUBND")]
    pub subnd: Option<i32>,

    #[doc = r#"Title: Int Inc - Total Other"#]
    #[doc = r#"Description: Total Other Interest Income (Represents the sum of Other Interest Income - Investment Securities, Trading Account Assets, Federal Funds Sold and Securities Purchased, and Balanaces Due from Depository Institutions)"#]
    #[serde(rename="TINTINC")]
    pub tintinc: Option<i32>,

    #[doc = r#"Title: Charter Transfers to Commercial Banks"#]
    #[doc = r#"Description: Represents the charter transfer of existing FDIC-insured savings institutions to an FDIC-insured commercial bank charter."#]
    #[serde(rename="tochrt")]
    pub tochrt: Option<i32>,

    #[doc = r#"Title: Assisted Mergers with Commercial Banks"#]
    #[doc = r#"Description: Represents the absorption of a failing savings institution by a commercial bank with assistance from either the BIF or SAIF."#]
    #[serde(rename="tofail")]
    pub tofail: Option<i32>,

    #[doc = r#"Title: Int Exp  - Total Deposits"#]
    #[doc = r#"Description: Total Other Interest Expense (Represents the sum of all components of Other Interest Expense)"#]
    #[serde(rename="TOINTEXP")]
    pub tointexp: Option<i32>,

    #[doc = r#"Title: Unassisted Mergers with Commercial Banks"#]
    #[doc = r#"Description: Represents the absorption of a savings institution charter by a commercial bank without assistance."#]
    #[serde(rename="tomerg")]
    pub tomerg: Option<i32>,

    #[doc = r#"Title: Failures Transferred to the RTC"#]
    #[doc = r#"Description: Represents institutions that were declared failed and placed under RTC conservatorship until a buyer(s) is(are) found or a payout to depositors occurs."#]
    #[serde(rename="tortc")]
    pub tortc: Option<i32>,

    #[doc = r#"Title: Total Commercial Banks (Total Insured)"#]
    #[doc = r#"Description: Total Insured Commercial Banks including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)"#]
    #[serde(rename="TOTAL")]
    pub total: Option<i32>,

    #[doc = r#"Title: Total FDIC Supervised Savings Institutions"#]
    #[doc = r#"Description: Total FDIC Supervised Savings Institutions"#]
    #[serde(rename="TOT_FDIC")]
    pub tot_fdic: Option<i32>,

    #[doc = r#"Title: Total Non FDIC Supervised Savings Institutions"#]
    #[doc = r#"Description: Total Non FDIC Supervised Savings Institutions"#]
    #[serde(rename="TOT_OTS")]
    pub tot_ots: Option<i32>,

    #[doc = r#"Title: Total Savings Institutions"#]
    #[doc = r#"Description: All FDIC Insured Savings Institutions filing a 12/31 financial report"#]
    #[serde(rename="TOT_SAVE")]
    pub tot_save: Option<i32>,

    #[doc = r#"Title: Total Loans and Leases Past Due"#]
    #[doc = r#"Description: Total Loans and Leases Past Due"#]
    #[serde(rename="TPD")]
    pub tpd: Option<i32>,

    #[doc = r#"Title: Trading Account Assets"#]
    #[doc = r#"Description: Assets Held In Trading Accounts On A Consolidated Basis
Note:
(1) Effective March 1994 Item Reported On A Gross Basis
(2) Additional Detail Can Be Found On Schedule Rc-D
(3) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis,
(4) For Periods 1972 Through 1983 Includes Only Securities"#]
    #[serde(rename="TRADE")]
    pub trade: Option<i32>,

    #[doc = r#"Title: Less: Trading Accounts"#]
    #[doc = r#"Description: Trading Accounts"#]
    #[serde(rename="TRADES")]
    pub trades: Option<i32>,

    #[doc = r#"Title: Memo: Domestic Deposits Transaction"#]
    #[doc = r#"Description: Represents all demand deposits, NOW accounts, ATS accounts, accounts from which payments may be made to third parties by means of an automated teller machine, a remote service unit, or another electronic device, and accounts that permit third party payments through use of checks, drafts, negotiable instruments, or other similar instrument. (MMDA's are specifically excluded from the latter two definitions)"#]
    #[serde(rename="TRN")]
    pub trn: Option<i32>,

    #[doc = r#"Title: Unassisted Mergers"#]
    #[doc = r#"Description: Voluntary mergers, consolidations or absorptions of two or more institutions."#]
    #[serde(rename="UNASSIST")]
    pub unassist: Option<i32>,

    #[doc = r#"Title: Unearned Income"#]
    #[doc = r#"Description: Unearned Income On Loans On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March 1976 Through September 1978 Are Domestic Only"#]
    #[serde(rename="UNINC")]
    pub uninc: Option<i32>,

    #[doc = r#"Title: Unit Banks"#]
    #[doc = r#"Description: Unit banks are institutions that are operating only one office at which deposits are received or other banking business is conducted."#]
    #[serde(rename="UNIT")]
    pub unit: Option<i32>,

    #[doc = r#"Title: Year (Search-Eligible)"#]
    #[doc = r#"Description: Statistics reported as of end of year. This field can be used for search and filtering."#]
    #[serde(rename="YEAR")]
    pub year: Option<String>,

}

/// Auto-generated response envelope struct for `/summary` endpoint.
/// Spec: summary_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SummaryResponse {
    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    #[serde(rename="data")]
    pub data: Option<String>,

}

/// FDIC BankFind API `/summary` endpoint handler
/// Get Historical Aggregate Data by Year
/// Returns aggregate financial and structure data, subtotaled by year, regarding finanical institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter criteria that refines the records included in the calculated result. All values must be entered in UPPERCASE.
example: >-
STNAME:\"Alabama\" AND YEAR:2005
Examples:
* Filter by Community Banks (CB) vs. Savings Institutions (SI)  
`CB_SI:CB`  
* Filter by State name  
`STNAME:\"Virginia\"`    
* Filter all but a specified value  
`!(STNAME:\"Virginia\")`  
* Filter for any one of multiple State names  
`STNAME:(\"West Virginia\",\"Delaware\")`    
* Filter data by the year range  
`YEAR:&#91;\"2015\" TO \"2017\"&#93;`
STNAME:\"Alabama\" - `fields` (String, optional): Comma delimited list of fields with aggregated annual financial data to return. All values must be entered in UPPERCASE.
STNAME,YEAR,INTINC,EINTEXP,NIM,NONII,NONIX,ELNATR,ITAXR,IGLSEC,ITAX,EXTRA,NETINC - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
YEAR - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE. - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE. - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE. - `agg_limit` (i32, optional): The limit on how many aggregated results will be displayed. - `max_value` (String, optional): The field by which the max value is desired. - `max_value_by` (String, optional): The field that will be used to determine unique records, similar to a primary key (i.e. CERT). All values must be entered in UPPERCASE. - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /summary
Parameters: SummaryParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Historical"#]
pub async fn summary_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<SummaryParameters>,
) -> Response {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "summary",
        method = "GET",
        path = "/summary",
        params = serde_json::to_string(&params).unwrap()
    );
    let resp = list_endpoint(
        State(config),
        Query(params.clone()),
        "summary",
    ).await;
    // Log outgoing FDIC API request as structured JSON
    debug!(
        target = "fdic_proxy",
        event = "proxied_fdic_api_request",
        endpoint = "summary",
        method = "GET",
        path = "/summary",
        params = serde_json::to_string(&params).unwrap()
    );
    resp
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = SummaryParameters {
            common: CommonParameters::default(),
            agg_by: None,
            agg_term_fields: None,
            agg_sum_fields: None,
            agg_limit: None,
            max_value: None,
            max_value_by: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }
    #[test]
    fn test_properties_struct_serialization() {
        let props = SummaryProperties {
            
            allother: None,
            alsonew: None,
            asset: None,
            banks: None,
            bkprem: None,
            branches: None,
            branchin: None,
            bro: None,
            brwdmony: None,
            cb_si: None,
            chartoth: None,
            chbal: None,
            chbali: None,
            chrtrest: None,
            comboass: None,
            combos: None,
            cons: None,
            corpbnds: None,
            count: None,
            crlnls: None,
            ddt: None,
            dep: None,
            depdom: None,
            depfor: None,
            depi: None,
            depifor: None,
            depni: None,
            depnifor: None,
            drlnls: None,
            eamintan: None,
            edep: None,
            edepdom: None,
            edepfor: None,
            eerepp: None,
            efhlbadv: None,
            efrepp: None,
            eintexp: None,
            eintexp2: None,
            elnatr: None,
            eothnint: None,
            epremagg: None,
            eq: None,
            eqcdiv: None,
            eqcdivc: None,
            eqcdivp: None,
            eqcs: None,
            eqdiv: None,
            eqnm: None,
            eqnwcert: None,
            eqothcc: None,
            eqpp: None,
            eqsur: None,
            equptot: None,
            esal: None,
            esubnd: None,
            extra: None,
            fd_bif: None,
            fd_saif: None,
            frepo: None,
            frepp: None,
            ichbal: None,
            ifee: None,
            ifrepo: None,
            iglsec: None,
            ilndom: None,
            ilnfor: None,
            ilnls: None,
            ilns: None,
            ils: None,
            intan: None,
            intbast: None,
            intblib: None,
            intinc: None,
            intinc2: None,
            irakeogh: None,
            isc: None,
            iserchg: None,
            itax: None,
            itaxr: None,
            itrade: None,
            liab: None,
            liabeq: None,
            liqasstd: None,
            liqunass: None,
            lnag: None,
            lnalloth: None,
            lnatres: None,
            lnauto: None,
            lnci: None,
            lncon: None,
            lnconot1: None,
            lnconoth: None,
            lncrcd: None,
            lndep: None,
            lnls: None,
            lnlsgr: None,
            lnlsnet: None,
            lnmobile: None,
            lnmuni: None,
            lnre: None,
            lnreag: None,
            lnrecons: None,
            lnredom: None,
            lnrefor: None,
            lnreloc: None,
            lnremult: None,
            lnrenres: None,
            lnreres: None,
            lnresre: None,
            lnsp: None,
            ls: None,
            mergers: None,
            missadj: None,
            mtgls: None,
            nalnls: None,
            nchgrec: None,
            nclnls: None,
            netimin: None,
            netinc: None,
            newcount: None,
            new_char: None,
            new6_1: None,
            new9_1: None,
            new10_1: None,
            new10_2: None,
            new10_3: None,
            new11_1: None,
            new14_1: None,
            new14_2: None,
            new14_3: None,
            new14_4: None,
            new15_1: None,
            new15_2: None,
            new15_3: None,
            new15_4: None,
            new15_5: None,
            new15_7: None,
            new16_1: None,
            new16_2: None,
            nim: None,
            nonii: None,
            nonix: None,
            ntlnls: None,
            ntr: None,
            ntrtime: None,
            ntrtmlg: None,
            numemp: None,
            oea: None,
            offices: None,
            ointbor: None,
            ointexp: None,
            ointinc: None,
            oononii: None,
            ore: None,
            oret: None,
            ot_bif: None,
            ot_saif: None,
            othasst: None,
            othbfhlb: None,
            othborr: None,
            otheq: None,
            other: None,
            othliab: None,
            othnborr: None,
            otlncnta: None,
            paid_off: None,
            p3lnls: None,
            p9lnls: None,
            ptxnoinc: None,
            rel_co: None,
            savings: None,
            sc: None,
            scage: None,
            sceq: None,
            scmtgbk: None,
            scmuni: None,
            scmv: None,
            scres: None,
            scus: None,
            scusa: None,
            scust: None,
            stname: None,
            stnum: None,
            subllpf: None,
            subnd: None,
            tintinc: None,
            tochrt: None,
            tofail: None,
            tointexp: None,
            tomerg: None,
            tortc: None,
            total: None,
            tot_fdic: None,
            tot_ots: None,
            tot_save: None,
            tpd: None,
            trade: None,
            trades: None,
            trn: None,
            unassist: None,
            uninc: None,
            unit: None,
            year: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
