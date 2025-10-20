use reqwest::{Client, header};
use serde::Deserialize;
use serde_json::json;

use crate::errors::errors::Error;

#[derive(Debug, Deserialize)]
struct RpcSendTransactionResponse {
    jsonrpc: String,
    id: String,
    result: String,
}

pub async fn sendTransaction(
    rpc_url: &str,
    request_id: &str,
    transaction: &str,
) -> Result<String, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": "sendTransaction",
        "params": [transaction,
        {
            "encoding": "base58",
            "preflightCommitment": "processed",
            "skipPreflight": true,
            "maxRetries": 3
        }
        ]
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

    let parsed: RpcSendTransactionResponse = serde_json::from_str(&text)?;
    Ok(parsed.result)
}
