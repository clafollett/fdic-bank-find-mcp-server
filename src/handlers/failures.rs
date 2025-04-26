//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/failures` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/failures` endpoint.
/// Spec: failure_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct FailuresParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
    #[doc = r#"Fields to sum up (in a totals response object). Only numeric columns are valid. All values must be entered in UPPERCASE."#]
    #[param(rename = "total_fields")]
    pub total_fields: Option<String>,
    #[doc = r#"The field by which data will be subtotaled (in totals response object). Only categorical values should be used. All values must be entered in UPPERCASE."#]
    #[param(rename = "subtotal_by")]
    pub subtotal_by: Option<String>,
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
impl QueryParameters for FailuresParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "NAME",
        "CERT",
        "FIN",
        "CITYST",
        "FAILDATE",
        "FAILYR",
        "SAVR",
        "RESTYPE1",
        "CHCLASS1",
        "RESDATE",
        "RESTYPE",
        "QBFDEP",
        "QBFASSET",
        "COST",
        "PSTALP",
    ];

    #[allow(unused_variables)]
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        if let Some(val) = &self.total_fields {
            query.insert("total_fields".to_string(), val.to_string());
        }
        if let Some(val) = &self.subtotal_by {
            query.insert("subtotal_by".to_string(), val.to_string());
        }
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

/// Auto-generated properties struct for `/failures` endpoint.
/// Spec: failure_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct FailuresProperties {
    #[doc = r#"## FDIC Field:: `NAME`
    Title: Institution Name
    Description: This is the legal name of the institution. When available, the Institution's name links to useful information for the customers and vendors of these institutions. This information includes press releases, information about the acquiring institution, (if applicable), how your accounts and loans are affected, and how vendors can file claims against the receivership."#]
    #[serde(rename="NAME")]
    pub institution_name: Option<String>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: Cert
    Description: The certificate number assigned by the FDIC used to identify institutions and for the issuance of insurance certificates. By clicking on this number, you will link to the Institution Directory (ID) system which will provide the last demographic and financial data filed by the selected institution."#]
    #[serde(rename="CERT")]
    pub cert: Option<String>,

    #[doc = r#"## FDIC Field:: `FIN`
    Title: FIN
    Description: Financial Institution Number (FIN) is a unique number assigned to the institution as an Assistance Agreement, Conservatorship, Bridge Bank or Receivership."#]
    #[serde(rename="FIN")]
    pub fin: Option<String>,

    #[doc = r#"## FDIC Field:: `CITYST`
    Title: Location
    Description: The city and state (or territory) of the headquarters of the institution."#]
    #[serde(rename="CITYST")]
    pub location: Option<String>,

    #[doc = r#"## FDIC Field:: `FAILDATE`
    Title: Effective Date
    Description: The date that the failed / assisted institution ceased to exist as a privately held going concern. For institutions that entered into government ownership, such as FDIC Bridge Banks and RTC conservatorships, this is the date that they entered into such ownership."#]
    #[serde(rename="FAILDATE")]
    pub effective_date: Option<String>,

    #[doc = r#"## FDIC Field:: `FAILYR`
    Title: Year
    Description: The 4-digit year that the failed / assisted institution ceased to exist as a privately held going concern. For institutions that entered into government ownership, such as FDIC Bridge Banks and RTC conservatorships, this is the date that they entered into such ownership."#]
    #[serde(rename="FAILYR")]
    pub year: Option<String>,

    #[doc = r#"## FDIC Field:: `SAVR`
    Title: Insurance Fund
    Description: Before 1989, there were two federal deposit insurance funds, one administered by the FDIC, which insured deposits in commercial banks and state-chartered savings banks, and another administered by the Federal Savings and Loan Insurance Corporation (FSLIC), which insured deposits in state- and federally-chartered savings associations. In 1989, the Financial Institutions Reform, Recovery and Enforcement Act (FIRREA) specified that thereafter the FDIC would be the federal deposit insurer of all banks and savings associations and would administer both the FDIC fund, which was renamed the Bank Insurance Fund (BIF) and the replacement for the insolvent FSLIC fund, which was called the Savings Association Insurance Fund (SAIF). Although it was created in 1989, the SAIF was not responsible for savings association failures until 1996. From 1989 through 1995, savings association failures were the responsibility of the Resolution Trust Corporation (RTC). In February 2006, The Federal Deposit Insurance Reform Act of 2005 provided for the merger of the BIF and the SAIF into a single Deposit Insurance Fund (DIF). Necessary technical and conforming changes to the law were made under The Federal Deposit Insurance Reform Conforming Amendments Act of 2005. The merger of the funds was effective on March 31, 2006. For additional information about deposit insurance fund and legislation, go to http://www.fdic.gov/deposit/insurance/index.html."#]
    #[serde(rename="SAVR")]
    pub insurance_fund: Option<String>,

    #[doc = r#"## FDIC Field:: `RESTYPE1`
    Title: Transaction Type
    Description: Institutions have been resolved through several different types of transactions. The transaction types outlined below can be grouped into three general categories, based upon the method employed to protect insured depositors and how each transaction affects a failed / assisted institution's charter. In most assistance transactions, insured and uninsured depositors are protected, the failed / assisted institution remains open and its charter survives the resolution process. In purchase and assumption transactions, the failed / assisted institution's insured deposits are transferred to a successor institution, and its charter is closed. In most of these transactions, additional liabilities and assets are also transferred to the successor institution. In payoff transactions, the deposit insurer - the FDIC or the former Federal Savings and Loan Insurance Corporation - pays insured depositors, the failed / assisted institution's charter is closed, and there is no successor institution. For a more complete description of resolution transactions and the FDIC's receivership activities, see Managing the Crisis: The FDIC and RTC Experience, a study prepared by the FDIC's Division of Resolutions and Receiverships. Copies are available from the FDIC's Public Information Center.
