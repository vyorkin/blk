use crate::timestamp::Timestamp;
use crate::Hash;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: Timestamp,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}",
               &self.index,
               hex::encode(&self.hash),
               &self.timestamp.0,
               &self.payload,
        )
    }
}

impl Block {
    pub fn new(index: u32, ts: u128, prev_block_hash: Hash, nonce: u64, payload: &str) -> Self {
        Block {
            index,
            timestamp: Timestamp(ts),
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload: String::from(payload),
        }
    }
}
