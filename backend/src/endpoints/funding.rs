use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::{AppState, endpoints::misc::ErrorResponse, errors::errors::Error};

#[derive(Deserialize)]
pub struct InitiateFundingRequest {
    lamports_per_wallet: u64,
}

#[derive(Serialize)]
struct JobProperty {
    funding_wallet_pubkey: String,
    total_fundig_lamports: u128,
}

#[derive(Serialize)]
pub struct InitiateFundingResponse {
    message: String,
    job: JobProperty,
}

pub async fn initiate_job(
    State(state): State<AppState>,
    Json(payload): Json<InitiateFundingRequest>,
) -> impl IntoResponse {
    let wallet_store_arc = Arc::clone(&state.services.wallet_store);
    let wallet_store = wallet_store_arc.read().await;
    let wallets = wallet_store.get_all_wallets(1, 9999);
    let pubkeys: Vec<Pubkey> = wallets.iter().map(|w| w.pubkey()).collect();
    let wallet_count = pubkeys.len() as u128;

    if wallet_count == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: String::from("There are 0 wallets."),
            }),
        )
            .into_response();
    }

    let funding_arc = Arc::clone(&state.services.funding);
    let mut funding = funding_arc.write().await;
    let job_result = funding
        .initiate_funding_job(state.rpc_url, pubkeys, payload.lamports_per_wallet)
        .await;

    let job = match job_result {
        Err(err) => {
            eprintln!("Error initiating funding job {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: String::from("There was an internal error starting the fundin job."),
                }),
            )
                .into_response();
        }
        Ok(j) => j,
    };

    let res = InitiateFundingResponse {
        message: format!("Initiated."),
        job: JobProperty {
            funding_wallet_pubkey: job.distro_wallet.pubkey().to_string(),
            total_fundig_lamports: job.total_funding_lamports,
        },
    };

    return (StatusCode::OK, Json(res)).into_response();
}

#[derive(Deserialize)]
pub struct CompleteFundingRequest {}

#[derive(Serialize)]
pub struct CompleteFundingResponse {
    message: String,
}

pub async fn complete_job(State(state): State<AppState>) -> impl IntoResponse {
    let funding_arc = Arc::clone(&state.services.funding);
    let mut funding = funding_arc.write().await;
    let completion_result = funding.complete_funding_job(state.rpc_url).await;

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
        message: format!("Completed."),
    };

    return (StatusCode::OK, Json(res)).into_response();
}
