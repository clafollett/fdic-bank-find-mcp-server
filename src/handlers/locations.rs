//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/locations` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/locations` endpoint.
/// Spec: location_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
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
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct LocationsProperties {
    #[doc = r#"## FDIC Field:: `ADDRESS`
    Title: Street Address
    Description: The street address in which an institution or branch office is physically located."#]
    #[serde(rename="ADDRESS")]
    pub street_address: Option<String>,

    #[doc = r#"## FDIC Field:: `BKCLASS`
    Title: Institution Class
    Description: A classification code assigned by the FDIC based on the institution's charter type (commercial bank or savings institution), charter agent (state or federal), Federal Reserve membership status (Fed member, Fed non-member) and its primary federal regulator (state chartered institutions are subject to both federal and state supervision). N - Commercial bank, national (federal) charter, Fed member, and supervised by the Office of the Comptroller of the Currency (OCC); NM - Commercial bank, state charter, Fed non-member, and supervised by the Federal Deposit Insurance Corporation (FDIC); OI - Insured U.S. branch of a foreign chartered institution (IBA) and supervised by the OCC or FDIC; SB – Federal savings banks, federal charter, supervised by the OCC or before July 21,2011 the Office of Thrift Supervision (OTS); SI - State chartered stock savings banks, supervised by the FDIC; SL - State chartered stock savings and loan associations, supervised by the FDIC or before July 21,2011 the OTS; SM - Commercial bank, state charter, Fed member, and supervised by the Federal Reserve Bank (FRB); NC – Noninsured non-deposit commercial banks and/or trust companies regulated by the OCC, a state, or a territory; NS - Noninsured stock savings bank supervised by a state or territory; CU - state or federally chartered credit unions supervised by the National Credit Union Association (NCUA)."#]
    #[serde(rename="BKCLASS")]
    pub institution_class: Option<String>,

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
    Description: Numeric code of the Core Based Statistical Division as defined by the US Census Bureau Office of Management and Budget."#]
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
    Description: A unique number assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub fdic_certificate: Option<String>,

    #[doc = r#"## FDIC Field:: `CITY`
    Title: City
    Description: The city in which an institution or branch office is physically located."#]
    #[serde(rename="CITY")]
    pub city: Option<String>,

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

    #[doc = r#"## FDIC Field:: `CSA_FLG`
    Title: Combined Statistical Area Flag  (Branch)
    Description: Flag (1=Yes) indicating member of a Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CSA_FLG")]
    pub combined_statistical_area_flag_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `CSA_NO`
    Title: Combined Statistical Area Number  (Branch)
    Description: Numeric code of the Combined Statistical Area (CSA) as defined by the US Census Bureau Office of Management and Budget"#]
    #[serde(rename="CSA_NO")]
    pub combined_statistical_area_number_branch: Option<String>,

    #[doc = r#"## FDIC Field:: `ESTYMD`
    Title: Branch Established Date
    Description: The date on which the branch began operations."#]
    #[serde(rename="ESTYMD")]
    pub branch_established_date: Option<String>,

    #[doc = r#"## FDIC Field:: `FI_UNINUM`
    Title: FDIC's unique number
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries. This value maps the branch back to the parent financial institution."#]
    #[serde(rename="FI_UNINUM")]
    pub fdic_s_unique_number: Option<String>,

    #[doc = r#"## FDIC Field:: `LATITUDE`
    Title: Location Address Latitude
    Description: The latitude of the physical address."#]
    #[serde(rename="LATITUDE")]
    pub location_address_latitude: Option<f64>,

    #[doc = r#"## FDIC Field:: `LONGITUDE`
    Title: Location Address Latitude
    Description: The longitude of the physical address."#]
    #[serde(rename="LONGITUDE")]
    pub location_address_latitude_longitude: Option<f64>,

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

    #[doc = r#"## FDIC Field:: `MAINOFF`
    Title: Main Office
    Description: Flag (1=Yes) indicating this location is the main office for the institution."#]
    #[serde(rename="MAINOFF")]
    pub main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `NAME`
    Title: Institution Name
    Description: The legal title or name of the institution."#]
    #[serde(rename="NAME")]
    pub institution_name: Option<String>,

    #[doc = r#"## FDIC Field:: `OFFNAME`
    Title: Office Name
    Description: Branch office name."#]
    #[serde(rename="OFFNAME")]
    pub office_name: Option<String>,

    #[doc = r#"## FDIC Field:: `OFFNUM`
    Title: Branch Number
    Description: An institution's branch office number used internally by FDIC."#]
    #[serde(rename="OFFNUM")]
    pub branch_number: Option<String>,

    #[doc = r#"## FDIC Field:: `RUNDATE`
    Title: Run Date
    Description: The day the institution information was updated."#]
    #[serde(rename="RUNDATE")]
    pub run_date: Option<String>,

    #[doc = r#"## FDIC Field:: `SERVTYPE`
    Title: Service Type Code
    Description: Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch"#]
    #[serde(rename="SERVTYPE")]
    pub service_type_code: Option<f64>,

    #[doc = r#"## FDIC Field:: `SERVTYPE_DESC`
    Title: Service Type Code Description
    Description: Define the various types of offices of FDIC-insured institutions. 11 - Full Service - Brick and Mortar; 12 - Full Service - Retail; 13 - Full Service - Home Banking; 14 - Full Service Mobile Office; 15 - Full Service Home/Phone Banking; 16 - Full Service Seasonal Office; 21 - Limited Service - Administrative; 22 - Limited Service - Military Facility; 23 - Limited Service - Drive Thru/Detached Facility; 24 - Limited Service - Loan Production; 25 - Limited Service - Consumer Credit; 26 - Limited Service - Contractual; 27 - Limited Service - Messenger; 28 - Limited Service - Retail; 29 - Limited Service - Mobile/Seasonal; 30 - Limited Service - Trust; 99 - Limited Service - Other Office/Branch"#]
    #[serde(rename="SERVTYPE_DESC")]
    pub service_type_code_description: Option<String>,

    #[doc = r#"## FDIC Field:: `STALP`
    Title: State Alpha Code
    Description: The state abbreviation of the location of the institution's main office."#]
    #[serde(rename="STALP")]
    pub state_alpha_code: Option<String>,

    #[doc = r#"## FDIC Field:: `STCNTY`
    Title: State and County Number
    Description: A five digit number representing the state and county in which the institution is physically located.  The first two digits represent the FIPS state numeric code and the last three digits represent the FIPS county numeric code."#]
    #[serde(rename="STCNTY")]
    pub state_and_county_number: Option<String>,

    #[doc = r#"## FDIC Field:: `STNAME`
    Title: Branch State
    Description: State in which the  branch is physically located. The FDIC Act defines state as any State of the United States, the District of Columbia, and any territory of the United States, Puerto Rico, Guam, American Samoa, the Trust Territory of the Pacific Islands, the Virgin Island, and the Northern Mariana Islands."#]
    #[serde(rename="STNAME")]
    pub branch_state: Option<String>,

    #[doc = r#"## FDIC Field:: `UNINUM`
    Title: FDIC's Unique Number
    Description: FDIC's unique identifier number for holding companies, banks, branches and nondeposit subsidiaries."#]
    #[serde(rename="UNINUM")]
    pub fdic_s_unique_number_uninum: Option<String>,

    #[doc = r#"## FDIC Field:: `ZIP`
    Title: Zip Code
    Description: The first three, four, or five digits of the full postal zip code representing physical location of the institution or one of its branch offices."#]
    #[serde(rename="ZIP")]
    pub zip_code: Option<String>,

}

