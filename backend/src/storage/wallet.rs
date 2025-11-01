use solana_sdk::signature::Keypair;

pub trait WalletStorage {
    fn store_new_wallet(&mut self, wallet: Keypair);
    fn get_all_wallets(&self, page: u16, page_size: u16) -> Vec<Keypair>;
    fn get_wallets_by_pubkey(&self, pubkeys: &Vec<String>) -> Vec<Keypair>;
}
