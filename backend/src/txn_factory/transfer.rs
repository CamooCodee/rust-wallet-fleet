use std::str::FromStr;

use crate::{
    errors::errors::Error, rpc::read::get_latest_blockhash, txn_factory::blockhash::get_blockhash,
};
use solana_sdk::{
    hash::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction,
};
use solana_system_interface::instruction;

pub async fn build_sol_transfer(
    wallet: &Keypair,
    lamports: u64,
    to_pubkey: &Pubkey,
    blockhash: &Hash,
) -> Result<String, Error> {
    let transfer_ix = instruction::transfer(&wallet.pubkey(), to_pubkey, lamports);

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&wallet.pubkey()));
    transaction.sign(&[wallet], blockhash.clone());

    let serialized = bincode::serialize(&transaction)?;
    let encoded = bs58::encode(serialized).into_string();

    Ok(encoded)
}
