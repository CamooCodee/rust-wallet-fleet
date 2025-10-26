use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};

use crate::{AppState, endpoints::misc::ErrorResponse, rpc::read::get_multiple_accounts};

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
    wallets: Vec<ListWalletResponseWallet>,
}

#[derive(Serialize)]
pub struct ListWalletResponseWallet {
    pubkey: String,
    sol_lamports: String,
}

pub async fn list_wallets(
    State(state): State<AppState>,
    Query(params): Query<ListWalletsRequest>,
) -> impl IntoResponse {
    let wallet_store_arc = Arc::clone(&state.services.wallet_store);
    let wallet_store = wallet_store_arc.read().await;
    let wallets = wallet_store.get_all_wallets(params.page, params.page_size);
    let pubkeys = wallets.iter().map(|w| w.pubkey().to_string()).collect();

    let accounts_result = get_multiple_accounts(&state.rpc_url, "list_wallets", &pubkeys).await;

    let accounts = match accounts_result {
        Ok(acc) => acc,
        Err(err) => {
            eprintln!("Error fetching multiple accounts {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: String::from("Interal error while feching wallet balances."),
                }),
            )
                .into_response();
        }
    };

    let mut wallets: Vec<ListWalletResponseWallet> = Vec::new();

    for (i, acc) in accounts.iter().enumerate() {
        if let Some(pk) = pubkeys.get(i) {
            let lamports = acc.as_ref().map(|a| a.lamports).unwrap_or(0u64);

            wallets.push(ListWalletResponseWallet {
                pubkey: pk.to_owned(),
                sol_lamports: lamports.to_string(),
            });
        }
    }

    let res = ListWalletsResponse {
        message: String::from("Retrieved wallets."),
        wallets: wallets,
    };

    return (StatusCode::OK, Json(res)).into_response();
}
