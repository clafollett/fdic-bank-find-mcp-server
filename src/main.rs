pub use crate::handlers::locations::__path_locations_handler;

// Standard library imports
use std::net::SocketAddr;

// External crate imports
use axum::{
    routing::get,
    Router,
    Json
};
use tower_http::services::ServeDir;
use axum::routing::get_service;
use tokio::net::TcpListener;
use tracing_subscriber;
use utoipa::OpenApi;

// Internal modules
mod handlers;
use handlers::locations::{locations_handler, LocationsQuery, LocationProperties, LocationsResponse, FdicApiConfig};

#[derive(OpenApi)]
#[openapi(
    paths(locations_handler),
    components(schemas(LocationsQuery, LocationProperties, LocationsResponse)),
    tags(
        (name = "Structure", description = "Financial institution demographic and location information")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let openapi_doc = ApiDoc::openapi();
    let fdic_config = FdicApiConfig {
        base_url: "https://banks.data.fdic.gov/api".to_string(),
    };
    let app = Router::new()
        .route("/locations", get(locations_handler))
        .with_state(fdic_config)
        .route("/openapi.json", get({
            let openapi = openapi_doc.clone();
            move || async move { Json(openapi.clone()) }
        }))
        .nest_service("/docs", get_service(ServeDir::new("public/swagger-ui")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`
    use serde_json::Value;

    #[tokio::test]
    async fn test_openapi_json_endpoint() {
        let openapi_doc = ApiDoc::openapi();
        let app = Router::new()
            .route("/openapi.json", get({
                let openapi = openapi_doc.clone();
                move || async move { Json(openapi.clone()) }
            }));
        let response = app
            .oneshot(Request::builder().uri("/openapi.json").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).expect("response is valid JSON");
        assert!(json.get("openapi").is_some(), "openapi field present");
        assert!(json.get("paths").is_some(), "paths field present");
    }

    #[tokio::test]
    async fn test_swagger_ui_docs_served() {
        let app = Router::new()
            .nest_service("/docs", get_service(ServeDir::new("public/swagger-ui")));
        let response = app
            .oneshot(Request::builder().uri("/docs/index.html").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();
        assert!(content_type.starts_with("text/html"), "Content-Type is HTML");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8_lossy(&body).to_lowercase();
        assert!(html.contains("<!doctype html>"), "Contains <!DOCTYPE html>");
        assert!(html.contains("<html"), "Contains <html> tag");
    }
}
