use crate::timestamp::Timestamp;
use crate::Hashable;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
pub struct Hash(Vec<u8>);

impl Hash {
    pub fn from(bytes: Vec<u8>) -> Self {
        assert_eq!(bytes.len(), 32);
        Hash(bytes)
    }

    pub fn new() -> Self {
        Hash(vec![0; 32])
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn check_difficulty(&self, difficulty: u128) -> bool {
        let last16 = &self.to_bytes()[0..16];
        let number = u128::from_be_bytes(last16.try_into().unwrap());
        number < difficulty
    }
}

pub struct Block {
    pub index: u32,
    pub timestamp: Timestamp,
    pub hash: Hash,
    pub prev_block_hash: Hash,
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
        prev_block_hash: Hash,
        nonce: u64,
        payload: &str,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp: Timestamp(ts),
            hash: Hash::new(),
            prev_block_hash,
            nonce,
            payload: String::from(payload),
            difficulty,
        }
    }

    pub fn gen_hash(&self) -> Hash {
        Hash::from(self.hash())
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..u64::MAX {
            self.nonce = nonce_attempt;
            let hash = self.gen_hash();
            if hash.check_difficulty(self.difficulty) {
                self.hash = hash;
                break;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(&self.timestamp.0.to_be_bytes());
        bytes.extend(&self.prev_block_hash.to_bytes());
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&self.difficulty.to_be_bytes());
        bytes
    }
}
