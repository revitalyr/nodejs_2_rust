use crate::blockchain::BlockchainService;
use crate::error::AppError;
use crate::models::{NFTTransfer, ERC20Transfer, AddressQuery};
use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<BlockchainService>> {
    Router::new()
        .route("/nft", get(get_nft_transfers))
        .route("/erc20", get(get_erc20_transfers))
}

/// Get NFT transfers for an address
async fn get_nft_transfers(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<NFTTransfer>>, AppError> {
    let transfers = blockchain.get_nft_transfers(query.address).await?;
    Ok(Json(transfers))
}

/// Get ERC20 token transfers for an address
async fn get_erc20_transfers(
    State(blockchain): State<Arc<BlockchainService>>,
    Query(query): Query<AddressQuery>,
) -> Result<Json<Vec<ERC20Transfer>>, AppError> {
    let transfers = blockchain.get_erc20_transfers(query.address).await?;
    Ok(Json(transfers))
}
