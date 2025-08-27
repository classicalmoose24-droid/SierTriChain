use crate::core::mining::MiningResult;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<String>, // Placeholder for transactions
    pub previous_hash: String,
    pub hash: String,
    pub mining_result: MiningResult,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let record = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.mining_result.address.0.iter().map(|&x| x.to_string()).collect::<String>()
        );
        hasher.update(record.as_bytes());
        hex::encode(hasher.finalize())
    }
}