/// FDIC BankFind API `/locations` endpoint handler
/// Get Institution Locations
/// Returns locations/branches of financial institutions.
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter for the location search. All values must be entered in UPPERCASE.
 - `fields` (String, optional): Comma delimited list of fields to return. All values must be entered in UPPERCASE.
    Example: NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY
 - `sort_by` (String, optional): Field name by which to sort returned data. All values must be entered in UPPERCASE.
    Example: NAME
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
    path = "/locations",
    params(LocationsParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<LocationsProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Structure"
)]
pub async fn locations_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<LocationsParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "locations",
    )
    .await
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
            
            street_address: None,
            institution_class: None,
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
            city: None,
            county: None,
            combined_statistical_area_name: None,
            combined_statistical_area_flag_branch: None,
            combined_statistical_area_number_branch: None,
            branch_established_date: None,
            fdic_s_unique_number: None,
            location_address_latitude: None,
            location_address_latitude_longitude: None,
            minority_status_code: None,
            minority_status_description: None,
            main_office: None,
            institution_name: None,
            office_name: None,
            branch_number: None,
            run_date: None,
            service_type_code: None,
            service_type_code_description: None,
            state_alpha_code: None,
            state_and_county_number: None,
            branch_state: None,
            fdic_s_unique_number_uninum: None,
            zip_code: None,
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
