//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/summary` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/summary` endpoint.
/// Spec: summary_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct SummaryParameters {
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
    #[doc = r#"The limit on how many aggregated results will be displayed."#]
    #[param(rename = "agg_limit")]
    pub agg_limit: Option<u32>,
    #[doc = r#"The field by which the max value is desired."#]
    #[param(rename = "max_value")]
    pub max_value: Option<String>,
    #[doc = r#"The field that will be used to determine unique records, similar to a primary key (i.e. CERT). All values must be entered in UPPERCASE."#]
    #[param(rename = "max_value_by")]
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
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct SummaryProperties {
    #[doc = r#"## FDIC Field:: `ALLOTHER`
    Title: All Other Loans
    Description: All Other Loans"#]
    #[serde(rename="ALLOTHER")]
    pub all_other_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `alsonew`
    Title: New Charters to Absorb Another Charter
    Description: New savings institution charter created to absorb any other type of charter in its first quarter of operation."#]
    #[serde(rename="alsonew")]
    pub new_charters_to_absorb_another_charter: Option<i64>,

    #[doc = r#"## FDIC Field:: `ASSET`
    Title: Total Assets
    Description: Total Assets On A Consolidated Basis
Note: For Banks With Foreign Operations Data For March &
September Of 1973 Through 1975 Are Reported On A
Domestic Basis"#]
    #[serde(rename="ASSET")]
    pub total_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `BANKS`
    Title: Total Commercial Banks (Filing Y/E Call)
    Description: Total Insured Commercial Banks filing 12/31 fincncial report  (See Notes to User for definition of commercial bank)"#]
    #[serde(rename="BANKS")]
    pub total_commercial_banks_filing_y_e_call: Option<i64>,

    #[doc = r#"## FDIC Field:: `BKPREM`
    Title: Bank Premises and Equipment
    Description: Premises and Fixed Assets
Note:
(1) Premises and Fixed Assets (Including Capitalized Leases) On A Consolidated Basis
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="BKPREM")]
    pub bank_premises_and_equipment: Option<i64>,

    #[doc = r#"## FDIC Field:: `BRANCHES`
    Title: Total Branches
    Description: Branches include all offices of a bank, other than its head office, at which deposits are received, checks paid or money lent. Banking facilities separate from a banking house, banking facilities at government installations, offices, agencies, paying or receiving stations, drive-in facilities and other facilities operated for limited purposes are defined as branches under the FDI Act (see Notes to User)"#]
    #[serde(rename="BRANCHES")]
    pub total_branches: Option<i64>,

    #[doc = r#"## FDIC Field:: `BRANCHIN`
    Title: Banks with Branches
    Description: Banks with branches are institutions that operate one or more offices at which deposits are received or other banking business conducted in addition to the main or head office."#]
    #[serde(rename="BRANCHIN")]
    pub banks_with_branches: Option<i64>,

    #[doc = r#"## FDIC Field:: `BRO`
    Title: Memo: Brokered Deposits
    Description: Borrowed Deposits (Represents funds which the reporting bank obtains, directly or indirectly, by or through any deposit broker for deposit into one or more deposit accounts. Includes both those in which the entire beneficial interest in a given bank deposit account or investment is held by a single depositor and those in which deposit broker sells participation in a given bank deposit account or instrument to one or more investors)."#]
    #[serde(rename="BRO")]
    pub memo_brokered_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `BRWDMONY`
    Title: Borrowed Funds
    Description: Borrowed Funds - (1969-Present -- Represents Federal Funds purchase. securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premsises.)"#]
    #[serde(rename="BRWDMONY")]
    pub borrowed_funds: Option<i64>,

    #[doc = r#"## FDIC Field:: `CB_SI`
    Title: Commercial Banks (CB) vs. Savings Institution (SI)
    Description: Differentiates the summarised data between the Commercial Banks and the Savings Institutions"#]
    #[serde(rename="CB_SI")]
    pub commercial_banks_cb_vs_savings_institution_si: Option<String>,

    #[doc = r#"## FDIC Field:: `chartoth`
    Title: Charter Transfers from Commercial Banks
    Description: Represents the transfer of a commercial bank to a savings institution charter that meets the definition of a thrift (see Notes to Table SI-1) and has applied for and received FDIC insurance (BIF or SAIF)."#]
    #[serde(rename="chartoth")]
    pub charter_transfers_from_commercial_banks: Option<i64>,

    #[doc = r#"## FDIC Field:: `CHBAL`
    Title: Cash & Due from Depository Institutions
    Description: Total Cash and Balances Due From Depository Institutions Which Include Both Noninterest-Bearing and Interest-Bearing Deposits On A Consolidated Basis
Note:
(1): Additional Detail Can Be Found On Schedule Rc-A
(2) For Banks With Foreign Operations Data For March and September 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="CHBAL")]
    pub cash_due_from_depository_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `CHBALI`
    Title: Interest Earning Balances
    Description: Interest-Bearing Balances Due From Depository Institutions On A Consolidated Basis
Note: Additional Detail Can Be Found On Schedule Rc-A"#]
    #[serde(rename="CHBALI")]
    pub interest_earning_balances: Option<i64>,

    #[doc = r#"## FDIC Field:: `chrtrest`
    Title: Non-insured Becoming insured
    Description: Represents the transfer of an existing institution that does not have deposit insurance to a savings institution charter with FDIC insurance from BIF or SAIF. Examples of such institutions include Trust Banks and savings institutions with state deposit insurance that apply for and receive FDIC insurance"#]
    #[serde(rename="chrtrest")]
    pub non_insured_becoming_insured: Option<i64>,

    #[doc = r#"## FDIC Field:: `comboass`
    Title: Assisted Mergers with Thrifts
    Description: Represents the absorption of a failing savings institution by another savings institution with assistance from either the BIF or SAIF. (Included are RTC Accelerated Resolution Program (ARP) assisted mergers. These institutions were not placed in RTC conservatorship.)"#]
    #[serde(rename="comboass")]
    pub assisted_mergers_with_thrifts: Option<i64>,

    #[doc = r#"## FDIC Field:: `combos`
    Title: Unassisted Mergers/Consolidations of Thrifts
    Description: Represents the absorption of a savings institution charter by another savings institution without assistance. Both institutions may be owned by the same holding company in a consolidation of affiliates."#]
    #[serde(rename="combos")]
    pub unassisted_mergers_consolidations_of_thrifts: Option<i64>,

    #[doc = r#"## FDIC Field:: `CONS`
    Title: RTC Conservatorships
    Description: Institutions in RTC Conservatorship"#]
    #[serde(rename="CONS")]
    pub rtc_conservatorships: Option<i64>,

    #[doc = r#"## FDIC Field:: `CORPBNDS`
    Title: Other Debt Securities
    Description: Other Debt Securities"#]
    #[serde(rename="CORPBNDS")]
    pub other_debt_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `COUNT`
    Title: Total Savings Institutions (Filing Y/E Call)
    Description: All FDIC Insured Savings Institutions filing a 12/31 financial report"#]
    #[serde(rename="COUNT")]
    pub total_savings_institutions_filing_y_e_call: Option<i64>,

    #[doc = r#"## FDIC Field:: `CRLNLS`
    Title: Loan & Lease Recoveries
    Description: Total Recoveries Of Loans and Lease Financing Receivables Credited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="CRLNLS")]
    pub loan_lease_recoveries: Option<i64>,

    #[doc = r#"## FDIC Field:: `DDT`
    Title: Deposits - Domestic Demand
    Description: Total Demand Deposits Included In Total Transaction Accounts Held In Domestic Offices
Note: For Tfr Filers Between June 1989 Through March 1990 Includes Non-interest Bearing Deposits"#]
    #[serde(rename="DDT")]
    pub deposits_domestic_demand: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEP`
    Title: Total Deposits
    Description: Total Deposits On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="DEP")]
    pub total_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPDOM`
    Title: Total Domestic Deposits
    Description: Represents the sum of total deposits, domestic offices only"#]
    #[serde(rename="DEPDOM")]
    pub total_domestic_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPFOR`
    Title: Total Foreign Deposits
    Description: Represents the sum of total deposits in foreign offices"#]
    #[serde(rename="DEPFOR")]
    pub total_foreign_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPI`
    Title: Interest Bearing Deposits
    Description: Interest-Bearing Consolidated Office Deposits

Note:
(1) Additional Detail Can Be Found On Schedule Rc-E
(2) Tfr Filers With Less Than $300 Million In Assets and Risk-Based Capital Ratios In Excess Of 12 Percent Are Not Required To File Schedule Cmr Beginning March 1993, However, When Cmr Data Is Either Incorrect Or Not Filed Fts Assumes That All Deposits Are Interest-Bearing
(3) Prior To Receipt Of The 75-Day Tfr Tape All Tfr Filers Deposits Are Assumed To Be Interest-Bearing
(4) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="DEPI")]
    pub interest_bearing_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPIFOR`
    Title: Foreign Deposits - Interest Bearing
    Description: Represents any deposit in foreign offices, whether demand, savings or time, on which the bank pays or accrues interest"#]
    #[serde(rename="DEPIFOR")]
    pub foreign_deposits_interest_bearing: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPNI`
    Title: Memo: Deposits - Non-interest Bearing
    Description: Represents any deposit on which the bank does not pay or accrue interest"#]
    #[serde(rename="DEPNI")]
    pub memo_deposits_non_interest_bearing: Option<i64>,

    #[doc = r#"## FDIC Field:: `DEPNIFOR`
    Title: Foreign Deposits - Non-interest Bearing
    Description: Represents any deposit in foreign offices on which the bank does not pay or accrue interest"#]
    #[serde(rename="DEPNIFOR")]
    pub foreign_deposits_non_interest_bearing: Option<i64>,

    #[doc = r#"## FDIC Field:: `DRLNLS`
    Title: Loan & Lease Charge-offs
    Description: Total Charged-Off Loans and Lease Financing Receivables Debited To The Allowance For Loan and Lease Losses
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="DRLNLS")]
    pub loan_lease_charge_offs: Option<i64>,

    #[doc = r#"## FDIC Field:: `EAMINTAN`
    Title: Memo: Amortization of Intangibles
    Description: Goodwill Impairment Losses and Amortization Expense and Impairment Loss For Other Intangible Assets On A Consolidated Basis

