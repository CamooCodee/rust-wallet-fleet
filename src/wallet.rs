pub struct Wallet {
    pub pubkey: String,
    pub pubkey_bytes: [u8; 32],
    pub secret_key: [u8; 32],
    pub tag: String,
    pub was_grinded: bool,
    pub grind_job_id: String,
}
