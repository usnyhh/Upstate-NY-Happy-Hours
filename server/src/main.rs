mod api;
mod config;
mod db;
mod entities;

use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let config = Config::from_env()?;
    let db = db::connect(&config.database_url).await?;
    let app = build_app(api::AppState { db });

    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!("API server running on http://{}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}

/// Log to stdout; control verbosity with RUST_LOG.
fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "server=info,tower_http=info".into());
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn build_app(state: api::AppState) -> Router {
    // The web client is served from a different origin (trunk on :8080), so allow
    // cross-origin requests. Permissive is fine for local dev; tighten before deploying.
    api::router(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
