//! Do not edit by hand.
//! Auto-generated handler stubs for FDIC BankFind API endpoints.
use axum::{Router, routing::get};
use crate::config::FDICApiConfig;

// MCP auto-generated: Endpoint handler modules
pub mod institutions;
pub mod locations;
pub mod summary;
pub mod failures;
pub mod history;
pub mod financials;
pub mod demographics;
pub mod sod;

// MCP auto-generated: Endpoint handler imports
use institutions::institutions_handler;
use locations::locations_handler;
use summary::summary_handler;
use failures::failures_handler;
use history::history_handler;
use financials::financials_handler;
use demographics::demographics_handler;
use sod::sod_handler;

// Import all generated handler functions
/// Registers all generated FDIC handlers on the provided router.
/// This function is auto-generated and should be called by the application to attach all FDIC routes.
pub fn register_fdic_handlers(router: Router<FDICApiConfig>) -> Router<FDICApiConfig> {
    router
        .route("/institutions", get(institutions_handler))
        .route("/locations", get(locations_handler))
        .route("/summary", get(summary_handler))
        .route("/failures", get(failures_handler))
        .route("/history", get(history_handler))
        .route("/financials", get(financials_handler))
        .route("/demographics", get(demographics_handler))
        .route("/sod", get(sod_handler))
    }
