mod api;
mod db;
mod entities;

use std::net::SocketAddr;

use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "server=info,tower_http=info".into()),
        )
        .init();

    // `mode=rwc` creates the SQLite file if it doesn't exist.
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://happy_hours.db?mode=rwc".into());
    let addr: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse()?;

    let db = db::connect(&database_url).await?;

    // The client is served from a different origin (trunk on :8080), so allow
    // cross-origin requests. Permissive is fine for local dev; tighten before deploying.
    let app = api::router(api::AppState { db })
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("API server running on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
