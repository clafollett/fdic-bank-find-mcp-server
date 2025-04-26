//! Do not edit by hand.
//! Auto-generated handler for FDIC BankFind API `/demographics` endpoint.// Internal imports (std, crate)
use std::collections::HashMap;
use crate::config::FDICApiConfig;
use crate::common::{list_endpoint, CommonParameters, QueryParameters};
use crate::fdic_response::FDICResponse;

// External imports (alphabetized)
use axum::{extract::{Query, State}, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Auto-generated parameters struct for `/demographics` endpoint.
/// Spec: demographics_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct DemographicsParameters {
    /// Shared FDIC query parameters
    #[serde(flatten)]
    pub common: CommonParameters,
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

    #[allow(unused_variables)]
    fn insert_endpoint_specific(&self, query: &mut HashMap<String, String>) {
    }

    fn common_mut(&mut self) -> &mut CommonParameters {
        &mut self.common
    }
}

/// Auto-generated properties struct for `/demographics` endpoint.
/// Spec: demographics_properties.yaml
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct DemographicsProperties {
    #[doc = r#"## FDIC Field:: `ACTEVT`
    Title: STRUCTURE ACTIVITY EVENT CODE.  MERGER OR CLOSING CODES ONLY.
    Description: Structure activity event code. Merger or closing codes only."#]
    #[serde(rename="ACTEVT")]
    pub structure_activity_event_code_merger_or_closing_codes_only: Option<String>,

    #[doc = r#"## FDIC Field:: `BRANCH`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION HAS BRANCHES. 0 = UNIT BANK (NO BRANCHES). 1 = BRANCHES.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="BRANCH")]
    pub a_flag_used_to_indicate_whether_an_institution_has_branches_0_unit_bank_no_branches_1_branches: Option<f64>,

    #[doc = r#"## FDIC Field:: `CALLYM`
    Title: REPRESENTS THE CALENDER DATE FOR WHICH THE FINANCIAL DATA WAS COLLECTED IN YEAR AND MONTH FORMAT (CCYYMM).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CALLYM")]
    pub represents_the_calender_date_for_which_the_financial_data_was_collected_in_year_and_month_format_ccyymm: Option<String>,

    #[doc = r#"## FDIC Field:: `CALLYMD`
    Title: REPRESENTS THE CALANDER DATE FOR WHICH THE FINANCIAL DATA WAS COLLECTED IN YEAR, MONTH, AND DAY FORMAT (CCYYMMDD).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CALLYMD")]
    pub represents_the_calander_date_for_which_the_financial_data_was_collected_in_year_month_and_day_format_ccyymmdd: Option<String>,

    #[doc = r#"## FDIC Field:: `CBSANAME`
    Title: THE U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES THE CORE BASED STATISTICAL AREA (CBSA).  IT IS A STATISTICAL GEOGRAPHIC ENTITY CONSISTING OF THE COUNTY OR COUNTIES ASSOCIATED WITH AT LEAST ONE CORE (URBANIZED AREA OR URBAN CLUSTER) OF AT LEAST 10,0000 POPULATION, PLUS ADJACENT COUNTIES HAVING A HIGH DEGREE OF SOCIAL AND ECONOMIC INTEGRATION WITH THE CORE AS MEASURED THROUGH COMMUTING TIES WITH THE COUNTIES CONTAINING THE  CORE.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CBSANAME")]
    pub the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_core_based_statistical_area_cbsa_it_is_a_statistical_geographic_entity_consisting_of_the_county_or_counties_associated_with_at_least_one_core_urbanized_area_or_urban_cluster_of_at_least_10_0000_population_plus_adjacent_counties_having_a_high_degree_of_social_and_economic_integration_with_the_core_as_measured_through_commuting_ties_with_the_counties_containing_the_core: Option<String>,

    #[doc = r#"## FDIC Field:: `CERT`
    Title: FDIC Certificate #
    Description: A unique NUMBER assigned by the FDIC used to identify institutions and for the issuance of insurance certificates."#]
    #[serde(rename="CERT")]
    pub fdic_certificate: Option<f64>,

    #[doc = r#"## FDIC Field:: `CLCODE`
    Title: A TWO DIGIT NUMERIC CODE WHICH IDENTIFIES THE MAJOR AND MINOR CATAGORIES OF AN INSTITUTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CLCODE")]
    pub a_two_digit_numeric_code_which_identifies_the_major_and_minor_catagories_of_an_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `CMSA`
    Title: THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) CONSOLIDATED METROPOLITAN STATISTICAL AREA (CMSA) CODE IS A NUMBER REPRESENTING THE INSTITUTION LOCATION.  A CMSA CONSISTS OF TWO OR MORE CONTIGUOUS METROPOLITAN STATISTICAL AREAS (MSA) WITH A  COMBINED POPULATION OF OVER 1 MILLION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CMSA")]
    pub the_federal_information_processing_standards_fips_consolidated_metropolitan_statistical_area_cmsa_code_is_a_number_representing_the_institution_location_a_cmsa_consists_of_two_or_more_contiguous_metropolitan_statistical_areas_msa_with_a_combined_population_of_over_1_million: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTRYALP`
    Title: THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) ALPHABETIC CODE OF THE COUNTRY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CNTRYALP")]
    pub the_federal_information_processing_standards_fips_alphabetic_code_of_the_country_in_which_the_institution_is_physically_located: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTRYNUM`
    Title: THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) NUMERIC CODE  OF THE COUNTRY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CNTRYNUM")]
    pub the_federal_information_processing_standards_fips_numeric_code_of_the_country_in_which_the_institution_is_physically_located: Option<String>,

    #[doc = r#"## FDIC Field:: `CNTYNUM`
    Title: THE FEDERAL INFORMATION PROCESSING STANDARDS (FIPS) NUMERIC CODE  OF THE COUNTY IN WHICH THE INSTITUTION IS PHYSICALLY LOCATED.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CNTYNUM")]
    pub the_federal_information_processing_standards_fips_numeric_code_of_the_county_in_which_the_institution_is_physically_located: Option<String>,

    #[doc = r#"## FDIC Field:: `CSA`
    Title: THE U.S. CENSUS BUREAU OFFICE OF MANANGEMENT AND BUDGET DEFINES THE COMBINED STATISTICAL AREA (CSA).  A GEOGRAPHIC ENTITY CONSISTING OF TWO OR MORE ADJACENT CORE BASED STATISTICAL AREAS (CBSAS) WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15.PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 25 COMBINE AUTOMATICALLY.  PAIRS OF CBSAS WITH EMPLOYMENT INTERCHANGE MEASURES OF AT LEAST 15, BUT LESS THAN 25, MAY COMBINE IF LOCAL OPTION IN BOTH AREAS FAVOR COMBINATION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="CSA")]
    pub the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_combined_statistical_area_csa_a_geographic_entity_consisting_of_two_or_more_adjacent_core_based_statistical_areas_cbsas_with_employment_interchange_measures_of_at_least_15_pairs_of_cbsas_with_employment_interchange_measures_of_at_least_25_combine_automatically_pairs_of_cbsas_with_employment_interchange_measures_of_at_least_15_but_less_than_25_may_combine_if_local_option_in_both_areas_favor_combination: Option<String>,

    #[doc = r#"## FDIC Field:: `DIVISION`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A CBSA DIVISION 0 = INSTITUTION IS NOT IN A CBSA DIVISION; 1 = INSTITUTION IS IN A CBSA DIVISION FRB NUMBER.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DIVISION")]
    pub a_flag_used_to_indicate_whether_an_institution_is_in_a_cbsa_division_0_institution_is_not_in_a_cbsa_division_1_institution_is_in_a_cbsa_division_frb_number: Option<f64>,

    #[doc = r#"## FDIC Field:: `DOCKET`
    Title: A UNIQUE IDENTIFICATION NUMBER ASSIGNED TO INSTITUTIONS CHARTERED BY THE OFFICE OF THRIFT SUPERVISION OR THAT BECOME MEMBERS OF THE  FEDERAL HOME LOAN SYSTEM.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="DOCKET")]
    pub a_unique_identification_number_assigned_to_institutions_chartered_by_the_office_of_thrift_supervision_or_that_become_members_of_the_federal_home_loan_system: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICAREA`
    Title: A NUMBER USED TO IDENTIFY THE FDIC COMPLIANCE AREA IN WHICH AN  INSTITUTION IS LOCATED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICAREA")]
    pub a_number_used_to_identify_the_fdic_compliance_area_in_which_an_institution_is_located: Option<f64>,

    #[doc = r#"## FDIC Field:: `FDICTERR`
    Title: AN ABBREVIATION OF THE CURRENT COMPLIANCE TERRITORY WHERE AN NSTITUTION IS LOCATED (FDIC COMPLIANCE TERRITORY).  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FDICTERR")]
    pub an_abbreviation_of_the_current_compliance_territory_where_an_nstitution_is_located_fdic_compliance_territory_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: Option<String>,

    #[doc = r#"## FDIC Field:: `FLDOFDCA`
    Title: THE NAME OF THE COMPLIANCE FIELD OFFICE TO WHICH AN INSTITUTION IS ASSIGNED.  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="FLDOFDCA")]
    pub the_name_of_the_compliance_field_office_to_which_an_institution_is_assigned_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: Option<String>,

    #[doc = r#"## FDIC Field:: `HCTNONE`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS AN INDEPENDENT BANK.  NOT A MEMBER OF A BANK HOLDING COMPANY.  0 = MEMBER OF A BANK HOLDING COMPANY OR 1 = NOT A MEMBER OF A BANK HOLDING COMPANY.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="HCTNONE")]
    pub a_flag_used_to_indicate_whether_an_institution_is_an_independent_bank_not_a_member_of_a_bank_holding_company_0_member_of_a_bank_holding_company_or_1_not_a_member_of_a_bank_holding_company: Option<f64>,

    #[doc = r#"## FDIC Field:: `INSAGNT2`
    Title: THE SECONDARY INSURER, INSURANCE AGENT, OR INSURANCE STATUS OF AN INSTITUTION
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="INSAGNT2")]
    pub the_secondary_insurer_insurance_agent_or_insurance_status_of_an_institution: Option<String>,

    #[doc = r#"## FDIC Field:: `METRO`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A METROPOLITAN STATISTICAL AREA THE U.S. CENSUS BUREAU OFFICE 0 = INSTITUTION IS NOT IN A METROPOLITAN STATISTICAL AREA AND 1 = INSTITUTION IS IN A METROPOLITAN STATISTICAL AREA.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="METRO")]
    pub a_flag_used_to_indicate_whether_an_institution_is_in_a_metropolitan_statistical_area_the_u_s_census_bureau_office_0_institution_is_not_in_a_metropolitan_statistical_area_and_1_institution_is_in_a_metropolitan_statistical_area: Option<f64>,

    #[doc = r#"## FDIC Field:: `MICRO`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION IS IN A MICROPOLITAN STATISTICAL AREA.  THE U.S. CENSUS BUREAU OFFICE OF  MANANGEMENT AND BUDGET DEFINES THE MICROPOLITAN STATISTICAL AREA.  A CORE BASED STATISTICAL AREA ASSOCIATED WITH AT LEAST ONE URBAN  CLUSTER THAT HAS A POPULATION OF AT LEAST 10,000 BUT LESS THAN 50,000.  THE MICROPOLITAN STATISTICAL AREA COMPRISES THE CENTRAL COUNTY OR COUNTIES CONTAINING THE CORE, PLUS ADJACENT OUTLYING  COUNTIES HAVING A HIGH DEGREE OF SOCIAL AND ECONOMIC INTEGRATION WITH THE CENTRAL COUNTY AS MEASURED THROUGH COMMUTING. 0 = INSTITUTION IS NOT IN A MICROPOLITAN STATISTICAL AREA; 1 = INSTITUTION IS IN A MICROPOLITAN STATISTICAL AREA.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MICRO")]
    pub a_flag_used_to_indicate_whether_an_institution_is_in_a_micropolitan_statistical_area_the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_micropolitan_statistical_area_a_core_based_statistical_area_associated_with_at_least_one_urban_cluster_that_has_a_population_of_at_least_10_000_but_less_than_50_000_the_micropolitan_statistical_area_comprises_the_central_county_or_counties_containing_the_core_plus_adjacent_outlying_counties_having_a_high_degree_of_social_and_economic_integration_with_the_central_county_as_measured_through_commuting_0_institution_is_not_in_a_micropolitan_statistical_area_1_institution_is_in_a_micropolitan_statistical_area: Option<f64>,

    #[doc = r#"## FDIC Field:: `MNRTYCDE`
    Title: A CHARACTER FIELD ON THE INSTITUTION FILE CORRESPONDING TO A TYPE OF MINORITY OWNERSHIP.  .  = NONE. ;  01 = AFRICAN AMERICAN; 02 = HISPANIC AMERICAN; 03 = ASIAN OR PACIFIC ISLANDER AMERICANS; 04 = NATIVE AMERICAN OR NATIVE ALASKAN AMERICAN; 05 = MULIT-RACIAL AMERICAN; 06 = MINORITY BOARD AND SERVING AFRICAN AMERICAN COMMUNITY; 08 = MINORITY BOARD AND SERVING ASIAN/PACIFIC ISLANDER AMERICANS; 10 = MINORITY BOARD AND SERVING MULTI-RACIAL COMMUNITY.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MNRTYCDE")]
    pub a_character_field_on_the_institution_file_corresponding_to_a_type_of_minority_ownership_none_01_african_american_02_hispanic_american_03_asian_or_pacific_islander_americans_04_native_american_or_native_alaskan_american_05_mulit_racial_american_06_minority_board_and_serving_african_american_community_08_minority_board_and_serving_asian_pacific_islander_americans_10_minority_board_and_serving_multi_racial_community: Option<f64>,

    #[doc = r#"## FDIC Field:: `MNRTYDTE`
    Title: REPRESENTS THE EFFECTIVE DATE ON WHICH AN INSTITUTION IS ASSIGNED A MINORITY STATUS.  TRANSACTION IN DATE9.  FORMAT (DDMONCCYY) DAY, MONTH ABBREV, CENTURY AND YEAR.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="MNRTYDTE")]
    pub represents_the_effective_date_on_which_an_institution_is_assigned_a_minority_status_transaction_in_date9_format_ddmonccyy_day_month_abbrev_century_and_year: Option<String>,

    #[doc = r#"## FDIC Field:: `OAKAR`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION ACQUIRED DEPOSITS  THAT WERE PREVIOUSLY INSURED UNDER A DIFFERENT INSURANCE FUND.  0 = HAS NO OAKAR DEPOSITS; 1 = HAS OAKAR DEPOSITS
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OAKAR")]
    pub a_flag_used_to_indicate_whether_an_institution_acquired_deposits_that_were_previously_insured_under_a_different_insurance_fund_0_has_no_oakar_deposits_1_has_oakar_deposits: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFDMULT`
    Title: THE NUMBER OF MULTIPLE SERVICE DOMESTIC OFFICES OPERATED BY AN  INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFDMULT")]
    pub the_number_of_multiple_service_domestic_offices_operated_by_an_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFNDOM`
    Title: THE NUMBER OF NONDOMESTIC OFFICES OPERATED BY AN INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFNDOM")]
    pub the_number_of_nondomestic_offices_operated_by_an_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFOTH`
    Title: THE NUMBER OF DOMESTIC NON-MULTIPLE SERVICE OFFICES OPERATED BY  INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFOTH")]
    pub the_number_of_domestic_non_multiple_service_offices_operated_by_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFSOD`
    Title: THE NUMBER OF OFFICES OPERATED BY AN INSTITUTION BASED ON THE SUMMARY OF DEPOSITS DEFINITION OF OFFICES.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFSOD")]
    pub the_number_of_offices_operated_by_an_institution_based_on_the_summary_of_deposits_definition_of_offices: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFSTATE`
    Title: THE NUMBER OF STATES WITH OFFICES (INCLUDING ITS MAIN OFFICE).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFSTATE")]
    pub the_number_of_states_with_offices_including_its_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFTOT`
    Title: THE TOTAL NUMBER OF OFFICES OPERATED BY AN INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFTOT")]
    pub the_total_number_of_offices_operated_by_an_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `OFFUSOA`
    Title: THE NUMBER OF DOMESTIC AND U.S. TERRITORIES OFFICES OPERATED BY AN INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OFFUSOA")]
    pub the_number_of_domestic_and_u_s_territories_offices_operated_by_an_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSDIST`
    Title: A NUMBER USED TO IDENTIFY THE OFFICE OF THRIFT SUPERVISION DISTRICT IN WHICH THE INSTITUTION IS LOCATED. 01 = NORTHEAST;  02 = SOUTHEAST; 04 = MIDWEST; 05 = WEST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTSDIST")]
    pub a_number_used_to_identify_the_office_of_thrift_supervision_district_in_which_the_institution_is_located_01_northeast_02_southeast_04_midwest_05_west: Option<f64>,

    #[doc = r#"## FDIC Field:: `OTSREGNO`
    Title: A NUMBER USED TO IDENTIFY THE OFFICE OF THRIFT SUPERVISION REGION IN WHICH THE INSTITUTION IS LOCATED.  1 = NORTHEAST;  2 = SOUTHEAST; 4 = MIDWEST; 5 = WEST
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="OTSREGNO")]
    pub a_number_used_to_identify_the_office_of_thrift_supervision_region_in_which_the_institution_is_located_1_northeast_2_southeast_4_midwest_5_west: Option<f64>,

    #[doc = r#"## FDIC Field:: `QTRNO`
    Title: IDENTIFIES THE CALENDAR QUARTER.  1 = MARCH; 2 = JUNE; 3 = SEPTEMBER; 4 = DECEMBER.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="QTRNO")]
    pub identifies_the_calendar_quarter_1_march_2_june_3_september_4_december: Option<f64>,

    #[doc = r#"## FDIC Field:: `REPDTE`
    Title: Report Date
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="REPDTE")]
    pub report_date: Option<String>,

    #[doc = r#"## FDIC Field:: `REPDTE_INT`
    Title: Report Date Integer
    Description: The last day of the financial reporting period selected."#]
    #[serde(rename="REPDTE_INT")]
    pub report_date_integer: Option<String>,

    #[doc = r#"## FDIC Field:: `RISKTERR`
    Title: AN ABBREVIATION OF THE CURRENT RISK TERRITORY FOR AN INSTITUTION (FDIC RISK TERRITORY).  ALL PERIODS ARE DISPLAYED IN THE CURRENT PERSPECTIVE (EXCEPTIONS CAN EXIST DEPENDING ON WHEN A QUARTER IS UPDATED).
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="RISKTERR")]
    pub an_abbreviation_of_the_current_risk_territory_for_an_institution_fdic_risk_territory_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: Option<String>,

    #[doc = r#"## FDIC Field:: `SASSER`
    Title: A FLAG USED TO INDICATE WHETHER AN INSTITUTION WAS A FORMER SAVINGS ASSOCIATION THAT HAS CONVERTED TO A BANK CHARTER AND IS STILL A SAIF INSURED INSTITUTION.  0 = NOT A SASSER INSTITUTION; 1 = IS A SASSER INSTITUTION.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SASSER")]
    pub a_flag_used_to_indicate_whether_an_institution_was_a_former_savings_association_that_has_converted_to_a_bank_charter_and_is_still_a_saif_insured_institution_0_not_a_sasser_institution_1_is_a_sasser_institution: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_LAT`
    Title: GEOGRAPHIC LATITUDE OF MAIN OFFICE.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SIMS_LAT")]
    pub geographic_latitude_of_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `SIMS_LONG`
    Title: GEOGRAPHIC LONGITUDE OF MAIN OFFICE.
    Description: MISSING DESCRIPTION"#]
    #[serde(rename="SIMS_LONG")]
    pub geographic_longitude_of_main_office: Option<f64>,

    #[doc = r#"## FDIC Field:: `WEBADDR`
    Title: Primary Internet Web Address
    Description: The primary internet web address is the public internet site obtained from the most recent FFIEC Call Report (CALL) for commercial banks or from the supplemental information for Thrift Financial Reporters (TFR). The primary internet web address is included only for those institutions reporting an address on the most recent FFIEC Call Report or Thrift Financial Report.  This information resides in the most recent demographic information file. For some institutions users will find that for the item Primary Internet Web Address: the caption will read 'Web site not available'.  Possible reasons that a Web site may not be available are: The institution failed to file on the most recent call report or TFR. The institution filed a primary Internet Web address on its most recent FFIEC Call Report; however, the address filed by the institution was not in accordance with the instructions provided by the FFIEC on how to file a primary Internet Web address or FDIC attempts to validate and access the site were unsuccessful. Users may also experience instances where the URL provided for primary Internet Web address in ID returns an error stating that the site is not found. Possible reasons for such occurrences are: The institution?s reported primary Web address was valid as of the date that the demographic information was updated in ID, but is no longer valid. The institution?s reported Internet Web address is valid, but the institution?s Web site was inoperable at the time that the user attempted to access it due to technical problems being experienced by the institution?s Web site, the institution?s web provider, the user?s Web provider, or other issues not related to the validity of the Web address.  Users are advised to contact the institution on any questions regarding the services provided by the institution. For questions involving the reporting of primary Internet Web address by those institutions that file a FFIEC Call report, users are advised to contact supervision@fdic.gov.  For questions involving the primary Internet Web address of institutions that file a Thrift Financial Report, users are advised to contact pamela.schaar@ots.treas.gov or call Ms. Schaar at (202) 906-7205. Disclaimer: The Primary Internet Web Addresses listed have been reported to the FDIC by each institution. The hyperlinks to institution Internet sites are provided solely as a convenience to users of the FDIC Internet site. The FDIC has made a limited effort to determine that these links function properly. However, linked sites are not under the control of FDIC, and FDIC is not responsible for the contents of any linked site, or any link contained in a linked site.  Even if you access an institution?s site by means of the link provided by FDIC, you are responsible for confirming the identity and authenticity of any institution you visit and transact business with online. The inclusion of a link does not imply or constitute an endorsement by FDIC of the institution, its ownership or management, the products or services it offers, or any advertisers or sponsors appearing on the institution?s web site."#]
    #[serde(rename="WEBADDR")]
    pub primary_internet_web_address: Option<String>,

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

}

/// FDIC BankFind API `/demographics` endpoint handler
/// Get Summary of Demographic Information
/// Returns summary of demographic information
/// **All string parameter values (except `api_key` and `filename`) are uppercased before proxying.**
#[allow(dead_code)]
#[doc = r#"## Query Parameters
 - `api_key` (String, optional): Api key used for api.fdic.gov
 - `filters` (String, optional): The filter criteria that refines the records included in the result. All values must be entered in UPPERCASE.
    Example: CERT:14 AND REPDTE:20230630
 - `format` (String, optional): The format of the data to return.
    Example: json
 - `download` (bool, optional): Whether the data should be downloaded as a file.
 - `filename` (String, optional): The filename to use when downloading data.
    Example: data_file
"#]
#[utoipa::path(
    get,
    path = "/demographics",
    params(DemographicsParameters),
    responses(
        (status = 200, description = "Successful Operation", body = FDICResponse<DemographicsProperties>) ,
        (status = 400, description = "Bad input parameter"),
        (status = 500, description = "Internal Server Error"),
        (status = 502, description = "Bad Gateway"),
        (status = 503, description = "Service Unavailable"),
        (status = 504, description = "Gateway Timeout"),
    ),
    tag = "Demographics"
)]
pub async fn demographics_handler(
    State(config): State<FDICApiConfig>,
    Query(params): Query<DemographicsParameters>,
) -> Response {
    list_endpoint(
        State(config),
        Query(params),
        "demographics",
    )
    .await
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
            
            structure_activity_event_code_merger_or_closing_codes_only: None,
            a_flag_used_to_indicate_whether_an_institution_has_branches_0_unit_bank_no_branches_1_branches: None,
            represents_the_calender_date_for_which_the_financial_data_was_collected_in_year_and_month_format_ccyymm: None,
            represents_the_calander_date_for_which_the_financial_data_was_collected_in_year_month_and_day_format_ccyymmdd: None,
            the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_core_based_statistical_area_cbsa_it_is_a_statistical_geographic_entity_consisting_of_the_county_or_counties_associated_with_at_least_one_core_urbanized_area_or_urban_cluster_of_at_least_10_0000_population_plus_adjacent_counties_having_a_high_degree_of_social_and_economic_integration_with_the_core_as_measured_through_commuting_ties_with_the_counties_containing_the_core: None,
            fdic_certificate: None,
            a_two_digit_numeric_code_which_identifies_the_major_and_minor_catagories_of_an_institution: None,
            the_federal_information_processing_standards_fips_consolidated_metropolitan_statistical_area_cmsa_code_is_a_number_representing_the_institution_location_a_cmsa_consists_of_two_or_more_contiguous_metropolitan_statistical_areas_msa_with_a_combined_population_of_over_1_million: None,
            the_federal_information_processing_standards_fips_alphabetic_code_of_the_country_in_which_the_institution_is_physically_located: None,
            the_federal_information_processing_standards_fips_numeric_code_of_the_country_in_which_the_institution_is_physically_located: None,
            the_federal_information_processing_standards_fips_numeric_code_of_the_county_in_which_the_institution_is_physically_located: None,
            the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_combined_statistical_area_csa_a_geographic_entity_consisting_of_two_or_more_adjacent_core_based_statistical_areas_cbsas_with_employment_interchange_measures_of_at_least_15_pairs_of_cbsas_with_employment_interchange_measures_of_at_least_25_combine_automatically_pairs_of_cbsas_with_employment_interchange_measures_of_at_least_15_but_less_than_25_may_combine_if_local_option_in_both_areas_favor_combination: None,
            a_flag_used_to_indicate_whether_an_institution_is_in_a_cbsa_division_0_institution_is_not_in_a_cbsa_division_1_institution_is_in_a_cbsa_division_frb_number: None,
            a_unique_identification_number_assigned_to_institutions_chartered_by_the_office_of_thrift_supervision_or_that_become_members_of_the_federal_home_loan_system: None,
            a_number_used_to_identify_the_fdic_compliance_area_in_which_an_institution_is_located: None,
            an_abbreviation_of_the_current_compliance_territory_where_an_nstitution_is_located_fdic_compliance_territory_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: None,
            the_name_of_the_compliance_field_office_to_which_an_institution_is_assigned_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: None,
            a_flag_used_to_indicate_whether_an_institution_is_an_independent_bank_not_a_member_of_a_bank_holding_company_0_member_of_a_bank_holding_company_or_1_not_a_member_of_a_bank_holding_company: None,
            the_secondary_insurer_insurance_agent_or_insurance_status_of_an_institution: None,
            a_flag_used_to_indicate_whether_an_institution_is_in_a_metropolitan_statistical_area_the_u_s_census_bureau_office_0_institution_is_not_in_a_metropolitan_statistical_area_and_1_institution_is_in_a_metropolitan_statistical_area: None,
            a_flag_used_to_indicate_whether_an_institution_is_in_a_micropolitan_statistical_area_the_u_s_census_bureau_office_of_manangement_and_budget_defines_the_micropolitan_statistical_area_a_core_based_statistical_area_associated_with_at_least_one_urban_cluster_that_has_a_population_of_at_least_10_000_but_less_than_50_000_the_micropolitan_statistical_area_comprises_the_central_county_or_counties_containing_the_core_plus_adjacent_outlying_counties_having_a_high_degree_of_social_and_economic_integration_with_the_central_county_as_measured_through_commuting_0_institution_is_not_in_a_micropolitan_statistical_area_1_institution_is_in_a_micropolitan_statistical_area: None,
            a_character_field_on_the_institution_file_corresponding_to_a_type_of_minority_ownership_none_01_african_american_02_hispanic_american_03_asian_or_pacific_islander_americans_04_native_american_or_native_alaskan_american_05_mulit_racial_american_06_minority_board_and_serving_african_american_community_08_minority_board_and_serving_asian_pacific_islander_americans_10_minority_board_and_serving_multi_racial_community: None,
            represents_the_effective_date_on_which_an_institution_is_assigned_a_minority_status_transaction_in_date9_format_ddmonccyy_day_month_abbrev_century_and_year: None,
            a_flag_used_to_indicate_whether_an_institution_acquired_deposits_that_were_previously_insured_under_a_different_insurance_fund_0_has_no_oakar_deposits_1_has_oakar_deposits: None,
            the_number_of_multiple_service_domestic_offices_operated_by_an_institution: None,
            the_number_of_nondomestic_offices_operated_by_an_institution: None,
            the_number_of_domestic_non_multiple_service_offices_operated_by_institution: None,
            the_number_of_offices_operated_by_an_institution_based_on_the_summary_of_deposits_definition_of_offices: None,
            the_number_of_states_with_offices_including_its_main_office: None,
            the_total_number_of_offices_operated_by_an_institution: None,
            the_number_of_domestic_and_u_s_territories_offices_operated_by_an_institution: None,
            a_number_used_to_identify_the_office_of_thrift_supervision_district_in_which_the_institution_is_located_01_northeast_02_southeast_04_midwest_05_west: None,
            a_number_used_to_identify_the_office_of_thrift_supervision_region_in_which_the_institution_is_located_1_northeast_2_southeast_4_midwest_5_west: None,
            identifies_the_calendar_quarter_1_march_2_june_3_september_4_december: None,
            report_date: None,
            report_date_integer: None,
            an_abbreviation_of_the_current_risk_territory_for_an_institution_fdic_risk_territory_all_periods_are_displayed_in_the_current_perspective_exceptions_can_exist_depending_on_when_a_quarter_is_updated: None,
            a_flag_used_to_indicate_whether_an_institution_was_a_former_savings_association_that_has_converted_to_a_bank_charter_and_is_still_a_saif_insured_institution_0_not_a_sasser_institution_1_is_a_sasser_institution: None,
            geographic_latitude_of_main_office: None,
            geographic_longitude_of_main_office: None,
            primary_internet_web_address: None,
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
        };
        let _ = serde_json::to_string(&props).unwrap();
    }
}
