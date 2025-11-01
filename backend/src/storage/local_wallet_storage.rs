use super::wallet::WalletStorage;
use solana_sdk::{signature::Keypair, signer::Signer};

pub struct LocalWalletStorage {
    pub wallets: Vec<Keypair>,
}

impl LocalWalletStorage {
    pub fn new() -> Self {
        Self {
            wallets: Vec::new(),
        }
    }
}

impl WalletStorage for LocalWalletStorage {
    fn store_new_wallet(&mut self, wallet: Keypair) {
        self.wallets.push(wallet);
    }
    fn get_all_wallets(&self, page: u16, page_size: u16) -> Vec<Keypair> {
        let mut all_wallets: Vec<Keypair> = Vec::new();

        for w in &self.wallets {
            all_wallets.push(w.insecure_clone());
        }

        all_wallets
    }
    fn get_wallets_by_pubkey(&self, pubkeys: &Vec<String>) -> Vec<Keypair> {
        let mut wallets: Vec<Keypair> = Vec::new();

        for (i, p) in pubkeys.iter().enumerate() {
            if *p == self.wallets[i].pubkey().to_string() {
                wallets.push(self.wallets[i].insecure_clone());
            }
        }

        return wallets;
    }
}
