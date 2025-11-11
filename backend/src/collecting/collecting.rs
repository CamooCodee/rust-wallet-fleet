use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{
    errors::errors::Error,
    rpc::{read::get_multiple_accounts, send::send_transaction},
    txn_factory::{blockhash::get_blockhash, transfer::build_sol_transfer},
};

pub async fn collect(
    rpc_url: &str,
    source_wallets: Vec<Keypair>,
    destination: Pubkey,
    total_lamports_to_collect: u64,
) -> Result<(), Error> {
    let source_pubkeys: Vec<String> = source_wallets
        .iter()
        .map(|w| w.pubkey().to_string())
        .collect();
    let balance_result = get_multiple_accounts(&rpc_url, "collect_sol", &source_pubkeys).await?;

    let lamports_to_collect_per_wallet = total_lamports_to_collect / source_pubkeys.len() as u64;

    let latest_hash = match get_blockhash(&rpc_url, "collect_sol").await {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to get blockhash, {}", err);
            return Err(err);
        }
    };

    let mut txns: Vec<String> = Vec::new();

    for (i, wallet) in source_wallets.iter().enumerate() {
        let mut balance = 0u64;
        if let Some(wallet_balance) = &balance_result[i] {
            balance = wallet_balance.lamports;
        }

        if balance < lamports_to_collect_per_wallet {
            return Err(Error::InsufficientSol(format!(
                "Need {} SOL to collect but got {} inside wallet {}.",
                lamports_to_collect_per_wallet, balance, source_pubkeys[i]
            )));
        }

        let txn = build_sol_transfer(
            &wallet,
            lamports_to_collect_per_wallet,
            &destination,
            &latest_hash,
        )
        .await?;

        txns.push(txn.transaction);
    }

    for txn in txns {
        let txn_hash = send_transaction(&rpc_url, "collect_sol", &txn).await?;
        println!("Collecting: {}", txn_hash);
    }

    Ok(())
}
