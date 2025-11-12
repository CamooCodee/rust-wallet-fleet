use crate::{
    Websocket,
    rpc::send::send_transaction,
    txn_factory::{
        blockhash::get_blockhash,
        util::{encode_transaction, encode_versioned_transaction},
    },
};
use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};

use crate::errors::errors::Error;

#[derive(Deserialize)]
struct JupiterOrderResponse {
    transaction: String,
}

pub async fn swap_jupiter(
    rpc_url: &str,
    input_mint: &str,
    output_mint: &str,
    amount: u64,
    wallet: &Keypair,
    websocket_service: Websocket,
) -> Result<(), Error> {
    println!("Swapping");
    let client = reqwest::Client::new();
    let url = format!(
        "https://lite-api.jup.ag/ultra/v1/order?inputMint={}&outputMint={}&amount={}&taker={}",
        input_mint,
        output_mint,
        amount,
        wallet.pubkey().to_string()
    );
    println!("{}", url);

    let res = client.get(url).send().await?.error_for_status()?;

    println!("Got result");

    let jup_res = res.json::<JupiterOrderResponse>().await?;

    if jup_res.transaction.is_empty() {
        return Err(Error::InsufficientFunding(
            "Not enough input mint on the taker wallet.".to_owned(),
        ));
    }

    println!("Read body {}", jup_res.transaction);
    let decoded = general_purpose::STANDARD.decode(jup_res.transaction)?;
    println!("decoded");
    let txn: VersionedTransaction = bincode::deserialize(&decoded)?;
    let vm = txn.message.clone();
    let txn = VersionedTransaction::try_new(vm, &[wallet])?;

    let websocket_service_arc = websocket_service.clone();

    println!("Signed txn");

    let signature = txn.signatures[0].to_string();

    println!("Sig: {}", signature);

    let confirmation_handle = tokio::spawn(async move {
        let mut websocket = websocket_service_arc.write().await;
        websocket.confirm_transaction(&signature).await;
    });

    let encoded = encode_versioned_transaction(&txn);

    send_transaction(&rpc_url, "funding", &encoded).await?;

    println!("Swapping via jupiter: {}", txn.signatures[0].to_string());

    confirmation_handle.await?;

    return Ok(());
}
