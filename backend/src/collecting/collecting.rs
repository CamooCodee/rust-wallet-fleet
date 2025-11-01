use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::errors::errors::Error;

#[async_trait]
pub trait Collecting: Send + Sync {
    async fn collect(
        &self,
        rpc_url: &str,
        source_wallets: Vec<Keypair>,
        destination: Pubkey,
        total_lamports_to_collect: u64,
    ) -> Result<(), Error>;
}
