//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/summary` endpoint.

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

/// Auto-generated parameters struct for `/summary` endpoint.
/// Spec: summary_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct SummaryParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[schemars(description = r#"The field by which data will be aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_by: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE."#)]
    pub agg_term_fields: Option<String>,
    #[schemars(description = r#"The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE."#)]
    pub agg_sum_fields: Option<String>,
    #[schemars(description = r#"The limit on how many aggregated results will be displayed."#)]
    pub agg_limit: Option<i32>,
    #[schemars(description = r#"The field by which the max value is desired."#)]
    pub max_value: Option<String>,
    #[schemars(description = r#"The field that will be used to determine unique records, similar to a primary key (i.e. CERT). All values must be entered in UPPERCASE."#)]
    pub max_value_by: Option<String>,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for SummaryParameters {
    fn name() -> &'static str {
        "summary"
    }
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
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct SummaryProperties {
    #[schemars(description = r#"All Other Loans - All Other Loans"#)]
    pub allother: Option<i32>,

    #[schemars(description = r#"New Charters to Absorb Another Charter - New savings institution charter created to absorb any other type of charter in its first quarter of operation."#)]
    pub alsonew: Option<i32>,

    #[schemars(description = r#"Total Assets - Total Assets On A Consolidated Basis
Note: For Banks With Foreign Operations Data For March &
September Of 1973 Through 1975 Are Reported On A
Domestic Basis"#)]
    pub asset: Option<i32>,

    #[schemars(description = r#"Total Commercial Banks (Filing Y/E Call) - Total Insured Commercial Banks filing 12/31 fincncial report  (See Notes to User for definition of commercial bank)"#)]
    pub banks: Option<i32>,

    #[schemars(description = r#"Bank Premises and Equipment - Premises and Fixed Assets
Note:
(1) Premises and Fixed Assets (Including Capitalized Leases) On A Consolidated Basis
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#)]
    pub bkprem: Option<i32>,

    #[schemars(description = r#"Total Branches - Branches include all offices of a bank, other than its head office, at which deposits are received, checks paid or money lent. Banking facilities separate from a banking house, banking facilities at government installations, offices, agencies, paying or receiving stations, drive-in facilities and other facilities operated for limited purposes are defined as branches under the FDI Act (see Notes to User)"#)]
    pub branches: Option<i32>,

    #[schemars(description = r#"Banks with Branches - Banks with branches are institutions that operate one or more offices at which deposits are received or other banking business conducted in addition to the main or head office."#)]
    pub branchin: Option<i32>,

    #[schemars(description = r#"Memo: Brokered Deposits - Borrowed Deposits (Represents funds which the reporting bank obtains, directly or indirectly, by or through any deposit broker for deposit into one or more deposit accounts. Includes both those in which the entire beneficial interest in a given bank deposit account or investment is held by a single depositor and those in which deposit broker sells participation in a given bank deposit account or instrument to one or more investors)."#)]
    pub bro: Option<i32>,

    #[schemars(description = r#"Borrowed Funds - Borrowed Funds - (1969-Present -- Represents Federal Funds purchase. securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premsises.)"#)]
    pub brwdmony: Option<i32>,

    #[schemars(description = r#"Commercial Banks (CB) vs. Savings Institution (SI) (Search-Eligible) - Differentiates the summarised data between the Commercial Banks and the Savings Institutions This field can be used for search and filtering."#)]
    pub cb_si: Option<String>,

    #[schemars(description = r#"Charter Transfers from Commercial Banks - Represents the transfer of a commercial bank to a savings institution charter that meets the definition of a thrift (see Notes to Table SI-1) and has applied for and received FDIC insurance (BIF or SAIF)."#)]
    pub chartoth: Option<i32>,

    #[schemars(description = r#"Cash & Due from Depository Institutions - Total Cash and Balances Due From Depository Institutions Which Include Both Noninterest-Bearing and Interest-Bearing Deposits On A Consolidated Basis
Note:
(1): Additional Detail Can Be Found On Schedule Rc-A
(2) For Banks With Foreign Operations Data For March and September 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub chbal: Option<i32>,

    #[schemars(description = r#"Interest Earning Balances - Interest-Bearing Balances Due From Depository Institutions On A Consolidated Basis
Note: Additional Detail Can Be Found On Schedule Rc-A"#)]
    pub chbali: Option<i32>,

    #[schemars(description = r#"Non-insured Becoming insured - Represents the transfer of an existing institution that does not have deposit insurance to a savings institution charter with FDIC insurance from BIF or SAIF. Examples of such institutions include Trust Banks and savings institutions with state deposit insurance that apply for and receive FDIC insurance"#)]
    pub chrtrest: Option<i32>,

    #[schemars(description = r#"Assisted Mergers with Thrifts - Represents the absorption of a failing savings institution by another savings institution with assistance from either the BIF or SAIF. (Included are RTC Accelerated Resolution Program (ARP) assisted mergers. These institutions were not placed in RTC conservatorship.)"#)]
    pub comboass: Option<i32>,

    #[schemars(description = r#"Unassisted Mergers/Consolidations of Thrifts - Represents the absorption of a savings institution charter by another savings institution without assistance. Both institutions may be owned by the same holding company in a consolidation of affiliates."#)]
    pub combos: Option<i32>,

    #[schemars(description = r#"RTC Conservatorships - Institutions in RTC Conservatorship"#)]
    pub cons: Option<i32>,

    #[schemars(description = r#"Other Debt Securities - Other Debt Securities"#)]
    pub corpbnds: Option<i32>,

    #[schemars(description = r#"Total Savings Institutions (Filing Y/E Call) - All FDIC Insured Savings Institutions filing a 12/31 financial report"#)]
    pub count: Option<i32>,

    #[schemars(description = r#"Loan & Lease Recoveries - Total Recoveries Of Loans and Lease Financing Receivables Credited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#)]
    pub crlnls: Option<i32>,

    #[schemars(description = r#"Deposits - Domestic Demand - Total Demand Deposits Included In Total Transaction Accounts Held In Domestic Offices
Note: For Tfr Filers Between June 1989 Through March 1990 Includes Non-interest Bearing Deposits"#)]
    pub ddt: Option<i32>,

    #[schemars(description = r#"Total Deposits - Total Deposits On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub dep: Option<i32>,

    #[schemars(description = r#"Total Domestic Deposits - Represents the sum of total deposits, domestic offices only"#)]
    pub depdom: Option<i32>,

    #[schemars(description = r#"Total Foreign Deposits - Represents the sum of total deposits in foreign offices"#)]
    pub depfor: Option<i32>,

    #[schemars(description = r#"Interest Bearing Deposits - Interest-Bearing Consolidated Office Deposits

Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) Tfr Filers With Less Than $300 Million In Assets and Risk-Based Capital Ratios In Excess Of 12 Percent Are Not Required To File Schedule Cmr Beginning March 1993, However, When Cmr Data Is Either Incorrect Or Not Filed Fts Assumes That All Deposits Are Interest-Bearing
(3) Prior To Receipt Of The 75-Day Tfr Tape All Tfr Filers Deposits Are Assumed To Be Interest-Bearing
(4) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub depi: Option<i32>,

    #[schemars(description = r#"Foreign Deposits - Interest Bearing - Represents any deposit in foreign offices, whether demand, savings or time, on which the bank pays or accrues interest"#)]
    pub depifor: Option<i32>,

    #[schemars(description = r#"Memo: Deposits - Non-interest Bearing - Represents any deposit on which the bank does not pay or accrue interest"#)]
    pub depni: Option<i32>,

    #[schemars(description = r#"Foreign Deposits - Non-interest Bearing - Represents any deposit in foreign offices on which the bank does not pay or accrue interest"#)]
    pub depnifor: Option<i32>,

    #[schemars(description = r#"Loan & Lease Charge-offs - Total Charged-Off Loans and Lease Financing Receivables Debited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub drlnls: Option<i32>,

    #[schemars(description = r#"Memo: Amortization of Intangibles - Goodwill Impairment Losses and Amortization Expense and Impairment Loss For Other Intangible Assets On A Consolidated Basis

