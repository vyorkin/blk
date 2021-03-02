use crate::{ByteHash, Hashable, Timestamp};
use std::{
    convert::TryInto,
    fmt::{self, Debug, Formatter},
};

pub struct Block {
    pub index: u32,
    pub timestamp: Timestamp,
    pub hash: ByteHash,
    pub parent_hash: ByteHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            hex::encode(&self.hash.to_bytes()),
            &self.timestamp.0,
            &self.payload,
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        ts: u128,
        parent_hash: ByteHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp: Timestamp(ts),
            hash: ByteHash::new(),
            parent_hash,
            nonce,
            payload,
            difficulty,
        }
    }

    pub fn is_genesis(&self) -> bool {
        self.index == 0
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..u64::MAX {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if self.check_difficulty(&hash) {
                self.hash = hash;
                break;
            }
        }
    }

    pub fn check_difficulty(&self, hash: &ByteHash) -> bool {
        let last16 = &hash.to_bytes()[0..16];
        let number = u128::from_be_bytes(last16.try_into().unwrap());
        number < self.difficulty
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(&self.timestamp.0.to_be_bytes());
        bytes.extend(&self.parent_hash.to_bytes());
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&self.difficulty.to_be_bytes());
        bytes
    }
}
