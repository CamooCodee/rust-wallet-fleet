use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::{
    AppState,
    collecting::collecting::collect,
    endpoints::responses::{bad_request, confilict, server_error},
    errors::errors::Error,
    storage::mnemonic_wallet_storage::get_wallets_by_pubkey,
};

#[derive(Deserialize)]
pub struct CollectSolRequest {
    lamports: String,
    source_pubkeys: Vec<String>,
    destination: String,
}

#[derive(Serialize)]
pub struct CollectSolResponse {
    message: String,
}

pub async fn collect_sol(
    State(state): State<AppState>,
    Json(payload): Json<CollectSolRequest>,
) -> Response {
    let config_arc = Arc::clone(&state.config);
    let config = config_arc.read().await;
    let source_wallets = get_wallets_by_pubkey(&*config, &payload.source_pubkeys);

    let lamports_parse_result = payload.lamports.parse::<u64>();
    let lamports = match lamports_parse_result {
        Ok(l) => l,
        Err(_err) => {
            return bad_request("The lamports are in invalid format");
        }
    };

    let destination_parse = Pubkey::from_str(&payload.destination);
    let destination = match destination_parse {
        Ok(d) => d,
        Err(_err) => {
            return bad_request("Destination is not a valid public key");
        }
    };

    let collect_result = collect(&state.rpc_url, source_wallets, destination, lamports).await;

    if let Err(err) = collect_result {
        eprintln!("Error while collecting sol {}", err);

        if matches!(err, Error::InsufficientSol(_)) {
            return confilict("Not enough SOL in wallets.");
        }

        return server_error("Error during collection");
    }

    let res = CollectSolResponse {
        message: String::from("Collected successfully."),
    };

    return (StatusCode::OK, Json(res)).into_response();
}