Note:
(1) Prior To March 2001, Listed As Memoranda Only and Is Included In All Other Non-interest Expense
(2) Includes Only Amortization Of Goodwill For Tfr Filers"#)]
    pub eamintan: Option<i32>,

    #[schemars(description = r#"Int Exp - Total Deposits - Interest Expense On Total Deposits (Domestic and Foreign) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub edep: Option<i32>,

    #[schemars(description = r#"Int Exp - Deposit in Domestic Offices - Interest Expense On Total Deposits Held In Domestic Offices
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub edepdom: Option<i32>,

    #[schemars(description = r#"Int Exp - Deposits in Foreign Offices - Deposit Interest Expense-For (1976-Present -- Represents all interests on all liabilities reportable as deposits in foreign offices. -- 1934-1975 -- Interest on foreign office deposits is not available. Reports of income were submitted on a domestic only basis.)"#)]
    pub edepfor: Option<i32>,

    #[schemars(description = r#"Fed Funds Purchased/Securities Sold - Represents the gross expenses of all liabilities reportable under this category"#)]
    pub eerepp: Option<i32>,

    #[schemars(description = r#"Int Exp Oth - Advances From FHLB - Interest Expense and The Amortization Of Any Related Yield Adjustments On Fhlbank Advances
Note: Only Reported By Tfr Filers"#)]
    pub efhlbadv: Option<i32>,

    #[schemars(description = r#"Int Exp - Fed Funds Purchased/Securities Sold - Interest Expense On Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis (Prior To March 1997 Was On A Consolidated Basis In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S)
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#)]
    pub efrepp: Option<i32>,

    #[schemars(description = r#"Total Interest Expense - Total Interest Expense On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#)]
    pub eintexp: Option<i32>,

    #[schemars(description = r#"Eintexp2 - Eintexp2"#)]
    pub eintexp2: Option<i32>,

    #[schemars(description = r#"Provision for Loan & Lease Losses - Provision For Loan & Lease Losses On A Consolidated Basis

Note:
(1) Beginning March 2003, Includes The Provision For Allocated Transfer Risk Related To Loans
(2) From March 1997 To December 2000, Defined As The Provision For Credit Losses & Allocated Transfer Risk Reserve Which Includes The Provision For Off-Balance Sheet Credit Losses For Call Report Filers
(3) Prior To March 1997, Defined As The Provision For Loan and Lease Losses & Allocated Transfer Risk
(4) For Tfr Filers, Consists Of The Provision For Loan and Lease Losses
(5) Reflects Net Provision For Losses On Interest-Bearing Assets For Tfr Filers
(6) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub elnatr: Option<i32>,

    #[schemars(description = r#"All Other Non-interest Expenses - All Other Non-interest Expense On A Consolidated Basis

Note:
(1) Prior To March 2001, Included The Amortization Of Intangible Assets For Call Reporters
(2) Greater Detail Is Provided In Subsequent Data Fields For All Items In Excess Of 10% Of This Item All Other Non-interest Expense On A Consolidated Basis
(3) Does Not Include Losses On Asset Sales For Tfr Filers Beginning June 1996, Such Gains (Losses) Are Included Net In Non-interest Income
(4) Includes Loss On Sale Of Securities Held For Investments For Tfr Filers Between March 1984 Through December 1986
(5) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub eothnint: Option<i32>,

    #[schemars(description = r#"Occupancy Expense - Expenses Of Premises and Fixed Assets (Net Of Rental Income and Excluding Salaries and Employee Benefits and Mortgage Interest) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub epremagg: Option<i32>,

    #[schemars(description = r#"Total Equity - Represents the sum of all capital accounts"#)]
    pub eq: Option<i32>,

    #[schemars(description = r#"Total Cash Dividends Declared - Cash Dividends Declared On Common and Preferred Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub eqcdiv: Option<i32>,

    #[schemars(description = r#"Cash Dividends Declared (Common) - Cash Dividends Declared On Common Stock On A Consolidated Basis

Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub eqcdivc: Option<i32>,

    #[schemars(description = r#"Cash Dividends Declared (Preferred) - Cash Dividends Declared On Preferred Stock On A Consolidated Basis
Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub eqcdivp: Option<i32>,

    #[schemars(description = r#"Common Stock - Common Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub eqcs: Option<i32>,

    #[schemars(description = r#"Total Cash Divident Declared - The total of cash dividends declared on all preferred and common stock during the calendar year, regardless of when payable"#)]
    pub eqdiv: Option<i32>,

    #[schemars(description = r#"Total Equity Capital - Total Capital (Represents the total of all capital components, including FDIC net worth certificates.)"#)]
    pub eqnm: Option<i32>,

    #[schemars(description = r#"FDIC Net Worth Certificates - Net Worth Certificates Represents The Outstanding Balances Issued To The Fdic In Exchange For Promissory Notes Received From The Fdic On A Consolidated Basis"#)]
    pub eqnwcert: Option<i32>,

    #[schemars(description = r#"Other Capital - Other Capital"#)]
    pub eqothcc: Option<i32>,

    #[schemars(description = r#"Perpetual Preferred Stock - Perpetual Preferred Stock and Related Surplus On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub eqpp: Option<i32>,

    #[schemars(description = r#"Surplus - Surplus (Excludes All Surplus Related To Preferred Stock) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis
"#)]
    pub eqsur: Option<i32>,

    #[schemars(description = r#"Undivided Profits - Undivided Profits, Capital Reserves, Net Unrealized Holding Gains (Losses) On Available-For-Sale Securities and Other Equity Capital Components and/Or
Accumulated Gains (Losses) On Cash Flow Hedges On A Consolidated Basis

