use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
pub struct Timestamp(pub u128);

impl Timestamp {
    pub fn now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
