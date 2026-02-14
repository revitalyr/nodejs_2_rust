use crate::blockchain::BlockchainService;
use crate::error::AppError;
use crate::models::{NFTBalance, ERC20Balance, AddressQuery};
use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<BlockchainService>> {
    Router::new()
        .route("/nft", get(get_nft_balances))
        .route("/erc20", get(get_erc20_balances))
        .route("/wallet", get(get_wallet_info))
}

/// Get NFT balances for an address
async fn get_nft_balances(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<NFTBalance>>, AppError> {
    let nfts = blockchain.get_nft_balances(query.address).await?;
    Ok(Json(nfts))
}

/// Get ERC20 token balances for an address
async fn get_erc20_balances(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<ERC20Balance>>, AppError> {
    let balances = blockchain.get_erc20_balances(query.address).await?;
    Ok(Json(balances))
}

/// Get comprehensive wallet information
async fn get_wallet_info(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let wallet_info = blockchain.get_wallet_info(query.address).await?;
    Ok(Json(serde_json::to_value(wallet_info)?))
}