Note:
(1) Prior To March 2001, Listed As Memoranda Only and Is Included In All Other Non-interest Expense
(2) Includes Only Amortization Of Goodwill For Tfr Filers"#]
    #[serde(rename="EAMINTAN")]
    pub memo_amortization_of_intangibles: Option<i64>,

    #[doc = r#"## FDIC Field:: `EDEP`
    Title: Int Exp - Total Deposits
    Description: Interest Expense On Total Deposits (Domestic and Foreign) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EDEP")]
    pub int_exp_total_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `EDEPDOM`
    Title: Int Exp - Deposit in Domestic Offices
    Description: Interest Expense On Total Deposits Held In Domestic Offices
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EDEPDOM")]
    pub int_exp_deposit_in_domestic_offices: Option<i64>,

    #[doc = r#"## FDIC Field:: `EDEPFOR`
    Title: Int Exp - Deposits in Foreign Offices
    Description: Deposit Interest Expense-For (1976-Present -- Represents all interests on all liabilities reportable as deposits in foreign offices. -- 1934-1975 -- Interest on foreign office deposits is not available. Reports of income were submitted on a domestic only basis.)"#]
    #[serde(rename="EDEPFOR")]
    pub int_exp_deposits_in_foreign_offices: Option<i64>,

    #[doc = r#"## FDIC Field:: `EEREPP`
    Title: Fed Funds Purchased/Securities Sold
    Description: Represents the gross expenses of all liabilities reportable under this category"#]
    #[serde(rename="EEREPP")]
    pub fed_funds_purchased_securities_sold: Option<i64>,

    #[doc = r#"## FDIC Field:: `EFHLBADV`
    Title: Int Exp Oth - Advances From FHLB
    Description: Interest Expense and The Amortization Of Any Related Yield Adjustments On Fhlbank Advances
Note: Only Reported By Tfr Filers"#]
    #[serde(rename="EFHLBADV")]
    pub int_exp_oth_advances_from_fhlb: Option<i64>,

    #[doc = r#"## FDIC Field:: `EFREPP`
    Title: Int Exp - Fed Funds Purchased/Securities Sold
    Description: Interest Expense On Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis (Prior To March 1997 Was On A Consolidated Basis In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S)
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EFREPP")]
    pub int_exp_fed_funds_purchased_securities_sold: Option<i64>,

    #[doc = r#"## FDIC Field:: `EINTEXP`
    Title: Total Interest Expense
    Description: Total Interest Expense On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December
1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EINTEXP")]
    pub total_interest_expense: Option<i64>,

    #[doc = r#"## FDIC Field:: `EINTEXP2`
    Title: Eintexp2
    Description: Eintexp2"#]
    #[serde(rename="EINTEXP2")]
    pub eintexp2: Option<i64>,

    #[doc = r#"## FDIC Field:: `ELNATR`
    Title: Provision for Loan & Lease Losses
    Description: Provision For Loan & Lease Losses On A Consolidated Basis

Note:
(1) Beginning March 2003, Includes The Provision For Allocated Transfer Risk Related To Loans
(2) From March 1997 To December 2000, Defined As The Provision For Credit Losses & Allocated Transfer Risk Reserve Which Includes The Provision For Off-Balance Sheet Credit Losses For Call Report Filers
(3) Prior To March 1997, Defined As The Provision For Loan and Lease Losses & Allocated Transfer Risk
(4) For Tfr Filers, Consists Of The Provision For Loan and Lease Losses
(5) Reflects Net Provision For Losses On Interest-Bearing Assets For Tfr Filers
(6) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ELNATR")]
    pub provision_for_loan_lease_losses: Option<i64>,

    #[doc = r#"## FDIC Field:: `EOTHNINT`
    Title: All Other Non-interest Expenses
    Description: All Other Non-interest Expense On A Consolidated Basis

Note:
(1) Prior To March 2001, Included The Amortization Of Intangible Assets For Call Reporters
(2) Greater Detail Is Provided In Subsequent Data Fields For All Items In Excess Of 10% Of This Item All Other Non-interest Expense On A Consolidated Basis
(3) Does Not Include Losses On Asset Sales For Tfr Filers Beginning June 1996, Such Gains (Losses) Are Included Net In Non-interest Income
(4) Includes Loss On Sale Of Securities Held For Investments For Tfr Filers Between March 1984 Through December 1986
(5) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EOTHNINT")]
    pub all_other_non_interest_expenses: Option<i64>,

    #[doc = r#"## FDIC Field:: `EPREMAGG`
    Title: Occupancy Expense
    Description: Expenses Of Premises and Fixed Assets (Net Of Rental Income and Excluding Salaries and Employee Benefits and Mortgage Interest) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EPREMAGG")]
    pub occupancy_expense: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQ`
    Title: Total Equity
    Description: Represents the sum of all capital accounts"#]
    #[serde(rename="EQ")]
    pub total_equity: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQCDIV`
    Title: Total Cash Dividends Declared
    Description: Cash Dividends Declared On Common and Preferred Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIV")]
    pub total_cash_dividends_declared: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQCDIVC`
    Title: Cash Dividends Declared (Common)
    Description: Cash Dividends Declared On Common Stock On A Consolidated Basis

Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIVC")]
    pub cash_dividends_declared_common: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQCDIVP`
    Title: Cash Dividends Declared (Preferred)
    Description: Cash Dividends Declared On Preferred Stock On A Consolidated Basis
