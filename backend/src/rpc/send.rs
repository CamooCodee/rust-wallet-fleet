use serde::Deserialize;
use serde_json::json;

use crate::{errors::errors::Error, rpc::core::make_rpc_request};

#[derive(Debug, Deserialize)]
struct RpcSendTransactionResponse {
    jsonrpc: String,
    id: String,
    result: String,
}

pub async fn send_transaction(
    rpc_url: &str,
    request_id: &str,
    transaction: &str,
) -> Result<String, Error> {
    let rpc_result: RpcSendTransactionResponse = make_rpc_request(
        rpc_url,
        request_id,
        "sendTransaction",
        json!([transaction,
        {
            "encoding": "base58",
            "preflightCommitment": "processed",
            "skipPreflight": true,
            "maxRetries": 3
        }
        ]),
    )
    .await?;

    Ok(rpc_result.result)
}
