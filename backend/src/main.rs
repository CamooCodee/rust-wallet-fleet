use axum::{
    Json, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde_json::json;
use std::{env, net::SocketAddr, sync::Arc};

mod endpoints;
mod errors;
mod funding;
mod rpc;
mod storage;
mod txn_factory;

#[derive(Clone)]
pub struct AppState {
    pub services: AppServices,
    pub rpc_url: String,
}

#[derive(Clone)]
pub struct AppServices {
    pub wallet_store:
        Arc<tokio::sync::RwLock<Box<dyn storage::wallet::WalletStorage + Send + Sync>>>,
    pub funding: Arc<tokio::sync::RwLock<Box<dyn funding::funding::Funding>>>,
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() {
    let services = AppServices {
        wallet_store: Arc::new(tokio::sync::RwLock::new(Box::new(
            storage::local_wallet_storage::LocalWalletStorage::new(),
        ))),
        funding: Arc::new(tokio::sync::RwLock::new(Box::new(
            funding::local_funding::LocalFunding::new(),
        ))),
    };

    dotenv().ok();

    let helius_api_key =
        env::var("HELIUS_API_KEY").expect("Missing HELIUS_API_KEY in environment or .env file");

    let rpc_url = format!("https://devnet.helius-rpc.com/?api-key={}", helius_api_key);

    let state = AppState {
        services,
        rpc_url: rpc_url,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/wallets/create", post(endpoints::wallet::create_wallets))
        .route("/wallets/list", get(endpoints::wallet::list_wallets))
        .route("/funding/initiate", post(endpoints::funding::initiate_job))
        .route("/funding/complete", post(endpoints::funding::complete_job))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8764));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