Note:
(1) 034 Reporters Only File Data On The December Call
(2) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EQCDIVP")]
    pub cash_dividends_declared_preferred: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQCS`
    Title: Common Stock
    Description: Common Stock On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQCS")]
    pub common_stock: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQDIV`
    Title: Total Cash Divident Declared
    Description: The total of cash dividends declared on all preferred and common stock during the calendar year, regardless of when payable"#]
    #[serde(rename="EQDIV")]
    pub total_cash_divident_declared: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQNM`
    Title: Total Equity Capital
    Description: Total Capital (Represents the total of all capital components, including FDIC net worth certificates.)"#]
    #[serde(rename="EQNM")]
    pub total_equity_capital: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQNWCERT`
    Title: FDIC Net Worth Certificates
    Description: Net Worth Certificates Represents The Outstanding Balances Issued To The Fdic In Exchange For Promissory Notes Received From The Fdic On A Consolidated Basis"#]
    #[serde(rename="EQNWCERT")]
    pub fdic_net_worth_certificates: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQOTHCC`
    Title: Other Capital
    Description: Other Capital"#]
    #[serde(rename="EQOTHCC")]
    pub other_capital: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQPP`
    Title: Perpetual Preferred Stock
    Description: Perpetual Preferred Stock and Related Surplus On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQPP")]
    pub perpetual_preferred_stock: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQSUR`
    Title: Surplus
    Description: Surplus (Excludes All Surplus Related To Preferred Stock) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis
"#]
    #[serde(rename="EQSUR")]
    pub surplus: Option<i64>,

    #[doc = r#"## FDIC Field:: `EQUPTOT`
    Title: Undivided Profits
    Description: Undivided Profits, Capital Reserves, Net Unrealized Holding Gains (Losses) On Available-For-Sale Securities and Other Equity Capital Components and/Or
Accumulated Gains (Losses) On Cash Flow Hedges On A Consolidated Basis

Note:
(1) Prior To March 1999 Included Undivided Profits, Capital Reserves and Net Unrealized Gains (Losses) On Available-For-Sale Securities
(2) Prior To March 1994 Included Undivided Profits and Capital Reserves Less Net Unrealized Loss On Marketable Equity Securities
(3) This Item Includes Net Worth Certificates For Bif Thrifts
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="EQUPTOT")]
    pub undivided_profits: Option<i64>,

    #[doc = r#"## FDIC Field:: `ESAL`
    Title: Employee Salaries and Benefits
    Description: Salaries and Employee Benefits On A Consolidated Basis Note: For Banks With Foreign Operations, Data For December 72 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ESAL")]
    pub employee_salaries_and_benefits: Option<i64>,

    #[doc = r#"## FDIC Field:: `ESUBND`
    Title: Int Exp - Subordinated Notes
    Description: Interest Expense On Subordinated Notes and Debentures On A Consolidated Basis
Note:
1. This Item Is Not Reported By Form 51 Filers
2. For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ESUBND")]
    pub int_exp_subordinated_notes: Option<i64>,

    #[doc = r#"## FDIC Field:: `EXTRA`
    Title: Net Extraordinary Items
    Description: Discontinued Operations, Net Of Applicable Income Taxes On A Consolidated Basis

Note:
(1) Prior To March 2016, Defined As Extraordinary Items and and Other Adjustments, Net Of Taxes On A Consolidated Basis
(2) This Item Does Not Include The Tax Effects Related To Securities Gains and Losses and Extraordinary Items From June 1984 Through December 1985 For Bif Thrifts (Refer To Applicable Income Taxes)
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="EXTRA")]
    pub net_extraordinary_items: Option<i64>,

    #[doc = r#"## FDIC Field:: `FD_BIF`
    Title: FDIC Supervised, BIF Insured Institutions
    Description: FDIC Supervised, BIF Insured Institutions"#]
    #[serde(rename="FD_BIF")]
    pub fdic_supervised_bif_insured_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `FD_SAIF`
    Title: FDIC supervised, SAIF Insured Institutions
    Description: FDIC supervised SAIF insured institutions"#]
    #[serde(rename="FD_SAIF")]
    pub fdic_supervised_saif_insured_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `FREPO`
    Title: Federal Funds Sold
    Description: Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis

Note:
(1) Prior To March 1997, Includes Only Federal Funds Sold and Securities Purchased Under Agreements To Resell In Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibf'S
(2) Prior To March 1998, Includes Only Federal Funds Sold For Tfr Filers
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="FREPO")]
    pub federal_funds_sold: Option<i64>,

    #[doc = r#"## FDIC Field:: `FREPP`
    Title: Fed Funds & Repos Purchased
    Description: Federal Funds Purchased and Securities Sold Under Agreements To Repurchase On A Consolidated Basis 
Note:
(1) Prior To March 1998, Includes Only Reverse Repurchase Agreements For Tfr Filers
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="FREPP")]
    pub fed_funds_repos_purchased: Option<i64>,

    #[doc = r#"## FDIC Field:: `ICHBAL`
    Title: Int Inc - Balances Due
    Description: Total Interest Income On Balances Due From Depository Institutions On A Consolidated Basis"#]
    #[serde(rename="ICHBAL")]
    pub int_inc_balances_due: Option<i64>,

    #[doc = r#"## FDIC Field:: `IFEE`
    Title: Fee Income
    Description: Fee Income (Represents service charges on deposit accounts such as maintenance fees, activity charges, administrative charges, overdraft charges and check certification charges; mortgage loans servicing fees plus other fees and charges, including prepayment loan fees, late charges, assumption fees, and amortization of commitment fees.)"#]
    #[serde(rename="IFEE")]
    pub fee_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `IFREPO`
    Title: Int Inc - Fed Funds Sold/Securities Purchased
    Description: Interest Income On Federal Funds Sold and Securities Purchased Under Agreements To Resell On A Consolidated Basis
Note:
(1) Prior To March 1997 Included Only Income From Domestic Offices Of The Bank and Of Its Edge and Agreement Subsidiaries, and In Ibfs On A Consolidated Basis
(2) For Banks With Foreign Operations Data For December 1972 Through 1975 Are Domestic Only"#]
    #[serde(rename="IFREPO")]
    pub int_inc_fed_funds_sold_securities_purchased: Option<i64>,

    #[doc = r#"## FDIC Field:: `IGLSEC`
    Title: Securities Gains and Losses
    Description: Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Debt Securities and Unrealized Holding Gains (Losses) On Equity Securities Not Held For Trading Before Adjustments For Income Taxes On A Consolidated Basis (Also Includes Realized Gains On Equity Securities Until The Institution Adopts Asu 2016-01)
Note:
1. Prior To March 2018, Defined As Realized Gains (Losses) On Held-To-Maturity and Available-For-Sale Securities Before Adjustments For Income Taxes On A Consolidated Basis
2. Beginning In The 2018 Reporting Year, Includes Unrealized Gains (Losses) On Equity Securities For Institutions That Adopted Asu2016-01 and Includes Realized Gains (Losses) On Equity Securities For Institutions That Have Not Yet Adopted Asu2016-01
3. Prior To March 1994 Defined As Gains (Losses) On Securities Not Held In Trading Accounts 
4. From March 1990 Through March 2009, Includes Gains (Losses) On Assets Held For Sale For Tfr Filers
5. Includes Gains (Losses) On Loans Held For Investment From March 1984 Through December 1989 For Tfr Filers
6. Tfr Filers Report Only Gains From March 1984 Through December 1986
7. For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="IGLSEC")]
    pub securities_gains_and_losses: Option<i64>,

    #[doc = r#"## FDIC Field:: `ILNDOM`
    Title: Int Inc - Domestic Office Loans
    Description: Total Interest and Fees On Loans Held In Domestic Offices
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ILNDOM")]
    pub int_inc_domestic_office_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `ILNFOR`
    Title: Int Inc - Foreign Office Loans
    Description: Total Interest and Fees On Loans Held In Foreign Offices, Edge and Agreement Subsidiaries, and Ibf'S"#]
    #[serde(rename="ILNFOR")]
    pub int_inc_foreign_office_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `ILNLS`
    Title: Int Inc - Total Loans & Leases
    Description: Interest and Fees On Loans and Lease Financing Receivables On A Consolidated Basis
Note:
(1) U-Size-Stratum = 0001 Means That Bank Has Total Assets Less Than $25 Million
(2) U-Size-Stratum = 0002 Means That Bank Has Total Assets Equal To Or Greater Than $25 Million
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ILNLS")]
    pub int_inc_total_loans_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `ILNS`
    Title: Int Inc - Loans
    Description: Loans (Represents all interest, fees and similar charges levied against or associated with all assets reportable as loans. Includes interest, yield related fees, commitment fees, service charges on loans and discount accretion. (One savings bank with an office in Canada has been reporting on the Domestic & Foreign Consolidated Call Report form (FFIEC 031). It does not, however, indicate any income or expenses related to foreign operations.))"#]
    #[serde(rename="ILNS")]
    pub int_inc_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `ILS`
    Title: Int Inc - Leases
    Description: Total Interest Income From Lease Financing Receivables On A
Consolidated Basis"#]
    #[serde(rename="ILS")]
    pub int_inc_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `INTAN`
    Title: Intangible Assets
    Description: Intangible Assets On A Consolidated Basis"#]
    #[serde(rename="INTAN")]
    pub intangible_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `INTBAST`
    Title: Total Interest Earning Assets
    Description: Total Interest Earning Assets (Derived See Si-19) - Sc"#]
    #[serde(rename="INTBAST")]
    pub total_interest_earning_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `INTBLIB`
    Title: Total Interest Bearing Liabilities
    Description: Total Interest Bearing Liabilities (Derived See Si-19) - Sc"#]
    #[serde(rename="INTBLIB")]
    pub total_interest_bearing_liabilities: Option<i64>,

    #[doc = r#"## FDIC Field:: `INTINC`
    Title: Total Interest Income
    Description: Total Interest Income On A Consolidated Basis
Note: For Banks With Foreign Operations Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="INTINC")]
    pub total_interest_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `INTINC2`
    Title: INTINC2
    Description: INTINC2"#]
    #[serde(rename="INTINC2")]
    pub intinc2: Option<i64>,

    #[doc = r#"## FDIC Field:: `IRAKEOGH`
    Title: Memo: IRA's and Keogh Plan-Deposits
    Description: Individual Retirement Accounts (Ira'S) and Keogh Plan
Accounts Held In Domestic Offices
Note: Listed As Memoranda Only"#]
    #[serde(rename="IRAKEOGH")]
    pub memo_ira_s_and_keogh_plan_deposits: Option<i64>,

    #[doc = r#"## FDIC Field:: `ISC`
    Title: Int Inc - Investment Securities
    Description: Total Interest and Dividend Income On: U.S. Treasury Securities, U.S. Government Agency and Corporation Obligations, Securities Issued By States and Political
Subdivision In The U.S., Other Domestic Debt Securities, Foreign Debt Securities, and Equity Securities (Including Investments In Mutual Funds) On A Consolidated Basis
Note:
(1) This Item Includes Interest Income On Deposits For Tfr Filers
(2) Includes Interest Income On Assets Held In Trading Accounts For Tfr Filers For Two Distinct Periods: (A) March 1984 Through December 1989 and (B) June 1996
And Following Quarters.
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="ISC")]
    pub int_inc_investment_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `ISERCHG`
    Title: Service Charges on Deposit Accounts
    Description: Represents service charges on deposit accounts in domestic offices such as maintenance fees, activity charges, administrative charges, overdraft charges, and check certification charges"#]
    #[serde(rename="ISERCHG")]
    pub service_charges_on_deposit_accounts: Option<i64>,

    #[doc = r#"## FDIC Field:: `ITAX`
    Title: Applicable Income Taxes
    Description: Represents Federal, state and local taxes on income. It does not include taxes relating to securities transactions or extraordinary items"#]
    #[serde(rename="ITAX")]
    pub applicable_income_taxes: Option<i64>,

    #[doc = r#"## FDIC Field:: `ITAXR`
    Title: Pre-Tax Net Operating Income
    Description: Pre-Tax Net Operating Income (Represents Net Interest Income plus Total Non-interest Income less Total Non-interest Expense and the Provision for Loan & Lease Losses.)"#]
    #[serde(rename="ITAXR")]
    pub pre_tax_net_operating_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `ITRADE`
    Title: Int Inc - Trading Account Assets
    Description: Interest Income From Assets Held In Trading Accounts On A Consolidated Basis 
Note:
Beginning March 2017, Reported As An Individual Income Category For Form 031 Filers Only and Is Included As A Component Of Other Interest Income For All Other Report Forms"#]
    #[serde(rename="ITRADE")]
    pub int_inc_trading_account_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `LIAB`
    Title: Total Liabilities
    Description: Total Liabilities Including Subordinated Notes and Debentures and Limited Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: Prior To March 2009, This Item Included Noncontrolling (Minority) Interests In Consolidated Subsidiaries For Call Report and Tfr Filers"#]
    #[serde(rename="LIAB")]
    pub total_liabilities: Option<i64>,

    #[doc = r#"## FDIC Field:: `LIABEQ`
    Title: Total Liabilities and Equity Capital
    Description: Total Liabilities, Limited-Life Preferred Stock, and Equity Capital On A Consolidated Basis Note: For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="LIABEQ")]
    pub total_liabilities_and_equity_capital: Option<i64>,

    #[doc = r#"## FDIC Field:: `liqasstd`
    Title: Assisted Payouts
    Description: Represents all assisted payouts of FDIC-insured savings institutions that are not in RTC conservatorship."#]
    #[serde(rename="liqasstd")]
    pub assisted_payouts: Option<i64>,

    #[doc = r#"## FDIC Field:: `liqunass`
    Title: Voluntary Liquidations
    Description: Represents all instances where the owners of a thrift voluntarily surrender their charter with all liabilities including deposits paid down and all assets sold."#]
    #[serde(rename="liqunass")]
    pub voluntary_liquidations: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNAG`
    Title: Agricultural Loans
    Description: Loans To Finance Agricultural Production and Other Loans To Farmers On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods Form December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNAG")]
    pub agricultural_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNALLOTH`
    Title: All Other Loans to Individuals
    Description: All Other Loans (1969-Present -- Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. -- 1934-1968 -- Does not include mortgage indebtedness which is netted against bank premises.)"#]
    #[serde(rename="LNALLOTH")]
    pub all_other_loans_to_individuals: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNATRES`
    Title: Allowance for Losses Loans and Leases
    Description: Allowance For Loan and Lease Financing Receivable Losses and Allocated Transfer Risk On A Consolidated Basis