Note:
(1) Prior To March 1999 Included Undivided Profits, Capital Reserves and Net Unrealized Gains (Losses) On Available-For-Sale Securities
(2) Prior To March 1994 Included Undivided Profits and Capital Reserves Less Net Unrealized Loss On Marketable Equity Securities
(3) This Item Includes Net Worth Certificates For Bif Thrifts
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub equptot: Option<i32>,

    #[schemars(description = r#"Employee Salaries and Benefits - Salaries and Employee Benefits On A Consolidated Basis Note: For Banks With Foreign Operations, Data For December 72 Through December 1975 Are Domestic Only"#)]
    pub esal: Option<i32>,

    #[schemars(description = r#"Int Exp - Subordinated Notes - Interest Expense On Subordinated Notes and Debentures On A Consolidated Basis
Note:
1. This Item Is Not Reported By Form 51 Filers
2. For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub esubnd: Option<i32>,

    #[schemars(description = r#"Net Extraordinary Items - Discontinued Operations, Net Of Applicable Income Taxes On A Consolidated Basis

Note:
(1) Prior To March 2016, Defined As Extraordinary Items and and Other Adjustments, Net Of Taxes On A Consolidated Basis
(2) This Item Does Not Include The Tax Effects Related To Securities Gains and Losses and Extraordinary Items From June 1984 Through December 1985 For Bif Thrifts (Refer To Applicable Income Taxes)
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub extra: Option<i32>,

    #[schemars(description = r#"FDIC Supervised, BIF Insured Institutions - FDIC Supervised, BIF Insured Institutions"#)]
    pub fd_bif: Option<i32>,

    #[schemars(description = r#"FDIC supervised, SAIF Insured Institutions - FDIC supervised SAIF insured institutions"#)]
    pub fd_saif: Option<i32>,

    #[schemars(description = r#"Federal Funds Sold - Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis

Note:
(1) Prior To March 1997, Includes Only Federal Funds Sold and Securities Purchased Under Agreements To Resell In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S
(2) Prior To March 1998, Includes Only Federal Funds Sold For Tfr Filers
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#)]
    pub frepo: Option<i32>,

    #[schemars(description = r#"Fed Funds & Repos Purchased - Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis 
Note:
(1) Prior To March 1998, Includes Only Reverse Repurchase Agreements For Tfr Filers
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub frepp: Option<i32>,

    #[schemars(description = r#"Int Inc - Balances Due - Total Interest Income On Balances Due From Depository Institutions On A Consolidated Basis"#)]
    pub ichbal: Option<i32>,

    #[schemars(description = r#"Fee Income - Fee Income (Represents service charges on deposit accounts such as maintenance fees, activity charges, administrative charges, overdraft charges and check certification charges; mortgage loans servicing fees plus other fees and charges, including prepayment loan fees, late charges, assumption fees, and amortization of commitment fees.)"#)]
    pub ifee: Option<i32>,

    #[schemars(description = r#"Int Inc - Fed Funds Sold/Securities Purchased - Interest Income On Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis
Note:
(1) Prior To March 1997 Included Only Income From Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibfs On A Consolidated Basis
(2) For Banks With Foreign Operations Data For December 1972 Through 1975 Are Domestic Only"#)]
    pub ifrepo: Option<i32>,

    #[schemars(description = r#"Securities Gains and Losses - Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Debt Securities and Unrealized Holding Gains (Losses) On Equity Securities Not Held For Trading Before Adjustments For Income Taxes On A Consolidated Basis (Also Includes Realized Gains On Equity Securities Until The Institution Adopts Asu 2016-01)
Note:
1. Prior To March 2018, Defined As Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Securities Before Adjustments For Income Taxes On A Consolidated Basis
2. Beginning In The 2018 Reporting Year, Includes Unrealized Gains (Losses) On Equity Securities For Institutions That Adopted Asu2016-01 and Includes Realized Gains (Losses) On Equity Securities For Institutions That Have Not Yet Adopted Asu2016-01
3. Prior To March 1994 Defined As Gains (Losses) On Securities Not Held In Trading Accounts 
4. From March 1990 Through March 2009, Includes Gains (Losses) On Assets Held For Sale For Tfr Filers
5. Includes Gains (Losses) On Loans Held For Investment From March 1984 Through December 1989 For Tfr Filers
6. Tfr Filers Report Only Gains From March 1984 Through December 1986
7. For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub iglsec: Option<i32>,

    #[schemars(description = r#"Int Inc - Domestic Office Loans - Total Interest and Fees On Loans Held In Domestic Offices
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub ilndom: Option<i32>,

    #[schemars(description = r#"Int Inc - Foreign Office Loans - Total Interest and Fees On Loans Held In Foreign Offices, Edge and Agreement Subsidiaries, and Ibf'S"#)]
    pub ilnfor: Option<i32>,

    #[schemars(description = r#"Int Inc - Total Loans & Leases - Interest and Fees On Loans and Lease Financing Receivables On A Consolidated Basis
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub ilnls: Option<i32>,

    #[schemars(description = r#"Int Inc - Loans - Loans (Represents all interest, fees and similar charges levied against or associated with all assets reportable as loans. Includes interest, yield related fees, commitment fees, service charges on loans and discount accretion. (One savings bank with an office in Canada has been reporting on the Domestic & Foreign Consolidated Call Report form (FFIEC 031). It does not, however, indicate any income or expenses related to foreign operations.))"#)]
    pub ilns: Option<i32>,

    #[schemars(description = r#"Int Inc - Leases - Total Interest Income From Lease Financing Receivables On A
Consolidated Basis"#)]
    pub ils: Option<i32>,

    #[schemars(description = r#"Intangible Assets - Intangible Assets On A Consolidated Basis"#)]
    pub intan: Option<i32>,

    #[schemars(description = r#"Total Interest Earning Assets - Total Interest Earning Assets (Derived See Si-19) - Sc"#)]
    pub intbast: Option<i32>,

    #[schemars(description = r#"Total Interest Bearing Liabilities - Total Interest Bearing Liabilities (Derived See Si-19) - Sc"#)]
    pub intblib: Option<i32>,

    #[schemars(description = r#"Total Interest Income - Total Interest Income On A Consolidated Basis
Note: For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub intinc: Option<i32>,

    #[schemars(description = r#"INTINC2 - INTINC2"#)]
    pub intinc2: Option<i32>,

    #[schemars(description = r#"Memo: IRA's and Keogh Plan-Deposits - Individual Retirement Accounts (Ira'S) and Keogh Plan
Accounts Held In Domestic Offices
Note: Listed As Memoranda Only"#)]
    pub irakeogh: Option<i32>,

    #[schemars(description = r#"Int Inc - Investment Securities - Total Interest and Dividend Income On: U.S. Treasury Securities, U.S. Government Agency and Corporation Obligations, Securities Issued By States and Political
