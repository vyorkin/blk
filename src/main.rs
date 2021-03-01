use blk::{block, Block};

fn main() {
    let difficulty = 0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let mut block = Block::new(
        0,
        0,
        block::Hash::from(vec![0; 32]),
        0,
        "Genesis block",
        difficulty,
    );
    block.hash = block.gen_hash();
    println!("{:?}", &block);
    block.mine();
    println!("{:?}", &block);
}
