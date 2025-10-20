use std::sync::Arc;

use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use tokio::task::JoinSet;

use crate::errors::errors::Error;
use crate::funding::funding::{Funding, FundingJob};
use crate::rpc;
use crate::rpc::read::get_balance;
use crate::rpc::send::sendTransaction;
use crate::txn_factory::blockhash::get_blockhash;
use crate::txn_factory::transfer::build_sol_transfer;

pub struct LocalFunding {
    active_job: Option<FundingJob>,
}

impl LocalFunding {
    pub fn new() -> Self {
        Self { active_job: None }
    }
}

#[async_trait]
impl Funding for LocalFunding {
    fn initiate_funding_job(
        &mut self,
        target_pubkeys: Vec<Pubkey>,
        lamports_per_wallet: u64,
    ) -> &FundingJob {
        self.active_job = Some(FundingJob {
            distro_wallet: Keypair::new(),
            target_pubkeys: target_pubkeys,
            lamports_per_wallet: lamports_per_wallet,
        });

        return self.active_job.as_ref().unwrap();
    }

    async fn complete_funding_job(&self, rpc_url: String) -> Result<(), Error> {
        let job = match &self.active_job {
            Some(j) => j,
            None => {
                return Err(Error::FundingJobNotStarted(String::from(
                    "There is no active funding job.",
                )));
            }
        };

        let balance_result = get_balance(
            &rpc_url,
            "complete_funding_balance_req",
            job.distro_wallet.pubkey().to_string().as_ref(),
        )
        .await;

        if balance_result.is_err() {
            println!("Error getting balance {}", balance_result.err().unwrap());
            return Err(Error::RpcError(String::from("Failed to get balance")));
        }

        let total_lamports_required =
            job.lamports_per_wallet as u64 * job.target_pubkeys.len() as u64;

        let provided_funding = balance_result.unwrap();

        if provided_funding < total_lamports_required {
            eprintln!("We dont have enough funding {}", provided_funding);
            return Err(Error::InsufficientFunding(String::from(
                "Insufficient funding to execute funding job",
            )));
        }

        let latest_hash = match get_blockhash(&rpc_url, "funding").await {
            Ok(v) => v,
            Err(err) => {
                eprintln!("failed to get blockhash, {}", err);
                return Err(err);
            }
        };

        // TODO run all of these in parallel
        let mut send_set: JoinSet<Result<String, Error>> = JoinSet::new();

        let distro_wallet = Arc::new(job.distro_wallet.insecure_clone());
        let lamports_per_wallet = job.lamports_per_wallet;
        let rpc_url = Arc::new(rpc_url.clone());

        for pubkey in job.target_pubkeys.iter().cloned() {
            let distro_wallet = Arc::clone(&distro_wallet);
            let latest_hash = latest_hash.clone();
            let rpc_url = Arc::clone(&rpc_url);

            send_set.spawn(async move {
                let txn =
                    build_sol_transfer(&distro_wallet, lamports_per_wallet, &pubkey, latest_hash)
                        .await?;

                let txn_hash = sendTransaction(&rpc_url, "funding", &txn).await?;

                Ok(txn_hash)
            });
        }

        while let Some(res) = send_set.join_next().await {
            match res {
                Ok(Ok(txn_hash)) => println!("Funded: {}", txn_hash),
                Ok(Err(e)) => eprintln!("Funding task error: {:?}", e),
                Err(e) => eprintln!("Join error: {:?}", e),
            }
        }

        Ok(())
    }
}