Subdivision In The U.S., Other Domestic Debt Securities, Foreign Debt Securities, and Equity Securities (Including Investments In Mutual Funds) On A Consolidated Basis
Note:
(1) This Item Includes Interest Income On Deposits For Tfr Filers
(2) Includes Interest Income On Assets Held In Trading Accounts For Tfr Filers For Two Distinct Periods: (A) March 1984 Through December 1989 and (B) June 1996
And Following Quarters.
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub isc: Option<i32>,

    #[schemars(description = r#"Service Charges on Deposit Accounts - Represents service charges on deposit accounts in domestic offices such as maintenance fees, activity charges, administrative charges, overdraft charges, and check certification charges"#)]
    pub iserchg: Option<i32>,

    #[schemars(description = r#"Applicable Income Taxes - Represents Federal, state and local taxes on income. It does not include taxes relating to securities transactions or extraordinary items"#)]
    pub itax: Option<i32>,

    #[schemars(description = r#"Pre-Tax Net Operating Income - Pre-Tax Net Operating Income (Represents Net Interest Income plus Total Non-interest Income less Total Non-interest Expense and the Provision for Loan & Lease Losses.)"#)]
    pub itaxr: Option<i32>,

    #[schemars(description = r#"Int Inc - Trading Account Assets - Interest Income From Assets Held In Trading Accounts On A Consolidated Basis 
Note:
Beginning March 2017, Reported As An Individual Income Category For Form 031 Filers Only and Is Included As A Component Of Other Interest Income For All Other Report Forms"#)]
    pub itrade: Option<i32>,

    #[schemars(description = r#"Total Liabilities - Total Liabilities Including Subordinated Notes and Debentures and Limited Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: Prior To March 2009, This Item Included Noncontrolling (Minority) Interests In Consolidated Subsidiaries For Call Report and Tfr Filers"#)]
    pub liab: Option<i32>,

    #[schemars(description = r#"Total Liabilities and Equity Capital - Total Liabilities, Limited-Life Preferred Stock, and Equity Capital On A Consolidated Basis Note: For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub liabeq: Option<i32>,

    #[schemars(description = r#"Assisted Payouts - Represents all assisted payouts of FDIC-insured savings institutions that are not in RTC conservatorship."#)]
    pub liqasstd: Option<i32>,

    #[schemars(description = r#"Voluntary Liquidations - Represents all instances where the owners of a thrift voluntarily surrender their charter with all liabilities including deposits paid down and all assets sold."#)]
    pub liqunass: Option<i32>,

    #[schemars(description = r#"Agricultural Loans - Loans To Finance Agricultural Production and Other Loans To Farmers On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods Form December 1972 Through September 1978 Are Domestic Only"#)]
    pub lnag: Option<i32>,

    #[schemars(description = r#"All Other Loans to Individuals - All Other Loans (1969-Present -- Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premises.)"#)]
    pub lnalloth: Option<i32>,

    #[schemars(description = r#"Allowance for Losses Loans and Leases - Allowance For Loan and Lease Financing Receivable Losses and Allocated Transfer Risk On A Consolidated Basis
Note:
(1) From March 2001 To Dec 2002 Allocated Transfer Riskis Netted From Loans & Not Included As Part Of The Reserve
(2) Additional Detail Can Be Found On Schedule Ri-B
(3) For Tfr Filers Between March 1984 Through December 1989 Includes Allowance For Mortgage Pool Securities
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#)]
    pub lnatres: Option<i32>,

    #[schemars(description = r#"Memo: Loans to Individuals - Auto - Represents installment loans to purchase private passenger automobiles, both direct loans and purchased paper"#)]
    pub lnauto: Option<i32>,

    #[schemars(description = r#"Commercial and Industrial Loans - Commercial and Industrial Loans On A Consolidated Basis Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#)]
    pub lnci: Option<i32>,

    #[schemars(description = r#"Total Loans to Individuals - Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) On A Consolidated Basis
Note:
(1) For Tfr Filers Includes Revolving Loans Secured By 1-4 Family Dwelling Units From March 1984 Through March 1988
(1) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#)]
    pub lncon: Option<i32>,

    #[schemars(description = r#"Loans to Individuals - Home Improvement - Installment Loans To Individuals To Repair and Modernize
Residential Property Held In Domestic Offices"#)]
    pub lnconot1: Option<i32>,

    #[schemars(description = r#"Loans to Individuals - All Others - Represents all other loans to individuals for household, family and other personal expenditures. It includes auto loans, both direct and indirect, mobile homes (unless secured by a real estate mortgage), education loans, other installment loans both secured by personal property or unsecured, and single payment loans (time or demand, secured or unsecured)"#)]
    pub lnconoth: Option<i32>,

    #[schemars(description = r#"Loans to Individuals - Credit Card Plans - Credit Cards Related Plans On A Consolidated Basis

Note:
(1)Prior To March 2001 Includes Credit Cards Related Plans-Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) Includes Check Credit and Other Revolving Credit Plans
(2) For Tfr Filers Between March 1984 Through March 1988 This Figure Includes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(3) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#)]
    pub lncrcd: Option<i32>,

    #[schemars(description = r#"Loans to Deposit Institutions - Loans To Depository Institutions On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only
Note:(1) Beginning March 2001 Includes Acceptances Of Other Banks
(2) Beginning March 2001, Includes Acceptances Of Other Banks For Ibas"#)]
    pub lndep: Option<i32>,

    #[schemars(description = r#"Gross Loans and Leases - Represents the sum of all components of loans"#)]
    pub lnls: Option<i32>,

    #[schemars(description = r#"Total Loans and Leases - Loans and Lease Financing Receivables, Net Of Unearned Income, On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-C
(2) For Tfr Filers This Item Is Net Of Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through December 1989
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#)]
    pub lnlsgr: Option<i32>,

    #[schemars(description = r#"Net Loans and Leases - Loans and Lease Financing Receivables, Net Of Unearned Income, Allowance, and Reserve On A Consolidated Basis
Note:
(1) For Tfr Filers This Item Is Net Of Valuation Allowances and Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through
(2) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub lnlsnet: Option<i32>,

    #[schemars(description = r#"Memo: Loans to Individuals - Mobile Homes - Represents loans to individuals to purchase mobile homes. (If the bank's security interest in the loan was represented by a mortgage or deed of trust, the loan should be included in real estate loans)"#)]
    pub lnmobile: Option<i32>,

    #[schemars(description = r#"Loans to States and Politicial Sub-divisions - Obligations (Other Than Securities and Leases) Of States and Political Subdivisions In The U.S. (Including Nonrated Industrial Development Obligations) On A Consolidated Basis"#)]
    pub lnmuni: Option<i32>,

    #[schemars(description = r#"Total Real Estate Loans - Loans Secured By Real Estate On A Consolidated Basis
