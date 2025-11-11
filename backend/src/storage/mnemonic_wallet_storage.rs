use crate::{config::Config, errors::errors::Error};
use solana_sdk::{
    signature::{Keypair, keypair_from_seed},
    signer::Signer,
};
use tokio_rusqlite::Connection;

fn get_wallet(seed: &Vec<u8>, index: u64) -> Keypair {
    let index_bytes = index.to_be_bytes();
    let mut seed_with_index = seed.clone();
    seed_with_index.extend_from_slice(&index_bytes);
    let keypair_result = keypair_from_seed(&seed_with_index);

    let keypair = match keypair_result {
        Ok(k) => k,
        Err(err) => {
            panic!("Failed to create keypair from seed {}", err);
        }
    };

    return keypair;
}

pub async fn create_new_wallet(database: &Connection, config: &Config) -> Result<Keypair, Error> {
    let wallet_index = database
        .call(|conn| {
            let new_seed = conn.query_row(
                "
                INSERT INTO wallets DEFAULT VALUES
                RETURNING seed;
                ",
                [],
                |row| row.get(0),
            )?;

            return Ok(new_seed);
        })
        .await?;

    let keypair = get_wallet(&config.wallet_seed, wallet_index);

    return Ok(keypair);
}

pub async fn get_all_wallets(
    database: &Connection,
    config: &Config,
    _page: u16,
    _page_size: u16,
) -> Result<Vec<Keypair>, Error> {
    let all_wallets = database
        .call(|conn| {
            let mut stmt = conn.prepare(
                "
                SELECT seed
                FROM wallets;
                ",
            )?;
            let wallet_seeds = stmt
                .query_map([], |row| Ok(row.get::<usize, u64>(0)?))?
                .collect::<Result<Vec<u64>, rusqlite::Error>>()?;

            return Ok(wallet_seeds);
        })
        .await?;

    let mut keypairs: Vec<Keypair> = Vec::new();
    for i in all_wallets {
        keypairs.push(get_wallet(&config.wallet_seed, i));
    }

    return Ok(keypairs);
}

pub fn get_wallets_by_pubkey(config: &Config, pubkeys: &Vec<String>) -> Vec<Keypair> {
    let mut keypairs: Vec<Keypair> = Vec::new();
    let mut pubkeys = pubkeys.clone();
    let max_checks = 10000;
    for i in 0..max_checks {
        let keypair = get_wallet(&config.wallet_seed, i);
        let found = pubkeys
            .iter()
            .position(|p| *p == keypair.pubkey().to_string());

        if let Some(index) = found {
            keypairs.push(keypair);
            pubkeys.remove(index);
        }
    }

    return keypairs;
}