Note:
(1) From March 2001 To Dec 2002 Allocated Transfer Riskis Netted From Loans & Not Included As Part Of The Reserve
(2) Additional Detail Can Be Found On Schedule Ri-B
(3) For Tfr Filers Between March 1984 Through December 1989 Includes Allowance For Mortgage Pool Securities
(4) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="LNATRES")]
    pub allowance_for_losses_loans_and_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNAUTO`
    Title: Memo: Loans to Individuals - Auto
    Description: Represents installment loans to purchase private passenger automobiles, both direct loans and purchased paper"#]
    #[serde(rename="LNAUTO")]
    pub memo_loans_to_individuals_auto: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNCI`
    Title: Commercial and Industrial Loans
    Description: Commercial and Industrial Loans On A Consolidated Basis Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCI")]
    pub commercial_and_industrial_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNCON`
    Title: Total Loans to Individuals
    Description: Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) On A Consolidated Basis
Note:
(1) For Tfr Filers Includes Revolving Loans Secured By 1-4 Family Dwelling Units From March 1984 Through March 1988
(1) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCON")]
    pub total_loans_to_individuals: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNCONOT1`
    Title: Loans to Individuals - Home Improvement
    Description: Installment Loans To Individuals To Repair and Modernize
Residential Property Held In Domestic Offices"#]
    #[serde(rename="LNCONOT1")]
    pub loans_to_individuals_home_improvement: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNCONOTH`
    Title: Loans to Individuals - All Others
    Description: Represents all other loans to individuals for household, family and other personal expenditures. It includes auto loans, both direct and indirect, mobile homes (unless secured by a real estate mortgage), education loans, other installment loans both secured by personal property or unsecured, and single payment loans (time or demand, secured or unsecured)"#]
    #[serde(rename="LNCONOTH")]
    pub loans_to_individuals_all_others: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNCRCD`
    Title: Loans to Individuals - Credit Card Plans
    Description: Credit Cards Related Plans On A Consolidated Basis

Note:
(1)Prior To March 2001 Includes Credit Cards Related Plans-Loans To Individuals For Household, Family, and Other Personal Expenditures (Consumer Loans) Includes Check Credit and Other Revolving Credit Plans
(2) For Tfr Filers Between March 1984 Through March 1988 This Figure Includes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(3) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNCRCD")]
    pub loans_to_individuals_credit_card_plans: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNDEP`
    Title: Loans to Deposit Institutions
    Description: Loans To Depository Institutions On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only
Note:(1) Beginning March 2001 Includes Acceptances Of Other Banks
(2) Beginning March 2001, Includes Acceptances Of Other Banks For Ibas"#]
    #[serde(rename="LNDEP")]
    pub loans_to_deposit_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNLS`
    Title: Gross Loans and Leases
    Description: Represents the sum of all components of loans"#]
    #[serde(rename="LNLS")]
    pub gross_loans_and_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNLSGR`
    Title: Total Loans and Leases
    Description: Loans and Lease Financing Receivables, Net Of Unearned Income, On A Consolidated Basis
Note:
(1) Additional Detail Can Be Found On Schedule Rc-C
(2) For Tfr Filers This Item Is Net Of Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through December 1989
(3) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="LNLSGR")]
    pub total_loans_and_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNLSNET`
    Title: Net Loans and Leases
    Description: Loans and Lease Financing Receivables, Net Of Unearned Income, Allowance, and Reserve On A Consolidated Basis
Note:
(1) For Tfr Filers This Item Is Net Of Valuation Allowances and Unamortized Yield Adjustments For Mortgage Pool Securities From March 1984 Through
(2) For Banks With Foreign Operations, Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="LNLSNET")]
    pub net_loans_and_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNMOBILE`
    Title: Memo: Loans to Individuals - Mobile Homes
    Description: Represents loans to individuals to purchase mobile homes. (If the bank's security interest in the loan was represented by a mortgage or deed of trust, the loan should be included in real estate loans)"#]
    #[serde(rename="LNMOBILE")]
    pub memo_loans_to_individuals_mobile_homes: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNMUNI`
    Title: Loans to States and Politicial Sub-divisions
    Description: Obligations (Other Than Securities and Leases) Of States and Political Subdivisions In The U.S. (Including Nonrated Industrial Development Obligations) On A Consolidated Basis"#]
    #[serde(rename="LNMUNI")]
    pub loans_to_states_and_politicial_sub_divisions: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRE`
    Title: Total Real Estate Loans
    Description: Loans Secured By Real Estate On A Consolidated Basis
Note:
(1) For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower (T-Sc340)
(2) For Banks With Foreign Operations, Data For All Periods From December 1972 Through September 1978 Are Domestic Only"#]
    #[serde(rename="LNRE")]
    pub total_real_estate_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNREAG`
    Title: R/E Loan - Farmland
    Description: Represents loans secured by farmland, including improvements, and other land known to be used or usable for agricultural purposes, as evidenced by mortgages or other liens. It includes loans secured by farmland that are guaranteed by the Farmers Home Administration (FHA) or by the Small Business Administration"#]
    #[serde(rename="LNREAG")]
    pub r_e_loan_farmland: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRECONS`
    Title: R/E Loan - Construction & Land Develop
    Description: Construction and Land Development Loans Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers Portions Of Lnrecons Were Included In Other Real Estate Loan Categories Prior To March 30, 1986"#]
    #[serde(rename="LNRECONS")]
    pub r_e_loan_construction_land_develop: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNREDOM`
    Title: Total R/E Loans in Domestic Offices
    Description: Represents the total of all loans secured by real estate in domestic offices (U.S. and other areas)"#]
    #[serde(rename="LNREDOM")]
    pub total_r_e_loans_in_domestic_offices: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNREFOR`
    Title: Total R/E Loans in Foreign Offices
    Description: Represents all loans secured by real estate in foreign offices"#]
    #[serde(rename="LNREFOR")]
    pub total_r_e_loans_in_foreign_offices: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRELOC`
    Title: Memo: Home Equity Loans
    Description: Revolving, Open-End Loans Secured By 1-4 Family Residential Properties and Extended Under Lines Of Credit Held In Domestic Offices"#]
    #[serde(rename="LNRELOC")]
    pub memo_home_equity_loans: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNREMULT`
    Title: R/E Loans -  Multifamily
    Description: Multifamily (5 Or More) Residential Properties Secured By Real Estate Held In Domestic Offices"#]
    #[serde(rename="LNREMULT")]
    pub r_e_loans_multifamily: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRENRES`
    Title: R/E Loan -  Non-farm/Non-residential Prop
    Description: Nonfarm Nonresidential Properties Secured By Real Estate Held In Domestic Offices
Note: For Tfr Filers This Figure Includes Mortgages On Properties That Are Used For Farming"#]
    #[serde(rename="LNRENRES")]
    pub r_e_loan_non_farm_non_residential_prop: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRERES`
    Title: R/E Loan - 1-4 Family
    Description: Total Loans Secured By 1-4 Family Residential Properties Held In Domestic Offices
Note: For Tfr Filers Between March 1984 Through March 1988 This Figure Excludes Home Equity Loans Based On The Creditworthiness Of The Borrower"#]
    #[serde(rename="LNRERES")]
    pub r_e_loan_1_4_family: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNRESRE`
    Title: Memo: Contra Account
    Description: Allowance For Loan Losses On Real Estate Loans On A Consolidated Basis
Note: For Tfr Filers Includes Allowance For Mortgage Pool Securities Between March 1984 Through December 1989"#]
    #[serde(rename="LNRESRE")]
    pub memo_contra_account: Option<i64>,

    #[doc = r#"## FDIC Field:: `LNSP`
    Title: Memo: Loans to Individuals - Single Payment
    Description: All loans both time or demand, secured or unsecured, to individuals for personal, family or other household expenditures"#]
    #[serde(rename="LNSP")]
    pub memo_loans_to_individuals_single_payment: Option<i64>,

    #[doc = r#"## FDIC Field:: `LS`
    Title: Leases
    Description: Lease Financing Receivables (Net Of Unearned Income) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March & September Call Dates Are Domestic Only Through December 1975"#]
    #[serde(rename="LS")]
    pub leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `MERGERS`
    Title: Failures: Assisted Merger
    Description: Mergers, consolidations or absorptions entered into as a result of supervisory actions. The transaction may or may not have required FDIC assistance."#]
    #[serde(rename="MERGERS")]
    pub failures_assisted_merger: Option<i64>,

    #[doc = r#"## FDIC Field:: `MISSADJ`
    Title: Other Misc. Adjustments
    Description: Represents any FDIC-insured savings institution that did not file a financial report during the year in which the charter was added or deleted."#]
    #[serde(rename="MISSADJ")]
    pub other_misc_adjustments: Option<i64>,

    #[doc = r#"## FDIC Field:: `MTGLS`
    Title: Mortgage and Other Borrowings
    Description: Represents mortgage indebtedness and liabilities under capitalized leases"#]
    #[serde(rename="MTGLS")]
    pub mortgage_and_other_borrowings: Option<i64>,

    #[doc = r#"## FDIC Field:: `NALNLS`
    Title: Non-accrual Loans & Leases
    Description: Total Nonaccrual Loans and Lease Financing Receivables On A Consolidated Basis"#]
    #[serde(rename="NALNLS")]
    pub non_accrual_loans_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `NCHGREC`
    Title: Net Loans and Leases Charge-offs
    Description: Net Loans and Leases Charge Offs (-- 1984-1989 -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries. Not collected by TFR filers. -- 1990-Present -- Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in paraentheses indicates net recoveries.)"#]
    #[serde(rename="NCHGREC")]
    pub net_loans_and_leases_charge_offs: Option<i64>,

    #[doc = r#"## FDIC Field:: `NCLNLS`
    Title: Total Non-current Loans & Leases
    Description: Total Loans and Lease Financing Receivables 90 Days Or More Past Due and Nonaccrual On A Consolidated Basis
