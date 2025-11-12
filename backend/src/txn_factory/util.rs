use solana_sdk::transaction::{Transaction, VersionedTransaction};

pub struct SimpleTransaction {
    pub transaction: String,
    pub signature: String,
}

pub fn encode_transaction(txn: &Transaction) -> String {
    let serialized = bincode::serialize(txn).expect("failed to serialize transaction");
    let encoded = bs58::encode(serialized).into_string();
    return encoded;
}

pub fn encode_versioned_transaction(txn: &VersionedTransaction) -> String {
    let serialized = bincode::serialize(txn).expect("failed to serialize transaction");
    let encoded = bs58::encode(serialized).into_string();
    return encoded;
}
