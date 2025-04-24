//! Comprehensive unit and integration tests for fdic-bank-find-mcp-server /locations handler and types

use fdic_bank_find_mcp_server::handlers::locations::{LocationProperties, LocationsResponse, locations_handler, LocationsQuery, FdicApiConfig};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::http::StatusCode;

// --- Unit & edge case tests ---

#[tokio::test]
async fn test_locations_handler_success() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "meta": {
            "total": 1,
            "page": 1,
            "limit": 10,
            "total_pages": 1,
            "filters": "NAME:TEST",
            "fields": "NAME,CITY,SERVTYPE,RUNDATE,STNAME,ZIP,COUNTY,FI_UNINUM",
            "sort_by": "NAME",
            "sort_order": "ASC"
        },
        "data": [
            {
                "NAME": "Test Bank",
                "SERVTYPE": 11,
                "RUNDATE": "2024-01-01",
                "CITY": "Test City",
                "STNAME": "Test State",
                "ZIP": "12345",
                "COUNTY": "Test County",
                "FI_UNINUM": "123456",
            }
        ]
    });
    Mock::given(method("GET"))
        .and(path("/locations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let params = LocationsQuery {
        api_key: None,
        filters: Some("NAME:TEST".to_string()),
        fields: Some("NAME,CITY,SERVTYPE,RUNDATE,STNAME,ZIP,COUNTY,FI_UNINUM".to_string()),
        sort_by: Some("NAME".to_string()),
        sort_order: Some("ASC".to_string()),
        limit: Some(10),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig {
        base_url: mock_server.uri(),
    };
    let response = locations_handler(
        State(config.clone()),
        Query(params)
    ).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: LocationsResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(body.meta["filters"], "NAME:TEST");
    assert_eq!(body.meta["fields"], "NAME,CITY,SERVTYPE,RUNDATE,STNAME,ZIP,COUNTY,FI_UNINUM");
    assert_eq!(body.meta["sort_by"], "NAME");
    assert_eq!(body.meta["sort_order"], "ASC");
    assert_eq!(body.data.len(), 1);
    let props = &body.data[0];
    assert_eq!(props.city.as_deref(), Some("Test City"));
    assert_eq!(props.stname.as_deref(), Some("Test State"));
    assert_eq!(props.zip.as_deref(), Some("12345"));
    assert_eq!(props.county.as_deref(), Some("Test County"));
}


#[test]
fn test_location_properties_all_none() {
    let json = r#"{}"#;
    let props: LocationProperties = serde_json::from_str(json).unwrap();
    assert!(props.address.is_none());
    assert!(props.bkclass.is_none());
    assert!(props.cbsa.is_none());
    assert!(props.cbsa_div.is_none());
    assert!(props.cbsa_div_flg.is_none());
    assert!(props.cbsa_div_no.is_none());
    assert!(props.cbsa_metro.is_none());
    assert!(props.cbsa_metro_flg.is_none());
    assert!(props.cbsa_metro_name.is_none());
    assert!(props.cbsa_micro_flg.is_none());
    assert!(props.cbsa_no.is_none());
    assert!(props.cert.is_none());
    assert!(props.city.is_none());
    assert!(props.county.is_none());
    assert!(props.csa.is_none());
    assert!(props.csa_no.is_none());
    assert!(props.csa_flg.is_none());
    assert!(props.estymd.is_none());
    assert!(props.fi_uninum.is_none());
    assert!(props.latitude.is_none());
    assert!(props.longitude.is_none());
    assert!(props.mdi_status_code.is_none());
    assert!(props.mdi_status_desc.is_none());
    assert!(props.mainoff.is_none());
    assert!(props.name.is_none());
    assert!(props.offname.is_none());
    assert!(props.offnum.is_none());
    assert!(props.rundate.is_none());
    assert!(props.servtype.is_none());
    assert!(props.stname.is_none());
    assert!(props.zip.is_none());
}

#[test]
fn test_locations_handler_response_empty_data() {
    let json = r#"{"meta":{"total":0,"page":1,"limit":10,"total_pages":0},"data":[]}"#;
    let resp: LocationsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.meta["total"], 0);
    assert_eq!(resp.data.len(), 0);
}

#[tokio::test]
async fn test_locations_handler_limit_too_high() {
    let params = LocationsQuery {
        api_key: None,
        filters: None,
        fields: None,
        sort_by: None,
        sort_order: None,
        limit: Some(10001),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: "http://dummy.local".to_string() };
    let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_locations_handler_invalid_sort_order() {
    let params = LocationsQuery {
        api_key: None,
        filters: None,
        fields: None,
        sort_by: None,
        sort_order: Some("INVALID".to_string()),
        limit: None,
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: "http://dummy.local".to_string() };
    let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// --- Integration tests for filter support ---

#[tokio::test]
async fn test_locations_handler_filter_by_city_and_class() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "meta": {"filters": "CITY:\"CHICAGO\" AND BKCLASS:NM"},
        "data": [
            {"NAME": "Test Bank", "CITY": "CHICAGO", "BKCLASS": "NM"}
        ]
    });
    Mock::given(method("GET"))
        .and(path("/locations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let params = LocationsQuery {
        api_key: None,
        filters: Some("CITY:\"CHICAGO\" AND BKCLASS:NM".to_string()),
        fields: None,
        sort_by: None,
        sort_order: None,
        limit: Some(10),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: mock_server.uri() };
    let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_locations_handler_filter_by_date_range() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "meta": {"filters": "RUNDATE:[2010-01-01 TO 2010-12-31]"},
        "data": [
            {"NAME": "Test Bank", "RUNDATE": "2010-06-15"}
        ]
    });
    Mock::given(method("GET"))
        .and(path("/locations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let params = LocationsQuery {
        api_key: None,
        filters: Some("RUNDATE:[2010-01-01 TO 2010-12-31]".to_string()),
        fields: None,
        sort_by: None,
        sort_order: None,
        limit: Some(10),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: mock_server.uri() };
    let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_locations_handler_filter_by_numeric_range() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "meta": {"filters": "FI_UNINUM:[10000 TO 15000]"},
        "data": [
            {"NAME": "Test Bank", "FI_UNINUM": 12000}
        ]
    });
    Mock::given(method("GET"))
        .and(path("/locations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let params = LocationsQuery {
        api_key: None,
        filters: Some("FI_UNINUM:[10000 TO 15000]".to_string()),
        fields: None,
        sort_by: None,
        sort_order: None,
        limit: Some(10),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: mock_server.uri() };
let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_locations_handler_filter_with_negation() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    let mock_server = MockServer::start().await;
    let mock_response = serde_json::json!({
        "meta": {"filters": "!(STNAME:\"VIRGINIA\")"},
        "data": [
            {"NAME": "Test Bank", "STNAME": "ILLINOIS"}
        ]
    });
    Mock::given(method("GET"))
        .and(path("/locations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    let params = LocationsQuery {
        api_key: None,
        filters: Some("!(STNAME:\"VIRGINIA\")".to_string()),
        fields: None,
        sort_by: None,
        sort_order: None,
        limit: Some(10),
        offset: None,
        format: None,
        download: None,
        filename: None,
    };
    let config = FdicApiConfig { base_url: mock_server.uri() };
let response = locations_handler(State(config.clone()), Query(params)).await.into_response();
    assert_eq!(response.status(), StatusCode::OK);
}
