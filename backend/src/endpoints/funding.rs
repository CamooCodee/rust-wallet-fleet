use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{
    AppState,
    endpoints::{
        misc::ErrorResponse,
        responses::{bad_request, server_error},
    },
    errors::errors::Error,
    storage::mnemonic_wallet_storage::get_all_wallets,
};

#[derive(Deserialize)]
pub struct InitiateFundingRequest {
    lamports_per_wallet: String,
}

#[derive(Serialize)]
struct JobProperty {
    funding_wallet_pubkey: String,
    total_funding_lamports: String,
}

#[derive(Serialize)]
pub struct InitiateFundingResponse {
    message: String,
    job: JobProperty,
}

pub async fn initiate_job(
    State(state): State<AppState>,
    Json(payload): Json<InitiateFundingRequest>,
) -> Response {
    let db_arc = Arc::clone(&state.services.database);
    let db = db_arc.read().await;
    let config_arc = Arc::clone(&state.config);
    let config = config_arc.read().await;

    let lamports = payload.lamports_per_wallet.parse::<u64>();
    let lamports_per_wallet = match lamports {
        Err(_) => {
            return bad_request("Invalid lamports string format");
        }
        Ok(v) => v,
    };

    let wallets = get_all_wallets(&*db, &*config, 1, 9999).await;
    let wallets = match wallets {
        Err(err) => {
            eprintln!("Error getting wallets for funding: {}", err);
            return server_error("Internal error");
        }
        Ok(w) => w,
    };

    let pubkeys: Vec<Pubkey> = wallets.iter().map(|w| w.pubkey()).collect();
    let wallet_count = pubkeys.len() as u128;

    if wallet_count == 0 {
        return bad_request("There are 0 wallets.");
    }

    let funding_arc = Arc::clone(&state.services.funding);
    let mut funding = funding_arc.write().await;
    let job_result = funding
        .initiate_funding_job(state.rpc_url, pubkeys, lamports_per_wallet)
        .await;

    let job = match job_result {
        Err(err) => {
            eprintln!("Error initiating funding job {}", err);
            return server_error("There was an internal error starting the fundin job.");
        }
        Ok(j) => j,
    };

    let res = InitiateFundingResponse {
        message: format!("Initiated funding."),
        job: JobProperty {
            funding_wallet_pubkey: job.distro_wallet.pubkey().to_string(),
            total_funding_lamports: job.total_funding_lamports.to_string(),
        },
    };

    return (StatusCode::OK, Json(res)).into_response();
}

#[derive(Serialize)]
pub struct CompleteFundingResponse {
    message: String,
}

pub async fn complete_job(State(state): State<AppState>) -> impl IntoResponse {
    let funding_arc = Arc::clone(&state.services.funding);
    let funding = funding_arc.write().await;
    let completion_result = funding
        .complete_funding_job(state.rpc_url, state.services.websocket)
        .await;

    if completion_result.is_err() {
        if let Err(Error::FundingJobNotStarted(_)) = completion_result {
            return (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: String::from("No active funding job."),
                }),
            )
                .into_response();
        } else if let Err(Error::InsufficientFunding(_)) = completion_result {
            return (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: String::from("Insufficient funding."),
                }),
            )
                .into_response();
        } else {
            eprintln!(
                "Error completing funding job: {:?}",
                completion_result.err()
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: String::from("Internal error."),
                }),
            )
                .into_response();
        }
    }

    let res = CompleteFundingResponse {
        message: format!("Completed funding."),
    };

    return (StatusCode::OK, Json(res)).into_response();
}
