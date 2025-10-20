use reqwest::{Client, header};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Value, json};

use crate::{errors::errors::Error, rpc::core::make_rpc_request};

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
    let rpc_result: RpcBalanceResponse =
        make_rpc_request(rpc_url, request_id, "getBalance", json!([account_pubkey])).await?;

    Ok(rpc_result.result.value)
}

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

pub async fn get_latest_blockhash(
    rpc_url: &str,
    request_id: &str,
) -> Result<RpcLatestBlockhash, Error> {
    let rpc_result: RpcLatestBlockhashResponse = make_rpc_request(
        rpc_url,
        request_id,
        "getLatestBlockhash",
        json!([{"commitment": "processed", "minContextSlot": 1000}]),
    )
    .await?;

    Ok(rpc_result.result.value)
}

#[derive(Debug, Deserialize)]
pub struct RpcMinimumBalanceForRentExemptionResponse {
    jsonrpc: String,
    id: String,
    result: u128,
}

pub async fn get_minimum_balance_for_rent_exemption(
    rpc_url: &str,
    request_id: &str,
    bytes: u128,
) -> Result<u128, Error> {
    let rpc_result: RpcMinimumBalanceForRentExemptionResponse = make_rpc_request(
        rpc_url,
        request_id,
        "getMinimumBalanceForRentExemption",
        json!([bytes]),
    )
    .await?;

    Ok(rpc_result.result)
}
