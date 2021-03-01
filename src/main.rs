use blk::{block, Block};

fn main() {
    let mut block = Block::new(
        0,
        0,
        block::Hash::from(vec![0; 32]),
        0,
        "Genesis block",
        0x000000FFFFFFFFFFFFFFFFFFFFFFFFFF,
    );
    block.hash = block.gen_hash();
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}