Category 1 - Institution's charter survives
A/A	- Assistance Transactions. These include: 1) transactions where assistance was provided to the acquirer, who purchased the entire institution. For a few FSLIC transactions, the acquirer purchased the entire bridge bank - type entity, but certain other assets were moved into a liquidating receivership prior to the sale, and 2) open bank assistance transactions, including those where assistance was provided under a systemic risk determination (in such cases any costs that exceed the amounts estimated under the least cost resolution requirement would be recovered through a special assessment on all FDIC-insured institutions).
REP -	Reprivatization, management takeover with or without assistance at takeover, followed by a sale with or without additional assistance.
Category 2 - Institution's charter is terminated, insured deposits plus some assets and other liabilities are transferred to a successor charter
P&A - Purchase and Assumption, where some or all of the deposits, certain other liabilities and a portion of the assets (sometimes all of the assets) were sold to an acquirer. It was not determined if all of the deposits (PA) or only the insured deposits (PI) were assumed.
PA - Purchase and Assumption, where the insured and uninsured deposits, certain other liabilities and a portion of the assets were sold to an acquirer.
PI - Purchase and Assumption of the insured deposits only, where the traditional P&A was modified so that only the insured deposits were assumed by the acquiring institution.
IDT - Insured Deposit Transfer, where the acquiring institution served as a paying agent for the insurer, established accounts on their books for depositors, and often acquired some assets as well. Includes ABT (asset-backed transfer, a FSLIC transaction that is very similar to an IDT).
MGR - An institution where FSLIC took over management and generally provided financial assistance. FSLIC closed down before the institution was sold.
Category 3
PO - Payout, where the insurer paid the depositors directly and placed the assets in a liquidating receivership. Note: Includes transactions where the FDIC established a Deposit Insurance National Bank to facilitate the payout process."#]
    #[serde(rename="RESTYPE1")]
    pub transaction_type: Option<String>,

    #[doc = r#"## FDIC Field:: `CHCLASS1`
    Title: Charter Class
    Description: The FDIC assigns classification codes indicating an institution's charter type (commercial bank, savings bank, or savings association), its chartering agent (state or federal government), its Federal Reserve membership status (member or nonmember), and its primary federal regulator (state-chartered institutions are subject to both federal and state supervision). These codes are:
N - National chartered commercial bank supervised by the Office of the Comptroller of the Currency;
SM - State charter Fed member commercial bank supervised by the Federal Reserve;
NM - State charter Fed nonmember commercial bank supervised by the FDIC;
SA - State or federal charter savings association supervised by the Office of Thrift Supervision or Office of the Comptroller of the Currency;
SB - State charter savings bank supervised by the FDIC."#]
    #[serde(rename="CHCLASS1")]
    pub charter_class: Option<String>,

    #[doc = r#"## FDIC Field:: `RESDATE`
    Title: Date of Resolution
    Description: Date of Resolution. Usually the same as the fail date, except with banks with Open Bank Assistance."#]
    #[serde(rename="RESDATE")]
    pub date_of_resolution: Option<String>,

    #[doc = r#"## FDIC Field:: `RESTYPE`
    Title: Resolution
    Description: The given institution has failure stature or it can be assistance has been provided by FDIC in merging with other institution."#]
    #[serde(rename="RESTYPE")]
    pub resolution: Option<String>,

    #[doc = r#"## FDIC Field:: `QBFDEP`
    Title: Total Deposits
    Description: Total including demand deposits, money market deposits, other savings deposits, time deposits and deposits in foreign offices as of the last Call Report or Thrift Financial Report filed by the institution prior to the effective date. Note this does not necessarily reflect total deposits on the last report filed because in some cases reports were filed after the effective date."#]
    #[serde(rename="QBFDEP")]
    pub total_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `QBFASSET`
    Title: Total Assets
    Description: The Total assets owned by the institution including cash, loans, securities, bank premises and other assets as of the last Call Report or Thrift Financial Report filed by the institution prior to the effective date. Note this does not necessarily reflect total assets on the last report filed because in some cases reports were filed after the effective date. This total does not include off-balance-sheet accounts."#]
    #[serde(rename="QBFASSET")]
    pub total_assets: Option<f64>,

    #[doc = r#"## FDIC Field:: `COST`
    Title: Estimated Loss
    Description: The estimated loss is the difference between the amount disbursed from the Deposit Insurance Fund (DIF) to cover obligations to insured depositors and the amount estimated to be ultimately recovered from the liquidation of the receivership estate. Estimated losses reflect unpaid principal amounts deemed unrecoverable and do not reflect interest that may be due on the DIF's administrative or subrogated claims should its principal be repaid in full.