Note: Includes Delinquent Loans (60 Or More Days Overdue) and Past Due Loans (One Or More Payments Missed) For Tfr Filers Prior To March 1990"#]
    #[serde(rename="NCLNLS")]
    pub total_non_current_loans_leases: Option<i64>,

    #[doc = r#"## FDIC Field:: `NETIMIN`
    Title: Net Income Attributable to Noncontrolling Interests
    Description: Net income (loss) attributable to noncontrolling (minority) interests on a consolidated basis."#]
    #[serde(rename="NETIMIN")]
    pub net_income_attributable_to_noncontrolling_interests: Option<i64>,

    #[doc = r#"## FDIC Field:: `NETINC`
    Title: Net Income
    Description: Net Income Attributable To The Bank On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NETINC")]
    pub net_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `newcount`
    Title: New Charters
    Description: Institutions newly chartered by federal or state banking authorities including authorities in the U. S. Territories or possessions."#]
    #[serde(rename="newcount")]
    pub new_charters: Option<i64>,

    #[doc = r#"## FDIC Field:: `New_Char`
    Title: New Charters
    Description: Institutions newly licensed or chartered by the Office of the Comptroller of the Currency (national banks) or by state banking authorities, including banking authorities in the U. S. territories or possessions. Includes de novo institutions as well as charters issued to take over a failing institution."#]
    #[serde(rename="New_Char")]
    pub new_charters_new_char: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW6_1`
    Title: Int Exp - Borrowed Money
    Description: Represents interest expense related to demand notes issued to the U. S. Treasury, mortgage indebtedness, obligations under capitalized leases, and other borrowed money"#]
    #[serde(rename="NEW6_1")]
    pub int_exp_borrowed_money: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW9_1`
    Title: All Other Assets
    Description: Represents all other assets not included in previously mentioned captions. Includes, for the most part, customers' liabilities on acceptances outstanding, income earned not collected as well as any other asset not included above"#]
    #[serde(rename="NEW9_1")]
    pub all_other_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW10_1`
    Title: Corporate Bonds and Other Securities
    Description: Represents all securities, bonds, notes and debentures of domestic and foreign corporations. Also includes privately issued or guaranteed mortgage backed securities and certain detached U.S. Government security coupons held as a result of either their purchase or the bank's stripping them (CATS, TIGRs, COUGARs, LIONs and ETRs)."#]
    #[serde(rename="NEW10_1")]
    pub corporate_bonds_and_other_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW10_2`
    Title: Trading Account Securities
    Description: Securities within the scope of ASC Topic 320, Investments – Debt Securities, that a bank has elected to report at fair value under a fair value option with changes in fair value reported in current earnings should be classified as trading securities. (https://www.fdic.gov/regulations/resources/call/crinst/2018-06/031-041-618rc-d-063018.pdf)"#]
    #[serde(rename="NEW10_2")]
    pub trading_account_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW10_3`
    Title: Memo: Valuation Reserves
    Description: For all years except 1969-1973, investment securities are reflected net of general valuation reserves. Specific reserves are deducted from each security so reserved"#]
    #[serde(rename="NEW10_3")]
    pub memo_valuation_reserves: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW11_1`
    Title: All Other Loans
    Description: Represents unplanned overdrafts and loans to: brokers and dealers in securities, any borrower for the purpose of purchasing and carrying securities, nonprofit institutions and organizations, individuals for investment purposes, real estate investment trusts, mortgage companies holding companies of depository institutions, insurance companies, finance companies, factors and other financial intermediaries, federally sponsored lending agencies, investment banks, the bank's own trust department, Small Business Investment Companies, foreign governments and official institutions, and any other loan not included in one of the above categories"#]
    #[serde(rename="NEW11_1")]
    pub all_other_loans_new11_1: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW14_1`
    Title: Borrowed Funds
    Description: Represents Federal funds purchased, securities sold under agreements to repurchase, demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money"#]
    #[serde(rename="NEW14_1")]
    pub borrowed_funds_new14_1: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW14_2`
    Title: Other Liabilities
    Description: Includes all liabilities not included above and limited life preferred stock"#]
    #[serde(rename="NEW14_2")]
    pub other_liabilities: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW14_3`
    Title: Total Liabilities
    Description: Represents the total of all components of liabilities"#]
    #[serde(rename="NEW14_3")]
    pub total_liabilities_new14_3: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW14_4`
    Title: Undivided Profits
    Description: Represents undivided profits and related accounts"#]
    #[serde(rename="NEW14_4")]
    pub undivided_profits_new14_4: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_1`
    Title: Deposits - Individuals, Partnerships and Corporations
    Description: Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#]
    #[serde(rename="NEW15_1")]
    pub deposits_individuals_partnerships_and_corporations: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_2`
    Title: Deposits - U.S. Government
    Description: Represents all deposits of individuals, partnerships and corporations in domestic and foreign offices"#]
    #[serde(rename="NEW15_2")]
    pub deposits_u_s_government: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_3`
    Title: Deposits - States and Political Subdivisions
    Description: Represents all deposits of states, counties and municipalities in domestic offices. Such deposits, if any, in foreign offices are not separately reported"#]
    #[serde(rename="NEW15_3")]
    pub deposits_states_and_political_subdivisions: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_4`
    Title: Deposits - All Other
    Description: Represents all other deposits. Includes deposits of financial institutions, both domestic and foreign, deposits of foreign governments and official institutions and certified and official checks. Also includes deposits in foreign offices other than those of individuals, partnerships and corporations"#]
    #[serde(rename="NEW15_4")]
    pub deposits_all_other: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_5`
    Title: Deposits - Domestic Savings
    Description: Represents all savings deposits in domestic offices"#]
    #[serde(rename="NEW15_5")]
    pub deposits_domestic_savings: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW15_7`
    Title: Total Domestic Deposits
    Description: Total Domestic Deposits"#]
    #[serde(rename="NEW15_7")]
    pub total_domestic_deposits_new15_7: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW16_1`
    Title: Demand Notes and Other Liabilities
    Description: Represents demand notes issued to the U.S. Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers"#]
    #[serde(rename="NEW16_1")]
    pub demand_notes_and_other_liabilities: Option<i64>,

    #[doc = r#"## FDIC Field:: `NEW16_2`
    Title: Interest Bearing Deposits
    Description: Represents any deposit in domestic and foreign offices on which the banks pays or accrues interest"#]
    #[serde(rename="NEW16_2")]
    pub interest_bearing_deposits_new16_2: Option<i64>,

    #[doc = r#"## FDIC Field:: `NIM`
    Title: Net Interest Income
    Description: Net Interest Income (Total Interest Income Minus Total Interest Expense) On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NIM")]
    pub net_interest_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `NONII`
    Title: Total Non-interest Income
    Description: Total Non-interest Income On A Consolidated Basis
Note:
(1) From March 1990 Through March 2009, Excludes Gains (Losses) On Assets Held For Sale For Tfr Filers, See Tfr Instructions For So430
(2) Excludes Gains On The Sale Of Loans Held For Investments From March 1984 Through December 1989 For Tfr Filers
(3) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NONII")]
    pub total_non_interest_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `NONIX`
    Title: Total Non-interest Expense
    Description: Total Non-interest Expense On A Consolidated Basis
Note:
(1) Excludes Losses On Asset Sales For Tfr Filers Beginning June 1996
(2) Includes Loss On Sale Of Mortgage Pool and Other Securities Held For Investment For Tfr Filers From March 1984 Through December 1986
(3) Excludes Losses On Loans Held For Investment For Tfr Filers From March 1987 Through December 1989
(4) For Banks With Foreign Operations, Data For December 1972 Through December 1975 Are Domestic Only"#]
    #[serde(rename="NONIX")]
    pub total_non_interest_expense: Option<i64>,

    #[doc = r#"## FDIC Field:: `NTLNLS`
    Title: Net Loan & Lease Charge-offs
    Description: Represents Loan and Lease Charge-offs less Loan and Lease Recoveries. An amount enclosed in parentheses indicates net recoveries. Not collected by TFR filers"#]
    #[serde(rename="NTLNLS")]
    pub net_loan_lease_charge_offs: Option<i64>,

    #[doc = r#"## FDIC Field:: `NTR`
    Title: Memo: Domestic Deposits Non-Transaction
    Description: Represents deposits that are not included in the definition of transaction accounts above or that do not satisfy the criteria necessary to be defined as a transaction account. MMDA's are specifically defined as nontransaction accounts"#]
    #[serde(rename="NTR")]
    pub memo_domestic_deposits_non_transaction: Option<i64>,

    #[doc = r#"## FDIC Field:: `NTRTIME`
    Title: Deposits - Domestic Time
    Description: Represents all time certificates of deposit, time open accounts and similar deposits in domestic offices"#]
    #[serde(rename="NTRTIME")]
    pub deposits_domestic_time: Option<i64>,

    #[doc = r#"## FDIC Field:: `NTRTMLG`
    Title: Memo: Time Deposits (Over $100K)
    Description: Time Deposits Over $100,000 Or More Held In Domestic Offices
Note:
(1) Listed As Memoranda Only and Is Included In Total Nontransaction Accounts
(2) Prior To March 2007, Includes All Deposits (Not Just Time) Greater Than $100,000 For Tfr Filers. Except For December 2006, Includes All Nonretirement Deposits Over
$100,000 and All Retirement Deposits Over $250,000 For Tfr Filers
(3) Includes Time Deposits Of $100,000 Or More"#]
    #[serde(rename="NTRTMLG")]
    pub memo_time_deposits_over_100k: Option<i64>,

    #[doc = r#"## FDIC Field:: `NUMEMP`
    Title: Number of Full Time Employees
    Description: Number Of Full Time-Equivalent Employees On Payroll At The End Of The Current Period
Note:
(1) Listed As Memoranda Only
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="NUMEMP")]
    pub number_of_full_time_employees: Option<i64>,

    #[doc = r#"## FDIC Field:: `OEA`
    Title: Other Earning Assests
    Description: Other Earning Assests (-- 1984-1989 -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Items not separately reported by TFR filers. They are included in Secruties. -- 1990-Present -- Represents Federal funds sold and securities purchased under agreements to resell (repurchase agreements). Includes only Federal funds sold for TFR filers. Repurchase agreements are included in Securities.)"#]
    #[serde(rename="OEA")]
    pub other_earning_assests: Option<i64>,

    #[doc = r#"## FDIC Field:: `OFFICES`
    Title: Offices
    Description: Offices include: Multiple service offices, Military facilities, Drive-in facilities, Loan production offices, Consumer credit offices, Seasonal offices, Administrative offices, Messenger service offices, Supermarket banking offices, and Other offices."#]
    #[serde(rename="OFFICES")]
    pub offices: Option<i64>,

    #[doc = r#"## FDIC Field:: `OINTBOR`
    Title: Demand Notes and Other Borrowings
    Description: Demand Notes and Other Borrowings (Represents demand notes issued to US Treasury (Treasury tax & loan account), and all other borrowings. Includes mortgage indebtedness and liabilities under capitalized leases for Call report filers. Includes FSLIC net worth certificates for TFR filers.)"#]
    #[serde(rename="OINTBOR")]
    pub demand_notes_and_other_borrowings: Option<i64>,

    #[doc = r#"## FDIC Field:: `OINTEXP`
    Title: Total Other Interest Expenses
    Description: Total Other Interest Expenses (Federal Funds Purchased and Securities Sold -- Represents the gross expense of all liabilities reportable under this category. This item is not reported separately by TFR filers. It is included in Borrowed Money)."#]
    #[serde(rename="OINTEXP")]
    pub total_other_interest_expenses: Option<i64>,

    #[doc = r#"## FDIC Field:: `OINTINC`
    Title: Int Inc - Total Other
    Description: Total Other Interest Income (Represents the total of all Other Interest Income components)."#]
    #[serde(rename="OINTINC")]
    pub int_inc_total_other: Option<i64>,

    #[doc = r#"## FDIC Field:: `OONONII`
    Title: Other Non-interest Income
    Description: Other Non Interest Income (1984-1989 -- Same as Total Other Interest Income except gains on the sale of loans held for investment are excluded for TFR filers. -- 1990- Present -- Represents income derived from the sale of assets held for sale; office building operations; real estate held for investment; REO operations; LOCOM adjustments made to assets held for sale; net income (loss) from investements in service corporations/subsidiaries (other than operating or finance subsidiaires); leasing operations; realized and unrealized gains (losses) on trading assets; gains on the sale of REO real estate held for investment, and loans held for investment; and the amoritization of deferred gains (losses) on asset hedges.)"#]
    #[serde(rename="OONONII")]
    pub other_non_interest_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `ORE`
    Title: Other Real Estate Owned
    Description: Other Real Estate Owned On A Consolidated Basis
Note:
(1) Prior To June 2009, Includes Direct and Indirect Investments In Real Estate
(2) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Was Reported On A Domestic Basis"#]
    #[serde(rename="ORE")]
    pub other_real_estate_owned: Option<i64>,

    #[doc = r#"## FDIC Field:: `ORET`
    Title: Other Real Estate
    Description: Other Real Estate (Represents other real estate owned net of reserves for losses). Not available for 1997. For 1986 through 1988 ORET = ORE + INVSUORE; for all other years ORET = ORE"#]
    #[serde(rename="ORET")]
    pub other_real_estate: Option<i64>,

    #[doc = r#"## FDIC Field:: `OT_BIF`
    Title: Non FDIC Supervised BIF Insured Institutions
    Description: Non FDIC supervised BIF insured institutions"#]
    #[serde(rename="OT_BIF")]
    pub non_fdic_supervised_bif_insured_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `OT_SAIF`
    Title: Non FDIC Supervised SAIF Insured Institutions
    Description: Non FDIC supervised SAIF insured institutions"#]
    #[serde(rename="OT_SAIF")]
    pub non_fdic_supervised_saif_insured_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHASST`
    Title: All Other Assets
    Description: All Other Assets (Same as Other Real Estate except that investment in service corporations/subsidiaries is reported gross of valuation allowances by TFR filers, and assets held in trading accounts are included in Securities for TFR filers. -- 1990-Present -- Represents all associations assets not previously mentioned. Includes all non real estate repossessed property, investment in service corporations/subsidiaries, property leased to others, income earned but not yet collected, assets held in the trading accounts, and miscellaneous assets) For 2009- present OTHASST = SUM (INVSUB + INVSUORE + CUSLI + OA)"#]
    #[serde(rename="OTHASST")]
    pub all_other_assets_othasst: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHBFHLB`
    Title: Advances from FHLB
    Description: Other Liabilities From The Fhlb
Note:Prior To March 2001 Only Reported On Tfrs"#]
    #[serde(rename="OTHBFHLB")]
    pub advances_from_fhlb: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHBORR`
    Title: Int Exp Oth - Borrowed Money
    Description: Borrowed Money (Represents interest expense related to demand notes issued the US Treasury, mortage indebtedness, obligations under capitalized leases and on other borrowed money."#]
    #[serde(rename="OTHBORR")]
    pub int_exp_oth_borrowed_money: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHEQ`
    Title: Other Equity
    Description: Represents all equity securities not held for trading: investment in mutual funds, common stock of FNMA, Student Loan Marketing Association, Federal Home Loan Mortgage Corporation, Federal Reserve Bank stock, Federal Home Loan Bank stock, minority interests not meeting the definition of associated companies, "restricted" stock, and other equity securities in both domestic and foreign corporations
"#]
    #[serde(rename="OTHEQ")]
    pub other_equity: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHER`
    Title: Other
    Description: Withdrawals from FDIC insurance, voluntary liquidations, or conversions to institutions that are not considered commercial banks. Also includes relocation of banks from one state to another."#]
    #[serde(rename="OTHER")]
    pub other: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHLIAB`
    Title: Other Liabilities
    Description: Other Liabilities (Includes all liabilities not included above and limited life preferred stock. 2001- present -- Includes OTHER LIAB & MINOR IN SUBS)."#]
    #[serde(rename="OTHLIAB")]
    pub other_liabilities_othliab: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTHNBORR`
    Title: Borrowed Funds
    Description: Borrowed Funds (Includes federal funds purchased, securities sold under agreements to repurchase (reverse repurchase agreements), demand notes issued to the US Treasury, mortgage indebtedness, liabilities under capitalized leases and all other liabilities for borrowed money. Includes only reverse purchase agreements (securities sold under agreements to repurchase) and FSLIC net worth certificates for TFR filers)"#]
    #[serde(rename="OTHNBORR")]
    pub borrowed_funds_othnborr: Option<i64>,

    #[doc = r#"## FDIC Field:: `OTLNCNTA`
    Title: Less: Other Contra Accounts
    Description: Other Contracts (Represents amount reported by savings institutions that file on the Thrift Financial Report. Contra accounts include accrued interest receivable, unamortized yield adjustments and valuation allowances. Negative amounts reflect unamortized premiums and deferred direct costs exceeding unamortized discounts and deferred loan fees)."#]
    #[serde(rename="OTLNCNTA")]
    pub less_other_contra_accounts: Option<i64>,

    #[doc = r#"## FDIC Field:: `PAID_OFF`
    Title: Failures: Paid Off
    Description: Institutions that were declared insolvent, the insured deposits of which were paid by the FDIC."#]
    #[serde(rename="PAID_OFF")]
    pub failures_paid_off: Option<i64>,

    #[doc = r#"## FDIC Field:: `P3LNLS`
    Title: Loans & Leases P/D 30-89 Days
    Description: Total Loans and Lease Financing Receivables Past Due 30 Through 89 Days and Still Accruing Interest On A Consolidated Basis
Note:
(1) Prior To March 2001,This Information On An Institution Level Is Considered Confidential By The Ffiec"#]
    #[serde(rename="P3LNLS")]
    pub loans_leases_p_d_30_89_days: Option<i64>,

    #[doc = r#"## FDIC Field:: `P9LNLS`
    Title: Loans & Leases P/D 90+ Days
    Description: Total Loans and Lease Financing Receivables Past Due 90 Or More Days and Still Accruing Interest On A Consolidated Basis"#]
    #[serde(rename="P9LNLS")]
    pub loans_leases_p_d_90_days: Option<i64>,

    #[doc = r#"## FDIC Field:: `PTXNOINC`
    Title: Pre-Tax Net Operating Income
    Description: Pre-Tax Net Operating Income"#]
    #[serde(rename="PTXNOINC")]
    pub pre_tax_net_operating_income_ptxnoinc: Option<i64>,

    #[doc = r#"## FDIC Field:: `REL_CO`
    Title: Conversions
    Description: Conversions of existing institutions of any type that meet the definition of commercial banks (see Definition of Total Commercial Banks and have applied for and received FDIC insurance. Also includes bank relocations from one state to another."#]
    #[serde(rename="REL_CO")]
    pub conversions: Option<i64>,

    #[doc = r#"## FDIC Field:: `SAVINGS`
    Title: Total Savings Institutions (Total Insured)
    Description: Total Insured Savings Institutions including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)."#]
    #[serde(rename="SAVINGS")]
    pub total_savings_institutions_total_insured: Option<i64>,

    #[doc = r#"## FDIC Field:: `SC`
    Title: Total Investment Securities (Book Value)
    Description: Total Securities: The Sum Of Held-To-Maturity Securities At Amortized Cost, Available-For-Sale Securities At Fair Value and Equity Securities With Readily Determinable Fair Values Not Held For Trading On A Consolidated Basis
Note:
1. Prior To March 2018, Defined As Total Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities (Excludes Assets Held In Trading Accounts) On A Consolidated Basis
2. Beginning In 2018, Includes Equity Securities For Institutions That Have Adopted Asu2016-01 and Those Institutions That Have Not Yet Adopted This Accounting
Standard
3. Prior To March 1994 Item Defined As Book Value
4. Additional Detail Can Be Found On Schedule Rc-B
5. For Tfr Filers Between March 1984 Through December 1989 Includes Interest-Earning Deposits In Fhlbs, Other Interest-Earning Deposits, Federal Funds Sold and Assets Held In Trading Accounts
6. For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SC")]
    pub total_investment_securities_book_value: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCAGE`
    Title: U.S. Agencies and Corporation Securities
    Description: Total U.S. Government Agency and Corporation Obligations On A Consolidated Basis
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
    pub u_s_agencies_and_corporation_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCEQ`
    Title: Equity Securities
    Description: Total Equity Securities Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning March 2018 Does Not Include Equity Securities For Institutions That Have Adopted Asu 2016-01 See Sceqfv
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers"#]
    #[serde(rename="SCEQ")]
    pub equity_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCMTGBK`
    Title: Memo: Mortgage Backed Securities
    Description: Mortgage Backed Securities On A Consolidated Basis
Includes:
(1) U.S. Government Agency and Corporation Obligations Issued Or Guaranteed Certificates Of Participation In Pools Of Residential Mortgages,
(2) U.S. Government Agency and Corporation Obligations Collateralized Mortgage Obligations Issued By Fnma and Fhlmc (Including Remics)
(3) Other Domestic Debt Securities - Private (I.E., Non-Government-Issued-Or-Guaranteed) Certificates Of Participations In Pools Of Residential Mortgages, and
(4) Other Domestic Debt Securities - Privately-Issued Collateralized Mortgage Obligations (Including Remics)"#]
    #[serde(rename="SCMTGBK")]
    pub memo_mortgage_backed_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCMUNI`
    Title: States and Political Subdivisions Securities
    Description: Total Securities Issued By States and Political Subdivisions Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Prior To March 1994 Item Was Defined As Book Value
(2) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(3) For Banks With Foreign Opeations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCMUNI")]
    pub states_and_political_subdivisions_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCMV`
    Title: Market Values
    Description: Represents the market (fair) value of all investment securities"#]
    #[serde(rename="SCMV")]
    pub market_values: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCRES`
    Title: Less: Contra Accounts
    Description: Contra-Assets To Securities (Reserves)
Note: For Tfr Filers Only"#]
    #[serde(rename="SCRES")]
    pub less_contra_accounts: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCUS`
    Title: U.S. Treasury & Agency
    Description: Total U.S. Treasury Securities and U.S. Government Agency and Corporation Obligations On A Consolidated Basis
Note:
1) From June 2009 Through December 2010 This Item Excluded Commercial Mortgage Backed Securities
2) Prior To June 2009, This Item Included Commercial Mortgage Backed Securities
3) Beginning March 1994 Consists Of Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value Securities
4) Does Not Include Mortgage Derivative Securities From March 1984 Through December 1986 For Tfr Filers
5) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
6) For Banks With Foreign Operations Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCUS")]
    pub u_s_treasury_agency: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCUSA`
    Title: Securities Of Us Agencies
    Description: Securities Of Us Agencies"#]
    #[serde(rename="SCUSA")]
    pub securities_of_us_agencies: Option<i64>,

    #[doc = r#"## FDIC Field:: `SCUST`
    Title: U.S. Treasury Securities
    Description: U.S. Treasury Securities Held-To-Maturity At Amortized Cost and Available-For-Sale At Fair Value On A Consolidated Basis
Note:
(1) Beginning June 1996, Tfr Filers No Longer Report U.S. Treasury Securities Separately
(2) Prior To March 1994 Item Was Defined As Book Value
(3) Includes The Aforementioned Securities Held In Trading Accounts For Tfr Filers
(4) For Banks With Foreign Operations, Data For March & September Of 1973 Through 1975 Are Reported On A Domestic Basis"#]
    #[serde(rename="SCUST")]
    pub u_s_treasury_securities: Option<i64>,

    #[doc = r#"## FDIC Field:: `STNAME`
    Title: Locations
    Description: Locations"#]
    #[serde(rename="STNAME")]
    pub locations: Option<String>,

    #[doc = r#"## FDIC Field:: `STNUM`
    Title: State Number
    Description: State Number"#]
    #[serde(rename="STNUM")]
    pub state_number: Option<String>,

    #[doc = r#"## FDIC Field:: `SUBLLPF`
    Title: Subordinated Notes
    Description: Subordinated Notes and Debentures and Limited-Life Preferred Stock and Related Surplus On A Consolidated Basis
Note: (1) Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domesitc Basis"#]
    #[serde(rename="SUBLLPF")]
    pub subordinated_notes: Option<i64>,

    #[doc = r#"## FDIC Field:: `SUBND`
    Title: Subordinated Notes/Debentures
    Description: Represents all notes and debentures subordinated to deposits and all capital notes and debentures"#]
    #[serde(rename="SUBND")]
    pub subordinated_notes_debentures: Option<i64>,

    #[doc = r#"## FDIC Field:: `TINTINC`
    Title: Int Inc - Total Other
    Description: Total Other Interest Income (Represents the sum of Other Interest Income - Investment Securities, Trading Account Assets, Federal Funds Sold and Securities Purchased, and Balanaces Due from Depository Institutions)"#]
    #[serde(rename="TINTINC")]
    pub int_inc_total_other_tintinc: Option<i64>,

    #[doc = r#"## FDIC Field:: `tochrt`
    Title: Charter Transfers to Commercial Banks
    Description: Represents the charter transfer of existing FDIC-insured savings institutions to an FDIC-insured commercial bank charter."#]
    #[serde(rename="tochrt")]
    pub charter_transfers_to_commercial_banks: Option<i64>,

    #[doc = r#"## FDIC Field:: `tofail`
    Title: Assisted Mergers with Commercial Banks
    Description: Represents the absorption of a failing savings institution by a commercial bank with assistance from either the BIF or SAIF."#]
    #[serde(rename="tofail")]
    pub assisted_mergers_with_commercial_banks: Option<i64>,

    #[doc = r#"## FDIC Field:: `TOINTEXP`
    Title: Int Exp  - Total Deposits
    Description: Total Other Interest Expense (Represents the sum of all components of Other Interest Expense)"#]
    #[serde(rename="TOINTEXP")]
    pub int_exp_total_deposits_tointexp: Option<i64>,

    #[doc = r#"## FDIC Field:: `tomerg`
    Title: Unassisted Mergers with Commercial Banks
    Description: Represents the absorption of a savings institution charter by a commercial bank without assistance."#]
    #[serde(rename="tomerg")]
    pub unassisted_mergers_with_commercial_banks: Option<i64>,

    #[doc = r#"## FDIC Field:: `tortc`
    Title: Failures Transferred to the RTC
    Description: Represents institutions that were declared failed and placed under RTC conservatorship until a buyer(s) is(are) found or a payout to depositors occurs."#]
    #[serde(rename="tortc")]
    pub failures_transferred_to_the_rtc: Option<i64>,

    #[doc = r#"## FDIC Field:: `TOTAL`
    Title: Total Commercial Banks (Total Insured)
    Description: Total Insured Commercial Banks including institutions that did not file a 12/31 fincncial report and other adjustments (See Notes to User)"#]
    #[serde(rename="TOTAL")]
    pub total_commercial_banks_total_insured: Option<i64>,

    #[doc = r#"## FDIC Field:: `TOT_FDIC`
    Title: Total FDIC Supervised Savings Institutions
    Description: Total FDIC Supervised Savings Institutions"#]
    #[serde(rename="TOT_FDIC")]
    pub total_fdic_supervised_savings_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `TOT_OTS`
    Title: Total Non FDIC Supervised Savings Institutions
    Description: Total Non FDIC Supervised Savings Institutions"#]
    #[serde(rename="TOT_OTS")]
    pub total_non_fdic_supervised_savings_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `TOT_SAVE`
    Title: Total Savings Institutions
    Description: All FDIC Insured Savings Institutions filing a 12/31 financial report"#]
    #[serde(rename="TOT_SAVE")]
    pub total_savings_institutions: Option<i64>,

    #[doc = r#"## FDIC Field:: `TPD`
    Title: Total Loans and Leases Past Due
    Description: Total Loans and Leases Past Due"#]
    #[serde(rename="TPD")]
    pub total_loans_and_leases_past_due: Option<i64>,

    #[doc = r#"## FDIC Field:: `TRADE`
    Title: Trading Account Assets
    Description: Assets Held In Trading Accounts On A Consolidated Basis
Note:
(1) Effective March 1994 Item Reported On A Gross Basis
(2) Additional Detail Can Be Found On Schedule Rc-D
(3) For Banks With Foreign Operations Data For March & September Of 1972 Through 1975 Are Reported On A Domestic Basis,
(4) For Periods 1972 Through 1983 Includes Only Securities"#]
    #[serde(rename="TRADE")]
    pub trading_account_assets: Option<i64>,

    #[doc = r#"## FDIC Field:: `TRADES`
    Title: Less: Trading Accounts
    Description: Trading Accounts"#]
    #[serde(rename="TRADES")]
    pub less_trading_accounts: Option<i64>,

    #[doc = r#"## FDIC Field:: `TRN`
    Title: Memo: Domestic Deposits Transaction
    Description: Represents all demand deposits, NOW accounts, ATS accounts, accounts from which payments may be made to third parties by means of an automated teller machine, a remote service unit, or another electronic device, and accounts that permit third party payments through use of checks, drafts, negotiable instruments, or other similar instrument. (MMDA's are specifically excluded from the latter two definitions)"#]
    #[serde(rename="TRN")]
    pub memo_domestic_deposits_transaction: Option<i64>,

    #[doc = r#"## FDIC Field:: `UNASSIST`
    Title: Unassisted Mergers
    Description: Voluntary mergers, consolidations or absorptions of two or more institutions."#]
    #[serde(rename="UNASSIST")]
    pub unassisted_mergers: Option<i64>,

    #[doc = r#"## FDIC Field:: `UNINC`
    Title: Unearned Income
    Description: Unearned Income On Loans On A Consolidated Basis
Note: For Banks With Foreign Operations, Data For March 1976 Through September 1978 Are Domestic Only"#]
    #[serde(rename="UNINC")]
    pub unearned_income: Option<i64>,

    #[doc = r#"## FDIC Field:: `UNIT`
    Title: Unit Banks
    Description: Unit banks are institutions that are operating only one office at which deposits are received or other banking business is conducted."#]
    #[serde(rename="UNIT")]
    pub unit_banks: Option<i64>,

    #[doc = r#"## FDIC Field:: `YEAR`
    Title: Year
    Description: Statistics reported as of end of year."#]
    #[serde(rename="YEAR")]
    pub year: Option<String>,

}

/// FDIC BankFind API `/summary` endpoint handler
/// Get Historical Aggregate Data by Year
/// Returns aggregate financial and structure data, subtotaled by year, regarding finanical institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records included in the calculated result. All values must be entered in UPPERCASE.
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
    Example: STNAME:\"Alabama\"
 - `fields` (String, optional): Comma delimited list of fields with aggregated annual financial data to return. All values must be entered in UPPERCASE.
    Example: STNAME,YEAR,INTINC,EINTEXP,NIM,NONII,NONIX,ELNATR,ITAXR,IGLSEC,ITAX,EXTRA,NETINC
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: YEAR
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
 - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
 - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
 - `agg_limit` (u32, optional): The limit on how many aggregated results will be displayed.
 - `max_value` (String, optional): The field by which the max value is desired.
 - `max_value_by` (String, optional): The field that will be used to determine unique records, similar to a primary key (i.e. CERT). All values must be entered in UPPERCASE.
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/summary",
    params(SummaryParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<SummaryProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Historical"
)]
pub async fn summary_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<SummaryParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "summary",
    )
    .await
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
            
            all_other_loans: None,
            new_charters_to_absorb_another_charter: None,
            total_assets: None,
            total_commercial_banks_filing_y_e_call: None,
            bank_premises_and_equipment: None,
            total_branches: None,
            banks_with_branches: None,
            memo_brokered_deposits: None,
            borrowed_funds: None,
            commercial_banks_cb_vs_savings_institution_si: None,
            charter_transfers_from_commercial_banks: None,
            cash_due_from_depository_institutions: None,
            interest_earning_balances: None,
            non_insured_becoming_insured: None,
            assisted_mergers_with_thrifts: None,
            unassisted_mergers_consolidations_of_thrifts: None,
            rtc_conservatorships: None,
            other_debt_securities: None,
            total_savings_institutions_filing_y_e_call: None,
            loan_lease_recoveries: None,
            deposits_domestic_demand: None,
            total_deposits: None,
            total_domestic_deposits: None,
            total_foreign_deposits: None,
            interest_bearing_deposits: None,
            foreign_deposits_interest_bearing: None,
            memo_deposits_non_interest_bearing: None,
            foreign_deposits_non_interest_bearing: None,
            loan_lease_charge_offs: None,
            memo_amortization_of_intangibles: None,
            int_exp_total_deposits: None,
            int_exp_deposit_in_domestic_offices: None,
            int_exp_deposits_in_foreign_offices: None,
            fed_funds_purchased_securities_sold: None,
            int_exp_oth_advances_from_fhlb: None,
            int_exp_fed_funds_purchased_securities_sold: None,
            total_interest_expense: None,
            eintexp2: None,
            provision_for_loan_lease_losses: None,
            all_other_non_interest_expenses: None,
            occupancy_expense: None,
            total_equity: None,
            total_cash_dividends_declared: None,
            cash_dividends_declared_common: None,
            cash_dividends_declared_preferred: None,
            common_stock: None,
            total_cash_divident_declared: None,
            total_equity_capital: None,
            fdic_net_worth_certificates: None,
            other_capital: None,
            perpetual_preferred_stock: None,
            surplus: None,
            undivided_profits: None,
            employee_salaries_and_benefits: None,
            int_exp_subordinated_notes: None,
            net_extraordinary_items: None,
            fdic_supervised_bif_insured_institutions: None,
            fdic_supervised_saif_insured_institutions: None,
            federal_funds_sold: None,
            fed_funds_repos_purchased: None,
            int_inc_balances_due: None,
            fee_income: None,
            int_inc_fed_funds_sold_securities_purchased: None,
            securities_gains_and_losses: None,
            int_inc_domestic_office_loans: None,
            int_inc_foreign_office_loans: None,
            int_inc_total_loans_leases: None,
            int_inc_loans: None,
            int_inc_leases: None,
            intangible_assets: None,
            total_interest_earning_assets: None,
            total_interest_bearing_liabilities: None,
            total_interest_income: None,
            intinc2: None,
            memo_ira_s_and_keogh_plan_deposits: None,
            int_inc_investment_securities: None,
            service_charges_on_deposit_accounts: None,
            applicable_income_taxes: None,
            pre_tax_net_operating_income: None,
            int_inc_trading_account_assets: None,
            total_liabilities: None,
            total_liabilities_and_equity_capital: None,
            assisted_payouts: None,
            voluntary_liquidations: None,
            agricultural_loans: None,
            all_other_loans_to_individuals: None,
            allowance_for_losses_loans_and_leases: None,
            memo_loans_to_individuals_auto: None,
            commercial_and_industrial_loans: None,
            total_loans_to_individuals: None,
            loans_to_individuals_home_improvement: None,
            loans_to_individuals_all_others: None,
            loans_to_individuals_credit_card_plans: None,
            loans_to_deposit_institutions: None,
            gross_loans_and_leases: None,
            total_loans_and_leases: None,
            net_loans_and_leases: None,
            memo_loans_to_individuals_mobile_homes: None,
            loans_to_states_and_politicial_sub_divisions: None,
            total_real_estate_loans: None,
            r_e_loan_farmland: None,
            r_e_loan_construction_land_develop: None,
            total_r_e_loans_in_domestic_offices: None,
            total_r_e_loans_in_foreign_offices: None,
            memo_home_equity_loans: None,
            r_e_loans_multifamily: None,
            r_e_loan_non_farm_non_residential_prop: None,
            r_e_loan_1_4_family: None,
            memo_contra_account: None,
            memo_loans_to_individuals_single_payment: None,
            leases: None,
            failures_assisted_merger: None,
            other_misc_adjustments: None,
            mortgage_and_other_borrowings: None,
            non_accrual_loans_leases: None,
            net_loans_and_leases_charge_offs: None,
            total_non_current_loans_leases: None,
            net_income_attributable_to_noncontrolling_interests: None,
            net_income: None,
            new_charters: None,
            new_charters_new_char: None,
            int_exp_borrowed_money: None,
            all_other_assets: None,
            corporate_bonds_and_other_securities: None,
            trading_account_securities: None,
            memo_valuation_reserves: None,
            all_other_loans_new11_1: None,
            borrowed_funds_new14_1: None,
            other_liabilities: None,
            total_liabilities_new14_3: None,
            undivided_profits_new14_4: None,
            deposits_individuals_partnerships_and_corporations: None,
            deposits_u_s_government: None,
            deposits_states_and_political_subdivisions: None,
            deposits_all_other: None,
            deposits_domestic_savings: None,
            total_domestic_deposits_new15_7: None,
            demand_notes_and_other_liabilities: None,
            interest_bearing_deposits_new16_2: None,
            net_interest_income: None,
            total_non_interest_income: None,
            total_non_interest_expense: None,
            net_loan_lease_charge_offs: None,
            memo_domestic_deposits_non_transaction: None,
            deposits_domestic_time: None,
            memo_time_deposits_over_100k: None,
            number_of_full_time_employees: None,
            other_earning_assests: None,
            offices: None,
            demand_notes_and_other_borrowings: None,
            total_other_interest_expenses: None,
            int_inc_total_other: None,
            other_non_interest_income: None,
            other_real_estate_owned: None,
            other_real_estate: None,
            non_fdic_supervised_bif_insured_institutions: None,
            non_fdic_supervised_saif_insured_institutions: None,
            all_other_assets_othasst: None,
            advances_from_fhlb: None,
            int_exp_oth_borrowed_money: None,
            other_equity: None,
            other: None,
            other_liabilities_othliab: None,
            borrowed_funds_othnborr: None,
            less_other_contra_accounts: None,
            failures_paid_off: None,
            loans_leases_p_d_30_89_days: None,
            loans_leases_p_d_90_days: None,
            pre_tax_net_operating_income_ptxnoinc: None,
            conversions: None,
            total_savings_institutions_total_insured: None,
            total_investment_securities_book_value: None,
            u_s_agencies_and_corporation_securities: None,
            equity_securities: None,
            memo_mortgage_backed_securities: None,
            states_and_political_subdivisions_securities: None,
            market_values: None,
            less_contra_accounts: None,
            u_s_treasury_agency: None,
            securities_of_us_agencies: None,
            u_s_treasury_securities: None,
            locations: None,
            state_number: None,
            subordinated_notes: None,
            subordinated_notes_debentures: None,
            int_inc_total_other_tintinc: None,
            charter_transfers_to_commercial_banks: None,
            assisted_mergers_with_commercial_banks: None,
            int_exp_total_deposits_tointexp: None,
            unassisted_mergers_with_commercial_banks: None,
            failures_transferred_to_the_rtc: None,
            total_commercial_banks_total_insured: None,
            total_fdic_supervised_savings_institutions: None,
            total_non_fdic_supervised_savings_institutions: None,
            total_savings_institutions: None,
            total_loans_and_leases_past_due: None,
            trading_account_assets: None,
            less_trading_accounts: None,
            memo_domestic_deposits_transaction: None,
            unassisted_mergers: None,
            unearned_income: None,
            unit_banks: None,
            year: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
