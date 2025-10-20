use async_trait::async_trait;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::errors::errors::Error;

#[derive(Debug)]
pub struct FundingJob {
    pub distro_wallet: Keypair,
    pub target_pubkeys: Vec<Pubkey>,
    pub lamports_per_wallet: u64,
    pub total_funding_lamports: u128,
}

#[async_trait]
pub trait Funding: Send + Sync {
    async fn initiate_funding_job(
        &mut self,
        rpc_url: String,
        target_pubkeys: Vec<Pubkey>,
        lamports_per_wallet: u64,
    ) -> Result<&FundingJob, Error>;
    async fn complete_funding_job(&self, rpc_url: String) -> Result<(), Error>;
}