Note:
(1) For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(2) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#)]
    pub lnre: Option<i32>,

    #[schemars(description = r#"R/E Loan - Farmland - Represents loans secured by farmland, including improvements, and other land known to be used or usable for agricultural purposes, as evidenced by mortgages or other liens. It includes loans secured by farmland that are guaranteed by the Farmers Home Administration (FHA) or by the Small Business Administration"#)]
    pub lnreag: Option<i32>,

    #[schemars(description = r#"R/E Loan - Construction & Land Develop - Construction and Land Development Loans Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers Portions Of Lnrecons Were Included In Other Real Estate Loan Categories Prior To March 30, 1986"#)]
    pub lnrecons: Option<i32>,

    #[schemars(description = r#"Total R/E Loans in Domestic Offices - Represents the total of all loans secured by real estate in domestic offices (U.S. and other areas)"#)]
    pub lnredom: Option<i32>,

    #[schemars(description = r#"Total R/E Loans in Foreign Offices - Represents all loans secured by real estate in foreign offices"#)]
    pub lnrefor: Option<i32>,

    #[schemars(description = r#"Memo: Home Equity Loans - Revolving, Open-End Loans Secured By 1-4 Family Residential Properties and Extended Under Lines Of Credit Held In Domestic Offices"#)]
    pub lnreloc: Option<i32>,

    #[schemars(description = r#"R/E Loans -  Multifamily - Multifamily (5 Or More) Residential Properties Secured By Real Estate Held In Domestic Offices"#)]
    pub lnremult: Option<i32>,

    #[schemars(description = r#"R/E Loan -  Non-farm/Non-residential Prop - Nonfarm Nonresidential Properties Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers This Figure Includes Mortgages On Properties That Are Used For Farming"#)]
    pub lnrenres: Option<i32>,

    #[schemars(description = r#"R/E Loan - 1-4 Family - Total Loans Secured By 1-4 Family Residential Properties Held In Domestic Offices
Note: For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower"#)]
    pub lnreres: Option<i32>,

    #[schemars(description = r#"Memo: Contra Account - Allowance For Loan Losses On Real Estate Loans On A Consolidated Basis
Note: For Tfr Filers Includes Allowance For Mortgage Pool Securities Between March 1984 Through December 1989"#)]
    pub lnresre: Option<i32>,

    #[schemars(description = r#"Memo: Loans to Individuals - Single Payment - All loans both time or demand, secured or unsecured, to individuals for personal, family or other household expenditures"#)]
    pub lnsp: Option<i32>,

    #[schemars(description = r#"Leases - Lease Financing Receivables (Net Of Unearned Income) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Call Dates Are Domestic Only Through December 1975"#)]
    pub ls: Option<i32>,

    #[schemars(description = r#"Failures: Assisted Merger - Mergers, consolidations or absorptions entered into as a result of supervisory actions. The transaction may or may not have required FDIC assistance."#)]
    pub mergers: Option<i32>,

    #[schemars(description = r#"Other Misc. Adjustments - Represents any FDIC-insured savings institution that did not file a financial report during the year in which the charter was added or deleted."#)]
    pub missadj: Option<i32>,

    #[schemars(description = r#"Mortgage and Other Borrowings - Represents mortgage indebtedness and liabilities under capitalized leases"#)]
    pub mtgls: Option<i32>,

    #[schemars(description = r#"Non-accrual Loans & Leases - Total Nonaccrual Loans and Lease Financing Receivables On A Consolidated Basis"#)]
    pub nalnls: Option<i32>,

    #[schemars(description = r#"Net Loans and Leases Charge-offs - Net Loans and Leases Charge Offs (-- 1984-1989 -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries. Not collected by TFR filers. -- 1990-Present -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries.)"#)]
    pub nchgrec: Option<i32>,

    #[schemars(description = r#"Total Non-current Loans & Leases - Total Loans and Lease Financing Receivables 90 Days Or More Past Due and Nonaccrual On A Consolidated Basis
Note: Includes Delinquent Loans (60 Or More Days Overdue) and Past Due Loans (One Or More Payments Missed) For Tfr Filers Prior To March 1990"#)]
    pub nclnls: Option<i32>,

    #[schemars(description = r#"Net Income Attributable to Noncontrolling Interests - Net income (loss) attributable to noncontrolling (minority) interests on a consolidated basis."#)]
    pub netimin: Option<i32>,

    #[schemars(description = r#"Net Income - Net Income Attributable To The Bank On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub netinc: Option<i32>,

    #[schemars(description = r#"New Charters - Institutions newly chartered by federal or state banking authorities including authorities in the U. S. Territories or possessions."#)]
    pub newcount: Option<i32>,

    #[schemars(description = r#"New Charters - Institutions newly licensed or chartered by the Office of the Comptroller of the Currency (national banks) or by state banking authorities, including banking authorities in the U. S. territories or possessions. Includes de novo institutions as well as charters issued to take over a failing institution."#)]
    pub new_char: Option<i32>,

    #[schemars(description = r#"Int Exp - Borrowed Money - Represents interest expense related to demand notes issued to the U. S. Treasury, mortgage indebtedness, obligations under capitalized leases, and other borrowed money"#)]
    pub new6_1: Option<i32>,

    #[schemars(description = r#"All Other Assets - Represents all other assets not included in previously mentioned captions. Includes, for the most part, customers' liabilities on acceptances outstanding, income earned not collected as well as any other asset not included above"#)]
    pub new9_1: Option<i32>,

    #[schemars(description = r#"Corporate Bonds and Other Securities - Represents all securities, bonds, notes and debentures of domestic and foreign corporations. Also includes privately issued or guaranteed mortgage backed securities and certain detached U.S. Government security coupons held as a result of either their purchase or the bank's stripping them (CATS, TIGRs, COUGARs, LIONs and ETRs)."#)]
    pub new10_1: Option<i32>,

    #[schemars(description = r#"Trading Account Securities - Securities within the scope of ASC Topic 320, Investments – Debt Securities, that a bank has elected to report at fair value under a fair value option with changes in fair value reported in current earnings should be classified as trading securities. (https://www.fdic.gov/regulations/resources/call/crinst/2018-06/031-041-618rc-d-063018.pdf)"#)]
    pub new10_2: Option<i32>,

    #[schemars(description = r#"Memo: Valuation Reserves - For all years except 1969-1973, investment securities are reflected net of general valuation reserves. Specific reserves are deducted from each security so reserved"#)]
    pub new10_3: Option<i32>,

    #[schemars(description = r#"All Other Loans - Represents unplanned overdrafts and loans to: brokers and dealers in securities, any borrower for the purpose of purchasing and carrying securities, nonprofit institutions and organizations, individuals for investment purposes, real estate investment trusts, mortgage companies holding companies of depository institutions, insurance companies, finance companies, factors and other financial intermediaries, federally sponsored lending agencies, investment banks, the bank's own trust department, Small Business Investment Companies, foreign governments and official institutions, and any other loan not included in one of the above categories"#)]
    pub new11_1: Option<i32>,

    #[schemars(description = r#"Borrowed Funds - Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money"#)]
    pub new14_1: Option<i32>,

    #[schemars(description = r#"Other Liabilities - Includes all liabilities not included above and limited life preferred stock"#)]
    pub new14_2: Option<i32>,

    #[schemars(description = r#"Total Liabilities - Represents the total of all components of liabilities"#)]
    pub new14_3: Option<i32>,

    #[schemars(description = r#"Undivided Profits - Represents undivided profits and related accounts"#)]
    pub new14_4: Option<i32>,

    #[schemars(description = r#"Deposits - Individuals, Partnerships and Corporations - Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#)]
    pub new15_1: Option<i32>,

    #[schemars(description = r#"Deposits - U.S. Government - Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#)]
    pub new15_2: Option<i32>,

    #[schemars(description = r#"Deposits - States and Political Subdivisions - Represents all deposits of states, counties and municipalities in domestic offices. Such deposits, if any, in foreign offices are not separately reported"#)]
    pub new15_3: Option<i32>,

    #[schemars(description = r#"Deposits - All Other - Represents all other deposits. Includes deposits of financial institutions, both domestic and foreign, deposits of foreign governments and official institutions and certified and official checks. Also includes deposits in foreign offices other than those of individuals, partnerships and corporations"#)]
    pub new15_4: Option<i32>,

    #[schemars(description = r#"Deposits - Domestic Savings - Represents all savings deposits in domestic offices"#)]
    pub new15_5: Option<i32>,

    #[schemars(description = r#"Total Domestic Deposits - Total Domestic Deposits"#)]
    pub new15_7: Option<i32>,

    #[schemars(description = r#"Demand Notes and Other Liabilities - Represents demand notes issued to the U.S. Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers"#)]
    pub new16_1: Option<i32>,

    #[schemars(description = r#"Interest Bearing Deposits - Represents any deposit in domestic and foreign offices on which the banks pays or accrues interest"#)]
    pub new16_2: Option<i32>,

    #[schemars(description = r#"Net Interest Income - Net Interest Income (Total Interest Income Minus Total Interest Expense) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub nim: Option<i32>,

    #[schemars(description = r#"Total Non-interest Income - Total Non-interest Income On A Consolidated Basis
