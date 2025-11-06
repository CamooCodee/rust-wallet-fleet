use std::sync::Arc;

use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use tokio::task::JoinSet;

use crate::Websocket;
use crate::errors::errors::Error;
use crate::funding::funding::{Funding, FundingJob};
use crate::rpc::read::{get_balance, get_minimum_balance_for_rent_exemption};
use crate::rpc::send::send_transaction;
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
    async fn initiate_funding_job(
        &mut self,
        rpc_url: String,
        target_pubkeys: Vec<Pubkey>,
        lamports_per_wallet: u64,
    ) -> Result<&FundingJob, Error> {
        //TODO check that lamports per wallet is at least 0.001 or smth

        let total_funding_lamports =
            ((lamports_per_wallet + 5000u64) as u128) * target_pubkeys.len() as u128;

        let min_rent_result =
            get_minimum_balance_for_rent_exemption(&rpc_url, "initiate_funding", 0).await?;

        let total_lamports_to_provide = total_funding_lamports + min_rent_result;

        self.active_job = Some(FundingJob {
            distro_wallet: Keypair::new(),
            target_pubkeys: target_pubkeys,
            lamports_per_wallet: lamports_per_wallet,
            total_funding_lamports: total_lamports_to_provide,
        });

        return Ok(self.active_job.as_ref().unwrap());
    }

    async fn complete_funding_job(
        &self,
        rpc_url: String,
        websocket_service: Websocket,
    ) -> Result<(), Error> {
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

        let total_lamports_required = job.total_funding_lamports;

        let provided_funding = balance_result.unwrap();

        if (provided_funding as u128) < total_lamports_required {
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

        let mut send_set: JoinSet<Result<String, Error>> = JoinSet::new();

        let distro_wallet = Arc::new(job.distro_wallet.insecure_clone());
        let lamports_per_wallet = job.lamports_per_wallet;
        let rpc_url = Arc::new(rpc_url.clone());

        for pubkey in job.target_pubkeys.iter().cloned() {
            let distro_wallet = Arc::clone(&distro_wallet);
            let latest_hash = latest_hash.clone();
            let rpc_url = Arc::clone(&rpc_url);
            let websocket_service_arc = websocket_service.clone();

            send_set.spawn(async move {
                let txn =
                    build_sol_transfer(&distro_wallet, lamports_per_wallet, &pubkey, &latest_hash)
                        .await?;
                let sig = txn.signature[..6].to_string();

                println!("Built txn {}", sig);

                let confirmation_handle = tokio::spawn(async move {
                    let mut websocket = websocket_service_arc.write().await;
                    websocket.confirm_transaction(&txn.signature).await;
                });

                println!("Started confirmation {}", sig);

                let txn_hash = send_transaction(&rpc_url, "funding", &txn.transaction).await?;

                println!("Sent {}", sig);

                confirmation_handle.await;

                println!("Confirmed {}", sig);

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