Notes:
Comprehensive data on estimated losses are not available for FDIC-insured failures prior to 1986, or for FSLIC-insured failures from 1934-88. Estimated loss is presented as "N/A" in years for which comprehensive information is not available.
Estimated Loss data was previously referred to as 'Estimated Cost' in past releases of the Historical Statistic on Banking. For RTC receiverships, the 'Estimated Cost' included an allocation of FDIC corporate revenue and expense items such as interest expense on Federal Financing Bank debt, interest expense on escrowed funds and interest revenue on advances to receiverships. Other FDIC receiverships did not include such an allocation. To maintain consistency with FDIC receiverships, the RTC allocation is no longer reflected in the estimated loss amounts for failed / assisted institutions that were resolved through RTC receiverships.
Beginning with the release of 2007 information, the 'Estimated Loss' in the Historical Statistics on Banking is presented and defined consistently with the aggregate Estimated Receivership Loss for FRF-RTC institutions and Estimated Losses for FDIC receiverships that are reported in the FDIC's Annual Report. The estimated loss is obtained from the FDIC's Failed Bank Cost Analysis (FBCA) report and the RTC Loss report. The FBCA provides data for receiverships back to 1986. The RTC Loss Report provides similar data back to 1989. 
Questions regarding Estimated Loss should be sent to DOFBusinessCenter@fdic.gov. 
Also, for more detail regarding resolution transactions and the FDIC's receivership activities, see Managing the Crisis: The FDIC and RTC Experience, a historical study prepared by the FDIC's Division of Resolutions and Receiverships. Copies are available from the FDIC's Public Information Center."#]
    #[serde(rename="COST")]
    pub estimated_loss: Option<f64>,

    #[doc = r#"## FDIC Field:: `PSTALP`
    Title: State
    Description: Two-character alphanumeric code for US state or Territory"#]
    #[serde(rename="PSTALP")]
    pub state: Option<String>,

}

/// FDIC BankFind API `/failures` endpoint handler
/// Get detail on historical bank failures from 1934 to present.
/// Returns details on failed financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records returned. 
Examples: All values must be entered in UPPERCASE.
* Filter by Location              
`CITYST:\"MEMPHIS, TN\"`  
* Filter all but a specified value  
`!(CITY:\"PITTSBURGH\")`  
* Filter by institution fail year range  
`FAILYR:&#91;\"2015\" TO \"2016\"&#93;`
    Example: FAILYR:&#91;\"2014\" TO \"2015\"&#93;
 - `fields` (String, optional): Comma delimited list of fields of failed financial institutions to return. All values must be entered in UPPERCASE.
    Example: NAME,CERT,FIN,CITYST,FAILDATE,SAVR,RESTYPE,RESTYPE1,QBFDEP,QBFASSET,COST
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: FAILDATE
 - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
    Example: DESC
 - `limit` (u32, optional): The number of records to return. Default is 10 and maximum is 10,000.
    Example: 10
 - `offset` (u32, optional): The offset of page to return.
 - `total_fields` (String, optional): Fields to sum up (in a totals response object). Only numeric columns are valid. All values must be entered in UPPERCASE.
    Example: QBFDEP,QBFASSET,COST
 - `subtotal_by` (String, optional): The field by which data will be subtotaled (in totals response object). Only categorical values should be used. All values must be entered in UPPERCASE.
    Example: RESTYPE
 - `agg_by` (String, optional): The field by which data will be aggregated. All values must be entered in UPPERCASE.
    Example: FAILYR
 - `agg_term_fields` (String, optional): The field(s) for which aggregations will be counted for each unique term. All values must be entered in UPPERCASE.
    Example: RESTYPE
 - `agg_sum_fields` (String, optional): The field(s) for which aggregations will be summed or aggregated. All values must be entered in UPPERCASE.
    Example: QBFASSET,QBFDEP,COST
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
    path = "/failures",
    params(FailuresParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<FailuresProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Failures"
)]
pub async fn failures_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<FailuresParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "failures",
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = FailuresParameters {
            common: CommonParameters::default(),
            total_fields: None,
            subtotal_by: None,
            agg_by: None,
            agg_term_fields: None,
            agg_sum_fields: None,
            agg_limit: None,
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }
    #[test]
    fn test_properties_struct_serialization() {
        let props = FailuresProperties {
            
            institution_name: None,
            cert: None,
            fin: None,
            location: None,
            effective_date: None,
            year: None,
            insurance_fund: None,
            transaction_type: None,
            charter_class: None,
            date_of_resolution: None,
            resolution: None,
            total_deposits: None,
            total_assets: None,
            estimated_loss: None,
            state: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
