use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::AppState;

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    count: u16,
}

#[derive(Serialize)]
pub struct CreateWalletResponse {
    message: String,
    pubkeys: Vec<String>,
}

pub async fn create_wallets(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> Json<CreateWalletResponse> {
    let wallet_store_arc = Arc::clone(&state.services.wallet_store);
    let mut wallet_store = wallet_store_arc.write().await;
    let mut pubkeys: Vec<String> = Vec::new();

    for _ in 0..payload.count {
        let new_wallet = Keypair::new();
        pubkeys.push(new_wallet.pubkey().to_string());
        wallet_store.store_new_wallet(new_wallet);
    }

    let res = CreateWalletResponse {
        message: format!("Created {} wallets", payload.count),
        pubkeys: pubkeys,
    };
    return Json(res);
}

#[derive(Deserialize)]
#[serde(default)]
pub struct ListWalletsRequest {
    page: u16,
    page_size: u16,
}

impl Default for ListWalletsRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 999,
        }
    }
}

#[derive(Serialize)]
pub struct ListWalletsResponse {
    message: String,
    pubkeys: Vec<String>,
}

pub async fn list_wallets(
    State(state): State<AppState>,
    Query(params): Query<ListWalletsRequest>,
) -> Json<ListWalletsResponse> {
    let wallet_store_arc = Arc::clone(&state.services.wallet_store);
    let wallet_store = wallet_store_arc.read().await;
    let wallets = wallet_store.get_all_wallets(params.page, params.page_size);
    let pubkeys = wallets.iter().map(|w| w.pubkey().to_string()).collect();

    let res = ListWalletsResponse {
        message: String::from("Retrieved wallets."),
        pubkeys: pubkeys,
    };
    return Json(res);
}
