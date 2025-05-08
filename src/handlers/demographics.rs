//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/demographics` endpoint.

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

/// Auto-generated parameters struct for `/demographics` endpoint.
/// Spec: demographics_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
pub struct DemographicsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
}

// Implement FdicEndpoint for generic handler
impl FdicEndpoint for DemographicsParameters {
    fn name() -> &'static str {
        "demographics"
    }
}

// Implement QueryParameters for generic handler
impl QueryParameters for DemographicsParameters {
    const VALID_FIELDS: &'static [&'static str] = &[
        "ACTEVT",
        "BRANCH",
        "CALLYM",
        "CALLYMD",
        "CBSANAME",
        "CERT",
        "CLCODE",
        "CMSA",
        "CNTRYALP",
        "CNTRYNUM",
        "CNTYNUM",
        "CSA",
        "DIVISION",
        "DOCKET",
        "FDICAREA",
        "FDICTERR",
        "FLDOFDCA",
        "HCTNONE",
        "INSAGNT2",
        "METRO",
        "MICRO",
        "MNRTYCDE",
        "MNRTYDTE",
        "OAKAR",
        "OFFDMULT",
        "OFFNDOM",
        "OFFOTH",
        "OFFSOD",
        "OFFSTATE",
        "OFFTOT",
        "OFFUSOA",
        "OTSDIST",
        "OTSREGNO",
        "QTRNO",
        "REPDTE",
        "REPDTE_INT",
        "RISKTERR",
        "SASSER",
        "SIMS_LAT",
        "SIMS_LONG",
        "WEBADDR",
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
    ];

    #[allow(unused_variables)] // the `query` parameter is unused if there are no endpoint-specific parameters
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
        
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/demographics` endpoint.
/// Spec: demographics_properties.yaml
#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonSchema, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct DemographicsProperties {
    #[schemars(description = r#"STRUCTURE ACTIVITY EVENT CODE.  MERGER OR CLOSING CODES ONLY. - Structure activity event code. Merger or closing codes only."#)]
    pub actevt: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION HAS BRANCHES. 0 = UNIT BANK (NO BRANCHES). 1 = BRANCHES. - "#)]
    pub branch: Option<f32>,

    #[schemars(description = r#"REPRESENTS THE CALENDER DATE FOR WHICH THE FINANCIAL DATA WAS COLLECTED IN YEAR AND MONTH FORMAT (CCYYMM). - "#)]
    pub callym: Option<String>,

    #[schemars(description = r#"REPRESENTS THE CALANDER DATE FOR WHICH THE FINANCIAL DATA WAS COLLECTED IN YEAR, MONTH, AND DAY FORMAT (CCYYMMDD). - "#)]
    pub callymd: Option<String>,

    #[schemars(description = r#"THE U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES THE CORE BASED STATISTICAL AREA (CBSA).  IT IS A STATISTICAL GEOGRAPHIC ENTITY CONSISTING OF THE COUNTY OR COUNTIES ASSOCIATED WITH AT LEAST ONE CORE (URBANIZED AREA OR URBAN CLUSTER) OF AT LEAST 10,0000 POPULATION, PLUS ADJACENT COUNTIES HAVING A HIGH DEGREE OF SOCIAL AND ECONOMIC INTEGRATION WITH THE CORE AS MEASURED THROUGH COMMUTING TIES WITH THE COUNTIES CONTAINING THE  CORE. - "#)]
    pub cbsaname: Option<String>,

    #[schemars(description = r#"FDIC Certificate # - A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#)]
    pub cert: Option<f32>,

    #[schemars(description = r#"A TWO DIGIT NUMERIC CODE WHICH IDENTIFIES THE MAJOR AND MINOR CATAGORIES OF AN INSTITUTION - "#)]
    pub clcode: Option<f32>,

    #[schemars(description = r#"THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) CONSOLIDATED METROPOLITAN STATISTICAL AREA (CMSA) CODE IS A NUMBER REPRESENTING THE INSTITUTION LOCATION.  A CMSA CONSISTS OF TWO OR MORE CONTIGUOUS METROPOLITAN STATISTICAL AREAS (MSA) WITH A  COMBINED POPULATION OF OVER 1 MILLION - "#)]
    pub cmsa: Option<String>,

    #[schemars(description = r#"THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) ALPHABETIC CODE OF THE COUNTRY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED. - "#)]
    pub cntryalp: Option<String>,

    #[schemars(description = r#"THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) NUMERIC CODE  OF THE COUNTRY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED. - "#)]
    pub cntrynum: Option<String>,

    #[schemars(description = r#"THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) NUMERIC CODE  OF THE COUNTY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED. - "#)]
    pub cntynum: Option<String>,

    #[schemars(description = r#"THE U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES THE COMBINED STATISTICAL AREA (CSA).  A GEOGRAPHIC ENTITY CONSISTING OF TWO OR MORE ADJACENT CORE BASED STATISTICAL AREAS (CBSAS) WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15.PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 25 COMBINE AUTOMATICALLY.  PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15, BUT LESS THAN 25, MAY COMBINE IF LOCAL OPTION IN BOTH AREAS FAVOR COMBINATION. - "#)]
    pub csa: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A CBSA DIVISION 0 = INSTITUTION IS NOT IN A CBSA DIVISION; 1 = INSTITUTION IS IN A CBSA DIVISION FRB NUMBER. - "#)]
    pub division: Option<f32>,

    #[schemars(description = r#"A UNIQUE IDENTIFICATION NUMBER ASSIGNED TO INSTITUTIONS CHARTERED BY THE OFFICE OF THRIFT SUPERVISION OR THAT BECOME MEMBERS OF THE  FEDERAL HOME LOAN SYSTEM. - "#)]
    pub docket: Option<f32>,

    #[schemars(description = r#"A NUMBER USED TO IDENTIFY THE FDIC COMPLIANCE AREA IN WHICH AN  INSTITUTION IS LOCATED - "#)]
    pub fdicarea: Option<f32>,

    #[schemars(description = r#"AN ABBREVIATION OF THE CURRENT COMPLIANCE TERRITORY WHERE AN NSTITUTION IS LOCATED (FDIC COMPLIANCE TERRITORY).  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED). - "#)]
    pub fdicterr: Option<String>,

    #[schemars(description = r#"THE NAME OF THE COMPLIANCE FIELD OFFICE TO WHICH AN INSTITUTION IS ASSIGNED.  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED - "#)]
    pub fldofdca: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS AN INDEPENDENT BANK.  NOT A MEMBER OF A BANK HOLDING COMPANY.  0 = MEMBER OF A BANK HOLDING COMPANY OR 1 = NOT A MEMBER OF A BANK HOLDING COMPANY. - "#)]
    pub hctnone: Option<f32>,

    #[schemars(description = r#"THE SECONDARY INSURER, INSURANCE AGENT, OR INSURANCE STATUS OF AN INSTITUTION - "#)]
    pub insagnt2: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A METROPOLITAN STATISTICAL AREA THE U.S. CENSUS BUREAU OFFICE 0 = INSTITUTION IS NOT IN A METROPOLITAN STATISTICAL AREA AND 1 = INSTITUTION IS IN A METROPOLITAN STATISTICAL AREA. - "#)]
    pub metro: Option<f32>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A MICROPOLITAN STATISTICAL AREA.  THE U.S. CENSUS BUREAU OFFICE OF  MANANGEMENT AND BUDGET DEFINES THE MICROPOLITAN STATISTICAL AREA.  A CORE BASED STATISTICAL AREA ASSOCIATED WITH AT LEAST ONE URBAN  CLUSTER THAT HAS A POPULATION OF AT LEAST 10,000 BUT LESS THAN 50,000.  THE MICROPOLITAN STATISTICAL AREA COMPRISES THE CENTRAL COUNTY OR COUNTIES CONTAINING THE CORE, PLUS ADJACENT OUTLYING  COUNTIES HAVING A HIGH DEGREE OF SOCIAL AND ECONOMIC INTEGRATION WITH THE CENTRAL COUNTY AS MEASURED THROUGH COMMUTING. 0 = INSTITUTION IS NOT IN A MICROPOLITAN STATISTICAL AREA; 1 = INSTITUTION IS IN A MICROPOLITAN STATISTICAL AREA. - "#)]
    pub micro: Option<f32>,

    #[schemars(description = r#"A CHARACTER FIELD ON THE INSTITUTION FILE CORRESPONDING TO A TYPE OF MINORITY OWNERSHIP.  .  = NONE. ;  01 = AFRICAN AMERICAN; 02 = HISPANIC AMERICAN; 03 = ASIAN OR PACIFIC ISLANDER AMERICANS; 04 = NATIVE AMERICAN OR NATIVE ALASKAN AMERICAN; 05 = MULIT-RACIAL AMERICAN; 06 = MINORITY BOARD AND SERVING AFRICAN AMERICAN COMMUNITY; 08 = MINORITY BOARD AND SERVING ASIAN/PACIFIC ISLANDER AMERICANS; 10 = MINORITY BOARD AND SERVING MULTI-RACIAL COMMUNITY. - "#)]
    pub mnrtycde: Option<f32>,

    #[schemars(description = r#"REPRESENTS THE EFFECTIVE DATE ON WHICH AN INSTITUTION IS ASSIGNED A MINORITY STATUS.  TRANSACTION IN DATE9.  FORMAT (DDMONCCYY) DAY, MONTH ABBREV, CENTURY AND YEAR. - "#)]
    pub mnrtydte: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION ACQUIRED DEPOSITS  THAT WERE PREVIOUSLY INSURED UNDER A DIFFERENT INSURANCE FUND.  0 = HAS NO OAKAR DEPOSITS; 1 = HAS OAKAR DEPOSITS - "#)]
    pub oakar: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF MULTIPLE SERVICE DOMESTIC OFFICES OPERATED BY AN  INSTITUTION. - "#)]
    pub offdmult: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF NONDOMESTIC OFFICES OPERATED BY AN INSTITUTION. - "#)]
    pub offndom: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF DOMESTIC NON-MULTIPLE SERVICE OFFICES OPERATED BY  INSTITUTION. - "#)]
    pub offoth: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF OFFICES OPERATED BY AN INSTITUTION BASED ON THE SUMMARY OF DEPOSITS DEFINITION OF OFFICES. - "#)]
    pub offsod: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF STATES WITH OFFICES (INCLUDING ITS MAIN OFFICE). - "#)]
    pub offstate: Option<f32>,

    #[schemars(description = r#"THE TOTAL NUMBER OF OFFICES OPERATED BY AN INSTITUTION. - "#)]
    pub offtot: Option<f32>,

    #[schemars(description = r#"THE NUMBER OF DOMESTIC AND U.S. TERRITORIES OFFICES OPERATED BY AN INSTITUTION. - "#)]
    pub offusoa: Option<f32>,

    #[schemars(description = r#"A NUMBER USED TO IDENTIFY THE OFFICE OF THRIFT SUPERVISION DISTRICT IN WHICH THE INSTITUTION IS LOCATED. 01 = NORTHEAST;  02 = SOUTHEAST; 04 = MIDWEST; 05 = WEST - "#)]
    pub otsdist: Option<f32>,

    #[schemars(description = r#"A NUMBER USED TO IDENTIFY THE OFFICE OF THRIFT SUPERVISION REGION IN WHICH THE INSTITUTION IS LOCATED.  1 = NORTHEAST;  2 = SOUTHEAST; 4 = MIDWEST; 5 = WEST - "#)]
    pub otsregno: Option<f32>,

    #[schemars(description = r#"IDENTIFIES THE CALENDAR QUARTER.  1 = MARCH; 2 = JUNE; 3 = SEPTEMBER; 4 = DECEMBER. - "#)]
    pub qtrno: Option<f32>,

    #[schemars(description = r#"Report Date (Search-Eligible) - The last day of the financial reporting period selected. This field can be used for search and filtering."#)]
    pub repdte: Option<String>,

    #[schemars(description = r#"Report Date Integer (Search-Eligible) - The last day of the financial reporting period selected. This field can be used for search and filtering."#)]
    pub repdte_int: Option<String>,

    #[schemars(description = r#"AN ABBREVIATION OF THE CURRENT RISK TERRITORY FOR AN INSTITUTION (FDIC RISK TERRITORY).  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED). - "#)]
    pub riskterr: Option<String>,

    #[schemars(description = r#"A FLAG USED TO INDICATE WHETHER AN INSTITUTION WAS A FORMER SAVINGS ASSOCIATION THAT HAS CONVERTED TO A BANK CHARTER AND IS STILL A SAIF INSURED INSTITUTION.  0 = NOT A SASSER INSTITUTION; 1 = IS A SASSER INSTITUTION. - "#)]
    pub sasser: Option<f32>,

    #[schemars(description = r#"GEOGRAPHIC LATITUDE OF MAIN OFFICE. - "#)]
    pub sims_lat: Option<f32>,

    #[schemars(description = r#"GEOGRAPHIC LONGITUDE OF MAIN OFFICE. - "#)]
    pub sims_long: Option<f32>,

    #[schemars(description = r#"Primary Internet Web Address - The primary internet web address is the public internet site obtained from the most recent FFIEC Call Report (CALL) for commercial banks or from the supplemental information for Thrift Financial Reporters (TFR). The primary internet web address is included only for those institutions reporting an address on the most recent FFIEC Call Report or Thrift Financial Report.  This information resides in the most recent demographic information file. For some institutions users will find that for the item Primary Internet Web Address: the caption will read 'Web site not available'.  Possible reasons that a Web site may not be available are: The institution failed to file on the most recent call report or TFR. The institution filed a primary Internet Web address on its most recent FFIEC Call Report; however, the address filed by the institution was not in accordance with the instructions provided by the FFIEC on how to file a primary Internet Web address or FDIC attempts to validate and access the site were unsuccessful. Users may also experience instances where the URL provided for primary Internet Web address in ID returns an error stating that the site is not found. Possible reasons for such occurrences are: The institution?s reported primary Web address was valid as of the date that the demographic information was updated in ID, but is no longer valid. The institution?s reported Internet Web address is valid, but the institution?s Web site was inoperable at the time that the user attempted to access it due to technical problems being experienced by the institution?s Web site, the institution?s web provider, the user?s Web provider, or other issues not related to the validity of the Web address.  Users are advised to contact the institution on any questions regarding the services provided by the institution. For questions involving the reporting of primary Internet Web address by those institutions that file a FFIEC Call report, users are advised to contact supervision@fdic.gov.  For questions involving the primary Internet Web address of institutions that file a Thrift Financial Report, users are advised to contact pamela.schaar@ots.treas.gov or call Ms. Schaar at (202) 906-7205. Disclaimer: The Primary Internet Web Addresses listed have been reported to the FDIC by each institution. The hyperlinks to institution Internet sites are provided solely as a convenience to users of the FDIC Internet site. The FDIC has made a limited effort to determine that these links function properly. However, linked sites are not under the control of FDIC, and FDIC is not responsible for the contents of any linked site, or any link contained in a linked site.  Even if you access an institution?s site by means of the link provided by FDIC, you are responsible for confirming the identity and authenticity of any institution you visit and transact business with online. The inclusion of a link does not imply or constitute an endorsement by FDIC of the institution, its ownership or management, the products or services it offers, or any advertisers or sponsors appearing on the institution?s web site."#)]
    pub webaddr: Option<String>,

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

}

#[derive(Clone,Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DemographicsResponse {
    pub data: Vec<serde_json::Value>,
    pub meta: ResponseMeta,
    pub totals: ResponseTotals,
}

impl IntoContents for DemographicsResponse {
    fn into_contents(self) -> Vec<Content> {
        // Convert the response into a Vec<Content> as expected by MCP
        // Panics only if serialization fails, which should be impossible for valid structs
        vec![Content::json(self).expect("Failed to serialize DemographicsResponse to Content")]
    }
}

/// FDIC BankFind API `/demographics` endpoint handler
/// Get Summary of Demographic Information
/// Returns summary of demographic information
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[doc = r#" - `api_key` (String, optional): Api key used for api.fdic.gov - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
CERT:14 AND REPDTE:20230630 - `format` (String, optional): The format of the data to return.
json - `download` (bool, optional): Whether the data should be downloaded as a file. - `filename` (String, optional): The filename to use when downloading data.
data_file"#]
#[doc = r#"Verb: GET
Path: /demographics
Parameters: DemographicsParameters
Responses:
    200: Successful Operation
    400: Bad input parameter
    500: Internal Server Error
    502: Bad Gateway
    503: Service Unavailable
    504: Gateway Timeout
Tag: Demographics"#]
pub async fn demographics_handler(config: &FdicApiConfig, params: &DemographicsParameters) -> Result<CallToolResult, rmcp::Error> {
    // Log incoming request parameters and request details as structured JSON
    info!(
        target = "handler",
        event = "incoming_request",
        endpoint = "demographics",
        method = "GET",
        path = "/demographics",
        params = serde_json::to_string(params).unwrap()
    );

    let resp = get_fdic_bank_find_mcp_response::<_, DemographicsResponse>(config, params).await;

    // Log outgoing FDIC API request as structured JSON
    resp.and_then(|r| r.into_call_tool_result())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    #[test]
    fn test_parameters_struct_serialization() {
        let params = DemographicsParameters {
            common: CommonParameters::default(),
            
        };
        let _ = serde_json::to_string(&params).unwrap();
    }

    #[test]
    fn test_properties_struct_serialization() {
        let props = DemographicsProperties {
            actevt: None,
            branch: None,
            callym: None,
            callymd: None,
            cbsaname: None,
            cert: None,
            clcode: None,
            cmsa: None,
            cntryalp: None,
            cntrynum: None,
            cntynum: None,
            csa: None,
            division: None,
            docket: None,
            fdicarea: None,
            fdicterr: None,
            fldofdca: None,
            hctnone: None,
            insagnt2: None,
            metro: None,
            micro: None,
            mnrtycde: None,
            mnrtydte: None,
            oakar: None,
            offdmult: None,
            offndom: None,
            offoth: None,
            offsod: None,
            offstate: None,
            offtot: None,
            offusoa: None,
            otsdist: None,
            otsregno: None,
            qtrno: None,
            repdte: None,
            repdte_int: None,
            riskterr: None,
            sasser: None,
            sims_lat: None,
            sims_long: None,
            webaddr: None,
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
            };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
