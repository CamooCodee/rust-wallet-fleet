use crate::{
    errors::errors::Error,
    txn_factory::util::{self, encode_transaction},
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
) -> Result<util::SimpleTransaction, Error> {
    let transfer_ix = instruction::transfer(&wallet.pubkey(), to_pubkey, lamports);

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&wallet.pubkey()));
    transaction.sign(&[wallet], blockhash.clone());
    let hash = transaction.signatures[0].to_string();

    let encoded = encode_transaction(&transaction);

    Ok(util::SimpleTransaction {
        transaction: encoded,
        signature: hash,
    })
}
