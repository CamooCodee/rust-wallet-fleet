use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use solana_sdk::signer::Signer;

use crate::{
    AppState,
    endpoints::{misc::ErrorResponse, responses::server_error},
    rpc::read::get_multiple_accounts,
    storage::mnemonic_wallet_storage::{create_new_wallet, get_all_wallets},
};

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
) -> Response {
    let mut pubkeys: Vec<String> = Vec::new();
    let db_arc = Arc::clone(&state.services.database);
    let db = db_arc.read().await;
    let config_arc = Arc::clone(&state.config);
    let config = config_arc.read().await;

    for _ in 0..payload.count {
        let new_wallet = create_new_wallet(&*db, &*config).await;
        match new_wallet {
            Err(err) => {
                eprintln!("Error creating new wallet: {}", err);
                return server_error("Failed to create new wallet.");
            }
            Ok(wallet) => {
                pubkeys.push(wallet.pubkey().to_string());
            }
        }
    }

    let res = CreateWalletResponse {
        message: format!("Created {} wallets", payload.count),
        pubkeys: pubkeys,
    };
    return (StatusCode::OK, Json(res)).into_response();
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
    let db_arc = Arc::clone(&state.services.database);
    let db = db_arc.read().await;
    let config_arc = Arc::clone(&state.config);
    let config = config_arc.read().await;

    let wallets = get_all_wallets(&*db, &*config, params.page, params.page_size).await;
    let wallets = match wallets {
        Err(err) => {
            eprintln!("Error getting wallets: {}", err);
            return server_error("Internal error.");
        }
        Ok(w) => w,
    };

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
