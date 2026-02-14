use anyhow::Result;
use axum::{
    http::StatusCode,
    response::Json,
    routing::{get, Router},
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod api;
mod blockchain;
mod config;
mod error;
mod models;

use api::{balances, transfers, transactions};
use config::Config;
use blockchain::BlockchainService;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    
    info!("Starting Ethereum Boilerplate Rust server");
    info!("RPC URL: {}", config.ethereum_rpc_url);

    // Create router
    let blockchain_service = std::sync::Arc::new(BlockchainService::new(config)?);
    
    let app = Router::new()
        .route("/", get(health_check))
        .nest("/api/balances", balances::routes())
        .nest("/api/transfers", transfers::routes())
        .nest("/api/transactions", transactions::routes())
        .layer(CorsLayer::permissive())
        .with_state(blockchain_service);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "message": "Ethereum Boilerplate Rust API is running"
    })))
}
