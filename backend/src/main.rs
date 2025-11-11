use axum::{
    Json, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde_json::json;
use std::{env, net::SocketAddr, path::Path, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use crate::{config::Config, websocket::solana_websocket::SolanaWebsocket};

mod collecting;
mod config;
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
    pub config: Arc<RwLock<crate::config::Config>>,
}

pub type Websocket = Arc<RwLock<Box<SolanaWebsocket>>>;

#[derive(Clone)]
pub struct AppServices {
    pub funding: Arc<RwLock<Box<dyn funding::funding::Funding>>>,
    pub websocket: Websocket,
    pub database: Arc<RwLock<tokio_rusqlite::Connection>>,
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

    let bytes = env::var("MNEMONIC")
        .expect("No mnemonic provided")
        .bytes()
        .map(|b| b as u8)
        .collect();

    let db_path = env::var("DATABASE_PATH").expect("no database path in env");
    let db_path = Path::new(&db_path);

    let services = AppServices {
        funding: Arc::new(tokio::sync::RwLock::new(Box::new(
            funding::local_funding::LocalFunding::new(),
        ))),
        websocket: Arc::new(RwLock::new(Box::new(ws))),
        database: Arc::new(RwLock::new(
            tokio_rusqlite::Connection::open(db_path)
                .await
                .expect("failed to connect to db"),
        )),
    };

    let state = AppState {
        services,
        rpc_url: rpc_url,
        config: Arc::new(RwLock::new(Config { wallet_seed: bytes })),
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
