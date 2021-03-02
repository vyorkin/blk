use crate::ByteHash;
use crypto_hash::{digest, Algorithm};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> ByteHash {
        let bytes = digest(Algorithm::SHA256, &self.bytes());
        ByteHash::from(bytes)
    }
}
