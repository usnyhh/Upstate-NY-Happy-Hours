//! Typed client for the Axum server (see `server/src/api/`).

use serde::Deserialize;

use crate::http;

/// Base URL of the Axum server (see `server/src/config.rs`).
const API_BASE: &str = "http://127.0.0.1:3000/api";

/// Mirrors `entities::location::Model` on the server. Field names are the JSON
/// keys, so keep the two in sync.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[allow(dead_code)]
    pub location_id: i64,
    pub name: String,
    pub address: String,
}

/// `GET /api/locations`
pub async fn fetch_locations() -> Result<Vec<Location>, String> {
    http::get_json(&format!("{API_BASE}/locations")).await
}
