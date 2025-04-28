//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/locations` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// Auto-generated parameters struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
}

// Implement QueryParameters for generic handler
impl QueryParameters for LocationsParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ADDRESS",
        "BKCLASS",
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
        "CITY",
        "COUNTY",
        "CSA",
        "CSA_FLG",
        "CSA_NO",
        "ESTYMD",
        "FI_UNINUM",
        "LATITUDE",
        "LONGITUDE",
        "MDI_STATUS_CODE",
        "MDI_STATUS_DESC",
        "MAINOFF",
        "NAME",
        "OFFNAME",
        "OFFNUM",
        "RUNDATE",
        "SERVTYPE",
        "SERVTYPE_DESC",
        "STALP",
        "STCNTY",
        "STNAME",
        "UNINUM",
        "ZIP",
    ];

    #[allow(unused_variables)]
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationsProperties {
    #[doc = r#"Title: Street Address"#]
    #[doc = r#"Description: The street address in which an institution or branch office is physically located."#]
    #[serde(rename="ADDRESS")]
    pub address: Option<String>,

    #[doc = r#"Title: Institution Class"#]
    #[doc = r#"Description: A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#]
    #[serde(rename="BKCLASS")]
    pub bkclass: Option<String>,

    #[doc = r#"Title: Core Based Statistical Area Name"#]
    #[doc = r#"Description: Name of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA")]
    pub cbsa: Option<String>,

    #[doc = r#"Title: Metropolitan Divisions Name"#]
    #[doc = r#"Description: Name of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_DIV")]
    pub cbsa_div: Option<String>,

    #[doc = r#"Title: Metropolitan Divisions Flag"#]
    #[doc = r#"Description: A flag (1=Yes) indicating member of a Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_DIV_FLG")]
    pub cbsa_div_flg: Option<String>,

    #[doc = r#"Title: Metropolitan Divisions Number"#]
    #[doc = r#"Description: Numeric code of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_DIV_NO")]
    pub cbsa_div_no: Option<String>,

    #[doc = r#"Title: Metropolitan Division Number"#]
    #[doc = r#"Description: Numeric code of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO")]
    pub cbsa_metro: Option<String>,

    #[doc = r#"Title: Metropolitan Division Flag"#]
    #[doc = r#"Description: A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO_FLG")]
    pub cbsa_metro_flg: Option<String>,

    #[doc = r#"Title: Metropolitan Division Name"#]
    #[doc = r#"Description: Name of the Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_METRO_NAME")]
    pub cbsa_metro_name: Option<String>,

    #[doc = r#"Title: Micropolitan Division Flag"#]
    #[doc = r#"Description: A flag (1=Yes) used to indicate whether an branch is in a Metropolitan Statistical Area as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CBSA_MICRO_FLG")]
    pub cbsa_micro_flg: Option<String>,

    #[doc = r#"Title: Core Based Statistical Areas"#]
    #[doc = r#"Description: Numeric code of the Core Based Statistical Area (CBSA) as defined by the US Census Bureau Office of Management and Budget."#]
    #[serde(rename="CBSA_NO")]
    pub cbsa_no: Option<String>,

    #[doc = r#"Title: FDIC Certificate #"#]
    #[doc = r#"Description: A unique number assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub cert: Option<String>,

    #[doc = r#"Title: City"#]
    #[doc = r#"Description: The city in which an institution or branch office is physically located."#]
    #[serde(rename="CITY")]
    pub city: Option<String>,

    #[doc = r#"Title: County"#]
    #[doc = r#"Description: The county name in which an institution or branch office is physically located."#]
    #[serde(rename="COUNTY")]
    pub county: Option<String>,

    #[doc = r#"Title: Combined Statistical Area Name"#]
    #[doc = r#"Description: Name of the Combined Statistical Area (CSA) as defined by the US Census Bureau."#]
    #[serde(rename="CSA")]
    pub csa: Option<String>,

    #[doc = r#"Title: Combined Statistical Area Flag  (Branch)"#]
    #[doc = r#"Description: Flag (1=Yes) indicating member of a Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CSA_FLG")]
    pub csa_flg: Option<String>,

    #[doc = r#"Title: Combined Statistical Area Number  (Branch)"#]
    #[doc = r#"Description: Numeric code of the Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CSA_NO")]
    pub csa_no: Option<String>,

    #[doc = r#"Title: Branch Established Date"#]
    #[doc = r#"Description: The date on which the branch began operations."#]
    #[serde(rename="ESTYMD")]
    pub estymd: Option<String>,

    #[doc = r#"Title: FDIC's unique number"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps the branch back to the parent financial institution."#]
    #[serde(rename="FI_UNINUM")]
    pub fi_uninum: Option<String>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: The latitude of the physical address."#]
    #[serde(rename="LATITUDE")]
    pub latitude: Option<f32>,

    #[doc = r#"Title: Location Address Latitude"#]
    #[doc = r#"Description: The longitude of the physical address."#]
    #[serde(rename="LONGITUDE")]
    pub longitude: Option<f32>,

    #[doc = r#"Title: Minority Status Code"#]
    #[doc = r#"Description: A numeric flag used to indicate whether an institution is primarily a minority owned institution."#]
    #[serde(rename="MDI_STATUS_CODE")]
    pub mdi_status_code: Option<String>,

    #[doc = r#"Title: Minority Status Description"#]
    #[doc = r#"Description: A descriptive flag used to indicate type of minority owned institution."#]
    #[serde(rename="MDI_STATUS_DESC")]
    pub mdi_status_desc: Option<String>,

    #[doc = r#"Title: Main Office"#]
    #[doc = r#"Description: Flag (1=Yes) indicating this location is the main office for the institution."#]
    #[serde(rename="MAINOFF")]
    pub mainoff: Option<f32>,

    #[doc = r#"Title: Institution Name (Search-Eligible)"#]
    #[doc = r#"Description: The legal title or name of the institution. This field can be used for search and filtering."#]
    #[serde(rename="NAME")]
    pub name: Option<String>,

    #[doc = r#"Title: Office Name"#]
    #[doc = r#"Description: Branch office name."#]
    #[serde(rename="OFFNAME")]
    pub offname: Option<String>,

    #[doc = r#"Title: Branch Number"#]
    #[doc = r#"Description: An institution's branch office number used internally by FDIC."#]
    #[serde(rename="OFFNUM")]
    pub offnum: Option<String>,

    #[doc = r#"Title: Run Date"#]
    #[doc = r#"Description: The day the institution information was updated."#]
    #[serde(rename="RUNDATE")]
    pub rundate: Option<String>,

    #[doc = r#"Title: Service Type Code"#]
    #[doc = r#"Description: Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch"#]
    #[serde(rename="SERVTYPE")]
    pub servtype: Option<f32>,

    #[doc = r#"Title: Service Type Code Description (Search-Eligible)"#]
    #[doc = r#"Description: Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch This field can be used for search and filtering."#]
    #[serde(rename="SERVTYPE_DESC")]
    pub servtype_desc: Option<String>,

    #[doc = r#"Title: State Alpha Code"#]
    #[doc = r#"Description: The state abbreviation of the location of the institution's main office."#]
    #[serde(rename="STALP")]
    pub stalp: Option<String>,

    #[doc = r#"Title: State and County Number"#]
    #[doc = r#"Description: A five digit number representing the state and county in which the institution is physically located.  The first two digits represent the FIPS state numeric code and the last three digits represent the FIPS county numeric code."#]
    #[serde(rename="STCNTY")]
    pub stcnty: Option<String>,

    #[doc = r#"Title: Branch State"#]
    #[doc = r#"Description: State in which the  branch is physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="STNAME")]
    pub stname: Option<String>,

    #[doc = r#"Title: FDIC's Unique Number"#]
    #[doc = r#"Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    #[serde(rename="UNINUM")]
    pub uninum: Option<String>,

    #[doc = r#"Title: Zip Code"#]
    #[doc = r#"Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#]
    #[serde(rename="ZIP")]
    pub zip: Option<String>,

}

