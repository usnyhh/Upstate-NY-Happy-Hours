//! HTTP API, mounted under `/api`.

mod error;
mod locations;

use axum::Router;
use sea_orm::DatabaseConnection;

/// Shared state handed to every handler.
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub fn router(state: AppState) -> Router {
    let api = Router::new().merge(locations::routes());
    Router::new().nest("/api", api).with_state(state)
}