Note:
(1) From March 1990 Through March 2009, Excludes Gains (Losses) On Assets Held For Sale For Tfr Filers, See Tfr Instructions For So430
(2) Excludes Gains On The Sale Of Loans Held For Investments From March 1984 Through December 1989 For Tfr Filers
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub nonii: Option<i32>,

    #[schemars(description = r#"Total Non-interest Expense - Total Non-interest Expense On A Consolidated Basis
Note:
(1) Excludes Losses On Asset Sales For Tfr Filers Beginning June 1996
(2) Includes Loss On Sale Of Mortgage Pool and Other Securities Held For Investment For Tfr Filers From March 1984 Through December 1986
(3) Excludes Losses On Loans Held For Investment For Tfr Filers From March 1987 Through December 1989
(4) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#)]
    pub nonix: Option<i32>,

    #[schemars(description = r#"Net Loan & Lease Charge-offs - Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in parentheses indicates net recoveries. Not collected by TFR filers"#)]
    pub ntlnls: Option<i32>,

    #[schemars(description = r#"Memo: Domestic Deposits Non-Transaction - Represents deposits that are not included in the definition of transaction accounts above or that do not satisfy the criteria necessary to be defined as a transaction account. MMDA's are specifically defined as nontransaction accounts"#)]
    pub ntr: Option<i32>,

    #[schemars(description = r#"Deposits - Domestic Time - Represents all time certificates of deposit, time open accounts and similar deposits in domestic offices"#)]
    pub ntrtime: Option<i32>,

    #[schemars(description = r#"Memo: Time Deposits (Over $100K) - Time Deposits Over $100,000 Or More Held In Domestic Offices
Note:
(1) Listed As Memoranda Only and Is Included In Total Nontransaction Accounts
(2) Prior To March 2007, Includes All Deposits (Not Just Time) Greater Than $100,000 For Tfr Filers. Except For December 2006, Includes All Nonretirement Deposits Over
$100,000 and All Retirement Deposits Over $250,000 For Tfr Filers
(3) Includes Time Deposits Of $100,000 Or More"#)]
    pub ntrtmlg: Option<i32>,

    #[schemars(description = r#"Number of Full Time Employees - Number Of Full Time-Equivalent Employees On Payroll At The End Of The Current Period
Note:
(1) Listed As Memoranda Only
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub numemp: Option<i32>,

    #[schemars(description = r#"Other Earning Assests - Other Earning Assests (-- 1984-1989 -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Items not separately reported by TFR filers. They are included in Secruties. -- 1990-Present -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Includes only Federal funds sold for TFR filers. Repurchase agreements are included in Securities.)"#)]
    pub oea: Option<i32>,

    #[schemars(description = r#"Offices - Offices include: Multiple service offices, Military facilities, Drive-in facilities, Loan production offices, Consumer credit offices, Seasonal offices, Administrative offices, Messenger service offices, Supermarket banking offices, and Other offices."#)]
    pub offices: Option<i32>,

    #[schemars(description = r#"Demand Notes and Other Borrowings - Demand Notes and Other Borrowings (Represents demand notes issued to US Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers.)"#)]
    pub ointbor: Option<i32>,

    #[schemars(description = r#"Total Other Interest Expenses - Total Other Interest Expenses (Federal Funds Purchased and Securities Sold -- Represents the gross expense of all liabilities reportable under this category. This item is not reported separately by TFR filers. It is included in Borrowed Money)."#)]
    pub ointexp: Option<i32>,

    #[schemars(description = r#"Int Inc - Total Other - Total Other Interest Income (Represents the total of all Other Interest Income components)."#)]
    pub ointinc: Option<i32>,

    #[schemars(description = r#"Other Non-interest Income - Other Non Interest Income (1984-1989 -- Same as Total Other Interest Income except gains on the sale of loans held for investment are excluded for TFR filers. -- 1990- Present -- Represents income derived from the sale of assets held for sale; office building operations; real estate held for investment; REO operations; LOCOM adjustments made to assets held for sale; net income (loss) from investements in service corporations/subsidiaries (other than operating or finance subsidiaires); leasing operations; realized and unrealized gains (losses) on trading assets; gains on the sale of REO real estate held for investment, and loans held for investment; and the amoritization of deferred gains (losses) on asset hedges.)"#)]
    pub oononii: Option<i32>,

    #[schemars(description = r#"Other Real Estate Owned - Other Real Estate Owned On A Consolidated Basis