/// Auto-generated response envelope struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationsResponse {
    #[doc = r#"Title: "#]
    #[doc = r#"Description: "#]
    #[serde(rename="data")]
    pub data: Option<String>,

}

/// FDIC BankFind API `/locations` endpoint handler
/// Get Institution Locations
/// Returns locations/branches of financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter for the location search. All values must be entered in UPPERCASE. - `fields` (String, optional): Comma delimited list of fields to return. All values must be entered in UPPERCASE.
NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
NAME - `sort_order` (String, optional): Indicator if ascending (ASC) or descending (DESC). All values must be entered in UPPERCASE.
DESC - `limit` (i32, optional): The number of records to return. Default is 10 and maximum is 10,000. - `offset` (i32, optional): The offset of page to return. - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /locations
Parameters: LocationsParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Structure"#]
pub async fn locations_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<LocationsParameters>,
) -> Response {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "locations",
        method = "GET",
        path = "/locations",
        params = serde_json::to_string(&params).unwrap()
    );
    let resp = list_endpoint(
        State(config),
        Query(params.clone()),
        "locations",
    ).await;
    // Log outgoing FDIC API request as structured JSON
    debug!(
        target = "fdic_proxy",
        event = "proxied_fdic_api_request",
        endpoint = "locations",
        method = "GET",
        path = "/locations",
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
        let params = LocationsParameters {
            common: CommonParameters::default(),
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }
    #[test]
    fn test_properties_struct_serialization() {
        let props = LocationsProperties {
            
            address: None,
            bkclass: None,
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
            city: None,
            county: None,
            csa: None,
            csa_flg: None,
            csa_no: None,
            estymd: None,
            fi_uninum: None,
            latitude: None,
            longitude: None,
            mdi_status_code: None,
            mdi_status_desc: None,
            mainoff: None,
            name: None,
            offname: None,
            offnum: None,
            rundate: None,
            servtype: None,
            servtype_desc: None,
            stalp: None,
            stcnty: None,
            stname: None,
            uninum: None,
            zip: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
