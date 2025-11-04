use axum::{
    Json, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde_json::json;
use std::{env, net::SocketAddr, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tungstenite::Message;

use crate::websocket::solana_websocket::SolanaWebsocket;

mod collecting;
mod endpoints;
mod errors;
mod funding;
mod rpc;
mod storage;
mod txn_factory;
mod websocket;

#[derive(Clone)]
pub struct AppState {
    pub services: AppServices,
    pub rpc_url: String,
}

pub type Websocket = Arc<RwLock<Box<SolanaWebsocket>>>;

#[derive(Clone)]
pub struct AppServices {
    pub wallet_store: Arc<RwLock<Box<dyn storage::wallet::WalletStorage + Send + Sync>>>,
    pub funding: Arc<RwLock<Box<dyn funding::funding::Funding>>>,
    pub collecting: Arc<RwLock<Box<dyn collecting::collecting::Collecting>>>,
    pub websocket: Websocket,
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let helius_api_key =
        env::var("HELIUS_API_KEY").expect("Missing HELIUS_API_KEY in environment or .env file");

    let rpc_url = format!("https://devnet.helius-rpc.com/?api-key={}", helius_api_key);
    let websocket_url = format!("wss://devnet.helius-rpc.com/?api-key={}", helius_api_key);

    let ws = SolanaWebsocket::new(&websocket_url).await;

    let services = AppServices {
        wallet_store: Arc::new(tokio::sync::RwLock::new(Box::new(
            storage::local_wallet_storage::LocalWalletStorage::new(),
        ))),
        funding: Arc::new(tokio::sync::RwLock::new(Box::new(
            funding::local_funding::LocalFunding::new(),
        ))),
        collecting: Arc::new(tokio::sync::RwLock::new(Box::new(
            collecting::default_collecting::DefaultCollecting {},
        ))),
        websocket: Arc::new(RwLock::new(Box::new(ws))),
    };

    let state = AppState {
        services,
        rpc_url: rpc_url,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/wallets/create", post(endpoints::wallet::create_wallets))
        .route("/wallets/list", get(endpoints::wallet::list_wallets))
        .route("/funding/initiate", post(endpoints::funding::initiate_job))
        .route("/funding/complete", post(endpoints::funding::complete_job))
        .route("/collect", post(endpoints::collecting::collect_sol))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8764));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