Note:
(1) Prior To June 2009, Includes Direct and Indirect Investments In Real Estate
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#)]
    pub ore: Option<i32>,

    #[schemars(description = r#"Other Real Estate - Other Real Estate (Represents other real estate owned net of reserves for losses). Not available for 1997. For 1986 through 1988 ORET = ORE + INVSUORE; for all other years ORET = ORE"#)]
    pub oret: Option<i32>,

    #[schemars(description = r#"Non FDIC Supervised BIF Insured Institutions - Non FDIC supervised BIF insured institutions"#)]
    pub ot_bif: Option<i32>,

    #[schemars(description = r#"Non FDIC Supervised SAIF Insured Institutions - Non FDIC supervised SAIF insured institutions"#)]
    pub ot_saif: Option<i32>,

    #[schemars(description = r#"All Other Assets - All Other Assets (Same as Other Real Estate except that investment in service corporations/subsidiaries is reported gross of valuation allowances by TFR filers, and assets held in trading accounts are included in Securities for TFR filers. -- 1990-Present -- Represents all associations assets not previously mentioned. Includes all non real estate repossessed property, investment in service corporations/subsidiaries, property leased to others, income earned but not yet collected, assets held in the trading accounts, and miscellaneous assets) For 2009- present OTHASST = SUM (INVSUB + INVSUORE + CUSLI + OA)"#)]
    pub othasst: Option<i32>,

    #[schemars(description = r#"Advances from FHLB - Other Liabilities From The Fhlb
Note:Prior To March 2001 Only Reported On Tfrs"#)]
    pub othbfhlb: Option<i32>,

    #[schemars(description = r#"Int Exp Oth - Borrowed Money - Borrowed Money (Represents interest expense related to demand notes issued the US Treasury, mortage indebtedness, obligations under capitalized leases and on other borrowed money."#)]
    pub othborr: Option<i32>,

    #[schemars(description = r#"Other Equity - Represents all equity securities not held for trading: investment in mutual funds, common stock of FNMA, Student Loan Marketing Association, Federal Home Loan Mortgage Corporation, Federal Reserve Bank stock, Federal Home Loan Bank stock, minority interests not meeting the definition of associated companies, "restricted" stock, and other equity securities in both domestic and foreign corporations
"#)]
    pub otheq: Option<i32>,

    #[schemars(description = r#"Other - Withdrawals from FDIC insurance, voluntary liquidations, or conversions to institutions that are not considered commercial banks. Also includes relocation of banks from one state to another."#)]
    pub other: Option<i32>,

    #[schemars(description = r#"Other Liabilities - Other Liabilities (Includes all liabilities not included above and limited life preferred stock. 2001- present -- Includes OTHER LIAB & MINOR IN SUBS)."#)]
    pub othliab: Option<i32>,

    #[schemars(description = r#"Borrowed Funds - Borrowed Funds (Includes federal funds purchased, securities sold under agreements to repurchase (reverse repurchase agreements), demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. Includes only reverse purchase agreements (securities sold under agreements to repurchase) and FSLIC net worth certificates for TFR filers)"#)]
    pub othnborr: Option<i32>,

    #[schemars(description = r#"Less: Other Contra Accounts - Other Contracts (Represents amount reported by savings institutions that file on the Thrift Financial Report. Contra accounts include accrued interest receivable, unamortized yield adjustments and valuation allowances. Negative amounts reflect unamortized premiums and deferred direct costs exceeding unamortized discounts and deferred loan fees)."#)]
    pub otlncnta: Option<i32>,

    #[schemars(description = r#"Failures: Paid Off - Institutions that were declared insolvent, the insured deposits of which were paid by the FDIC."#)]
    pub paid_off: Option<i32>,

    #[schemars(description = r#"Loans & Leases P/D 30-89 Days - Total Loans and Lease Financing Receivables Past Due 30 Through 89 Days and Still Accruing Interest On A Consolidated Basis
Note:
(1) Prior To March 2001,This Information On An Institution Level Is Considered Confidential By The Ffiec"#)]
    pub p3lnls: Option<i32>,

    #[schemars(description = r#"Loans & Leases P/D 90+ Days - Total Loans and Lease Financing Receivables Past Due 90 Or More Days and Still Accruing Interest On A Consolidated Basis"#)]
    pub p9lnls: Option<i32>,

    #[schemars(description = r#"Pre-Tax Net Operating Income - Pre-Tax Net Operating Income"#)]
    pub ptxnoinc: Option<i32>,

    #[schemars(description = r#"Conversions - Conversions of existing institutions of any type that meet the definition of commercial banks (see Definition of Total Commercial Banks and have applied for and received FDIC insurance. Also includes bank relocations from one state to another."#)]
    pub rel_co: Option<i32>,

    #[schemars(description = r#"Total Savings Institutions (Total Insured) - Total Insured Savings Institutions including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)."#)]
    pub savings: Option<i32>,

    #[schemars(description = r#"Total Investment Securities (Book Value) - Total Securities: The Sum Of Held-To-Maturity Securities At Amortized Cost, Available-For-Sale Securities At Fair Value and Equity Securities With Readily Determinable Fair Values Not Held For Trading On A Consolidated Basis
Note:
1. Prior To March 2018, Defined As Total Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities (Excludes Assets Held In Trading Accounts) On A Consolidated Basis
2. Beginning In 2018, Includes Equity Securities For Institutions That Have Adopted Asu2016-01 and Those Institutions That Have Not Yet Adopted This Accounting
Standard
3. Prior To March 1994 Item Defined As Book Value
4. Additional Detail Can Be Found On Schedule Rc-B
5. For Tfr Filers Between March 1984 Through December 1989 Includes Interest-Earning Deposits In Fhlbs, Other Interest-Earning Deposits, Federal Funds Sold and Assets Held In Trading Accounts
6. For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#)]
    pub sc: Option<i32>,

    #[schemars(description = r#"U.S. Agencies and Corporation Securities - Total U.S. Government Agency and Corporation Obligations On A Consolidated Basis
Note:
1) From June 2009 Through December 2010, This Item Excluded Other Commercial
Mortgage-Backed Securities
2) Prior To June 2009, This Item Included Other Commercial Mortgage-Backed Securities
3) Beginning March 1994 Consists Of Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities
4) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
5) Includes U.S. Treasury Securities For Tfr Filers Between March 1984 Through December 1989 and After March 1996
6) Does Not Include Mortgage Derivative Securities For Tfr Filers Between March 1984 Through December 1986
7) For Banks With Foreign Operations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#)]
    pub scage: Option<i32>,

    #[schemars(description = r#"Equity Securities - Total Equity Securities Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning March 2018 Does Not Include Equity Securities For Institutions That Have Adopted Asu 2016-01 See Sceqfv
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers"#)]
    pub sceq: Option<i32>,

    #[schemars(description = r#"Memo: Mortgage Backed Securities - Mortgage Backed Securities On A Consolidated Basis
