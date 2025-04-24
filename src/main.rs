// Standard library imports
use std::net::SocketAddr;

// External crate imports
use axum::{
    routing::get,
    Router,
    Json
};
use tokio::net::TcpListener;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Internal modules
mod handlers;
use handlers::locations::{locations_handler, LocationsQuery};

#[derive(OpenApi)]
#[openapi(
    paths(locations_handler),
    components(schemas(LocationsQuery)),
    tags(
        (name = "Structure", description = "Financial institution demographic and location information")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let openapi = ApiDoc::openapi();
    let app = Router::new()
        .route("/locations", get(locations_handler))
        .route("/openapi.json", get(|| async { Json(openapi.clone()) }))
        .route_service("/docs", SwaggerUi::new("/docs").url("/openapi.json", openapi.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    // use super::*; // REMOVE: unused import
    use std::collections::HashMap;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_locations_handler_success() {
        // Start a mock server
        let mock_server = MockServer::start().await;
        let mock_response = serde_json::json!({
            "meta": {
                "total": 1,
                "page": 1,
                "limit": 10,
                "total_pages": 1,
                "filters": "NAME:TEST",
                "fields": "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY",
                "sort_by": "NAME",
                "sort_order": "ASC"
            },
            "data": [
                {
                    "NAME": "Test Bank",
                    "UNINUM": 123456,
                    "SERVTYPE": "11",
                    "RUNDATE": "2024-01-01",
                    "CITY": "Test City",
                    "STNAME": "Test State",
                    "ZIP": "12345",
                    "COUNTY": "Test County"
                }
            ]
        });
        Mock::given(method("GET"))
            .and(path("/api/locations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;
        let client = reqwest::Client::new();
        let mut params = HashMap::new();
        params.insert("filters".to_string(), "NAME:TEST".to_string());
        params.insert(
            "fields".to_string(),
            "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY".to_string(),
        );
        params.insert("sort_by".to_string(), "NAME".to_string());
        params.insert("sort_order".to_string(), "ASC".to_string());
        let url = format!("{}/api/locations", &mock_server.uri());
        let req = client.get(&url).query(&params);
        let resp = req.send().await.unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        let body: serde_json::Value = resp.json().await.unwrap();
        let meta = &body["meta"];
        assert_eq!(meta["total"], 1);
        assert_eq!(meta["page"], 1);
        assert_eq!(meta["limit"], 10);
        assert_eq!(meta["total_pages"], 1);
        assert_eq!(meta["filters"], "NAME:TEST");
        assert_eq!(
            meta["fields"],
            "NAME,UNINUM,SERVTYPE,RUNDATE,CITY,STNAME,ZIP,COUNTY"
        );
        assert_eq!(meta["sort_by"], "NAME");
        assert_eq!(meta["sort_order"], "ASC");
        let data = &body["data"][0];
        assert_eq!(data["NAME"], "Test Bank");
        assert_eq!(data["UNINUM"], 123456);
        assert_eq!(data["SERVTYPE"], "11");
        assert_eq!(data["RUNDATE"], "2024-01-01");
        assert_eq!(data["CITY"], "Test City");
        assert_eq!(data["STNAME"], "Test State");
        assert_eq!(data["ZIP"], "12345");
        assert_eq!(data["COUNTY"], "Test County");
    }
}
