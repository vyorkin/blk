mod block;
mod timestamp;

pub use block::Block;
pub use timestamp::Timestamp;

type Hash = Vec<u8>;

type Address = String;