Includes:
(1) U.S. Government Agency and Corporation Obligations Issued Or Guaranteed Certificates Of Participation In Pools Of Residential Mortgages,
(2) U.S. Government Agency and Corporation Obligations Collateralized Mortgage Obligations Issued By Fnma and Fhlmc (Including Remics)
(3) Other Domestic Debt Securities - Private (I.E., Non-Government-Issued-Or-Guaranteed) Certificates Of Participations In Pools Of Residential Mortgages, and
(4) Other Domestic Debt Securities - Privately-Issued Collateralized Mortgage Obligations (Including Remics)"#)]
    pub scmtgbk: Option<i32>,

    #[schemars(description = r#"States and Political Subdivisions Securities - Total Securities Issued By States and Political Subdivisions Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Prior To March 1994 Item Was Defined As Book Value
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(3) For Banks With Foreign Opeations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#)]
    pub scmuni: Option<i32>,

    #[schemars(description = r#"Market Values - Represents the market (fair) value of all investment securities"#)]
    pub scmv: Option<i32>,

    #[schemars(description = r#"Less: Contra Accounts - Contra-Assets To Securities (Reserves)
Note: For Tfr Filers Only"#)]
    pub scres: Option<i32>,

    #[schemars(description = r#"U.S. Treasury & Agency - Total U.S. Treasury Securities and U.S. Government Agency and Corporation Obligations On A Consolidated Basis
Note:
1) From June 2009 Through December 2010 This Item Excluded Commercial Mortgage Backed Securities
2) Prior To June 2009, This Item Included Commercial Mortgage Backed Securities
3) Beginning March 1994 Consists Of Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities
4) Does Not Include Mortgage Derivative Securities From March 1984 Through December 1986 For Tfr Filers
5) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
6) For Banks With Foreign Operations Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#)]
    pub scus: Option<i32>,

    #[schemars(description = r#"Securities Of Us Agencies - Securities Of Us Agencies"#)]
    pub scusa: Option<i32>,

    #[schemars(description = r#"U.S. Treasury Securities - U.S. Treasury Securities Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning June 1996, Tfr Filers No Longer Report U.S. Treasury Securities Separately
(2) Prior To March 1994 Item Was Defined As Book Value
(3) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(4) For Banks With Foreign Operations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#)]
    pub scust: Option<i32>,

    #[schemars(description = r#"Locations (Search-Eligible) - Locations This field can be used for search and filtering."#)]
    pub stname: Option<String>,

    #[schemars(description = r#"State Number (Search-Eligible) - State Number This field can be used for search and filtering."#)]
    pub stnum: Option<String>,

    #[schemars(description = r#"Subordinated Notes - Subordinated Notes and Debentures and Limited-Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: (1) Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domesitc Basis"#)]
    pub subllpf: Option<i32>,

    #[schemars(description = r#"Subordinated Notes/Debentures - Represents all notes and debentures subordinated to deposits and all capital notes and debentures"#)]
    pub subnd: Option<i32>,

    #[schemars(description = r#"Int Inc - Total Other - Total Other Interest Income (Represents the sum of Other Interest Income - Investment Securities, Trading Account Assets, Federal Funds Sold and Securities Purchased, and Balanaces Due from Depository Institutions)"#)]
    pub tintinc: Option<i32>,

    #[schemars(description = r#"Charter Transfers to Commercial Banks - Represents the charter transfer of existing FDIC-insured savings institutions to an FDIC-insured commercial bank charter."#)]
    pub tochrt: Option<i32>,

    #[schemars(description = r#"Assisted Mergers with Commercial Banks - Represents the absorption of a failing savings institution by a commercial bank with assistance from either the BIF or SAIF."#)]
    pub tofail: Option<i32>,

    #[schemars(description = r#"Int Exp  - Total Deposits - Total Other Interest Expense (Represents the sum of all components of Other Interest Expense)"#)]
    pub tointexp: Option<i32>,

    #[schemars(description = r#"Unassisted Mergers with Commercial Banks - Represents the absorption of a savings institution charter by a commercial bank without assistance."#)]
    pub tomerg: Option<i32>,

    #[schemars(description = r#"Failures Transferred to the RTC - Represents institutions that were declared failed and placed under RTC conservatorship until a buyer(s) is(are) found or a payout to depositors occurs."#)]
    pub tortc: Option<i32>,

    #[schemars(description = r#"Total Commercial Banks (Total Insured) - Total Insured Commercial Banks including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)"#)]
    pub total: Option<i32>,

    #[schemars(description = r#"Total FDIC Supervised Savings Institutions - Total FDIC Supervised Savings Institutions"#)]
    pub tot_fdic: Option<i32>,

    #[schemars(description = r#"Total Non FDIC Supervised Savings Institutions - Total Non FDIC Supervised Savings Institutions"#)]
    pub tot_ots: Option<i32>,

    #[schemars(description = r#"Total Savings Institutions - All FDIC Insured Savings Institutions filing a 12/31 financial report"#)]
    pub tot_save: Option<i32>,

    #[schemars(description = r#"Total Loans and Leases Past Due - Total Loans and Leases Past Due"#)]
    pub tpd: Option<i32>,

    #[schemars(description = r#"Trading Account Assets - Assets Held In Trading Accounts On A Consolidated Basis
Note:
(1) Effective March 1994 Item Reported On A Gross Basis
(2) Additional Detail Can Be Found On Schedule Rc-D
(3) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis,
(4) For Periods 1972 Through 1983 Includes Only Securities"#)]
    pub trade: Option<i32>,

    #[schemars(description = r#"Less: Trading Accounts - Trading Accounts"#)]
    pub trades: Option<i32>,

    #[schemars(description = r#"Memo: Domestic Deposits Transaction - Represents all demand deposits, NOW accounts, ATS accounts, accounts from which payments may be made to third parties by means of an automated teller machine, a remote service unit, or another electronic device, and accounts that permit third party payments through use of checks, drafts, negotiable instruments, or other similar instrument. (MMDA's are specifically excluded from the latter two definitions)"#)]
    pub trn: Option<i32>,

    #[schemars(description = r#"Unassisted Mergers - Voluntary mergers, consolidations or absorptions of two or more institutions."#)]
    pub unassist: Option<i32>,

    #[schemars(description = r#"Unearned Income - Unearned Income On Loans On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March 1976 Through September 1978 Are Domestic Only"#)]
    pub uninc: Option<i32>,

    #[schemars(description = r#"Unit Banks - Unit banks are institutions that are operating only one office at which deposits are received or other banking business is conducted."#)]
    pub unit: Option<i32>,

    #[schemars(description = r#"Year (Search-Eligible) - Statistics reported as of end of year. This field can be used for search and filtering."#)]
    pub year: Option<String>,

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct SummaryResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
    pub totals: ResponseTotals,
}

impl IntoContents for SummaryResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize SummaryResponse to Content")]
    }
}

/// FDIC BankFind API `/summary` endpoint handler
/// Get Historical Aggregate Data by Year
/// Returns aggregate financial and structure data, subtotaled by year, regarding finanical institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
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
pub async fn summary_handler(config: &FdicApiConfig, params: &SummaryParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "summary",
        method = "GET",
        path = "/summary",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response::<_, SummaryResponse>(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp.and_then(|r| r.into_call_tool_result())
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
