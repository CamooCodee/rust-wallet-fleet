use super::wallet::WalletStorage;
use solana_sdk::signature::Keypair;

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
    fn get_all_wallets(&self, page: u16, page_size: u16) -> &Vec<Keypair> {
        return &self.wallets;
    }
}
