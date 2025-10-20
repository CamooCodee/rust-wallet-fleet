use std::str::FromStr;

use solana_sdk::hash::Hash;

use crate::{errors::errors::Error, rpc::read::get_latest_blockhash};

pub async fn get_blockhash(rpc_url: &str, request_id: &str) -> Result<Hash, Error> {
    let latest_blockhash = get_latest_blockhash(rpc_url, request_id).await?;
    let parsed_hash = Hash::from_str(&latest_blockhash.blockhash)?;
    Ok(parsed_hash)
}
