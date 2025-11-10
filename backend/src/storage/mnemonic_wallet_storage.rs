use solana_sdk::{
    signature::{Keypair, keypair_from_seed},
    signer::Signer,
};

use crate::storage::wallet::WalletStorage;

pub struct MnemonicWalletStorage {
    seed: Vec<u8>,
    index: u64,
}

impl MnemonicWalletStorage {
    pub fn new(seed: Vec<u8>) -> Self {
        Self { seed, index: 0u64 }
    }

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
}

impl WalletStorage for MnemonicWalletStorage {
    fn create_new_wallet(&mut self) -> Keypair {
        let keypair = MnemonicWalletStorage::get_wallet(&self.seed, self.index);
        self.index += 1;

        return keypair;
    }

    fn get_all_wallets(&self, _page: u16, _page_size: u16) -> Vec<Keypair> {
        let mut keypairs: Vec<Keypair> = Vec::new();
        for i in 0..self.index {
            keypairs.push(MnemonicWalletStorage::get_wallet(&self.seed, i));
        }

        return keypairs;
    }

    fn get_wallets_by_pubkey(&self, pubkeys: &Vec<String>) -> Vec<Keypair> {
        let mut keypairs: Vec<Keypair> = Vec::new();
        let mut pubkeys = pubkeys.clone();
        let max_checks = 10000;
        for i in 0..max_checks {
            let keypair = MnemonicWalletStorage::get_wallet(&self.seed, i);
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
}
