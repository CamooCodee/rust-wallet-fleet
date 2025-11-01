use std::{str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::{self, Pubkey};

use crate::{AppState, endpoints::responses::bad_request};

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
    let collecting_arc = Arc::clone(&state.services.collecting);
    let collecting_service = collecting_arc.read().await;
    let wallet_arc = Arc::clone(&state.services.wallet_store);
    let wallet_service = wallet_arc.read().await;

    let source_wallets = wallet_service.get_wallets_by_pubkey(&payload.source_pubkeys);

    let lamports_parse_result = payload.lamports.parse::<u64>();
    let lamports = match lamports_parse_result {
        Ok(l) => l,
        Err(err) => {
            return bad_request("The lamports are in invalid format");
        }
    };

    let destination_parse = Pubkey::from_str(&payload.destination);
    let destination = match destination_parse {
        Ok(d) => d,
        Err(err) => {
            return bad_request("Destination is not a valid public key");
        }
    };

    collecting_service
        .collect(&state.rpc_url, source_wallets, destination, lamports)
        .await;

    let res = CollectSolResponse {
        message: String::from("Collected successfully"),
    };

    return (StatusCode::OK, Json(res)).into_response();
}
