use crate::blockchain::BlockchainService;
use crate::error::AppError;
use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use alloy::primitives::Address;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct TransactionsQuery {
    pub address: Address,
    pub limit: Option<u64>,
}

pub fn routes() -> Router<Arc<BlockchainService>> {
    Router::new()
        .route("/", get(get_transactions))
}

/// Get transaction history for an address
async fn get_transactions(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<TransactionsQuery>,
) -> Result<Json<Vec<crate::models::Transaction>>, AppError> {
    let transactions = blockchain.get_transactions(query.address, query.limit).await?;
    Ok(Json(transactions))
}
