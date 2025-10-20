use reqwest::{Client, header};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::errors::errors::Error;

pub async fn make_rpc_request<T>(
    rpc_url: &str,
    request_id: &str,
    rpc_method: &str,
    params: Value,
) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let body = json!({
        "jsonrpc": "2.0",
        "id": request_id,
        "method": rpc_method,
        "params": params
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

    let parsed: T = serde_json::from_str(&text)?;

    Ok(parsed)
}
