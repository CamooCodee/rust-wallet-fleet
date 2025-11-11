use crate::{config::Config, errors::errors::Error};
use rusqlite::params;
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
    let pubkey = keypair.pubkey().to_string();

    database
        .call(move |conn| {
            conn.execute(
                "UPDATE wallets SET pubkey = ?1 WHERE seed = ?2",
                params![pubkey, wallet_index],
            )?;

            return Ok(());
        })
        .await?;

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

pub async fn get_wallets_by_pubkey(
    database: &Connection,
    config: &Config,
    pubkeys: &Vec<String>,
) -> Result<Vec<Keypair>, Error> {
    let pubkeys_statement = pubkeys
        .iter()
        .map(|p| format!("'{}'", p))
        .collect::<Vec<String>>()
        .join(", ");

    let sql = format!(
        "SELECT seed FROM wallets WHERE pubkey IN ({});",
        pubkeys_statement
    );

    println!("Constructed sql to get_wallets_by_pubkey: {}", sql);

    let wallet_seeds = database
        .call(move |conn| {
            let mut stmt = conn.prepare(&sql)?;
            let wallet_seeds = stmt
                .query_map([], |row| Ok(row.get::<usize, u64>(0)?))?
                .collect::<Result<Vec<u64>, rusqlite::Error>>()?;

            return Ok(wallet_seeds);
        })
        .await?;

    let mut keypairs: Vec<Keypair> = Vec::new();
    for i in wallet_seeds {
        let keypair = get_wallet(&config.wallet_seed, i);
        keypairs.push(keypair);
    }

    return Ok(keypairs);
}
