use reqwest::{Client, header};
use serde::Deserialize;
use serde_json::json;

use crate::errors::errors::Error;

#[derive(Debug, Deserialize)]
struct RpcLatestBlockhashResponse {
    jsonrpc: String,
    id: String,
    result: RpcLatestBlockhashResult,
}

#[derive(Debug, Deserialize)]
struct RpcLatestBlockhashResult {
    context: RpcContext,
    value: RpcLatestBlockhash,
}

#[derive(Debug, Deserialize)]
pub struct RpcLatestBlockhash {
    pub blockhash: String,
    #[serde(rename = "lastValidBlockHeight")]
    pub last_valid_block_height: u32,
}

#[derive(Debug, Deserialize)]
struct RpcBalanceResponse {
    jsonrpc: String,
    id: String,
    result: RpcBalanceResult,
}

#[derive(Debug, Deserialize)]
struct RpcBalanceResult {
    context: RpcContext,
    value: u64,
}

#[derive(Debug, Deserialize)]
struct RpcContext {
    slot: u64,
    #[serde(rename = "apiVersion")]
    api_version: String,
}

pub async fn get_balance(
    rpc_url: &str,
    request_id: &str,
    account_pubkey: &str,
) -> Result<u64, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "getBalance",
        "params": [account_pubkey]
    });

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = client
        .post(rpc_url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(Error::RpcError(format!("http {}: {}", status, text)));
    }

    let parsed: RpcBalanceResponse = serde_json::from_str(&text)?;
    let balance = parsed.result.value;

    println!("Balance: {}", balance);
    Ok(balance)
}

pub async fn get_latest_blockhash(
    rpc_url: &str,
    request_id: &str,
) -> Result<RpcLatestBlockhash, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "getLatestBlockhash",
        "params": [{"commitment": "processed", "minContextSlot": 1000}]
    });

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let resp = client
        .post(rpc_url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        return Err(Error::RpcError(format!("http {}: {}", status, text)));
    }

    println!("{}", text);

    let parsed: RpcLatestBlockhashResponse = serde_json::from_str(&text)?;

    Ok(parsed.result.value)
}

pub fn get_minimum_balance_for_rent_exemption() {}
