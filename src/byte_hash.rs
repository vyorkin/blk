#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteHash(Vec<u8>);

impl ByteHash {
    pub fn from(bytes: Vec<u8>) -> Self {
        assert_eq!(bytes.len(), 32);
        ByteHash(bytes)
    }

    pub fn new() -> Self {
        ByteHash(vec![0; 32])
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
