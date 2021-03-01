use blk::{block, Block, Hashable};

fn main() {
    let mut block = Block::new(0, 0, block::Hash::from(vec![0; 32]), 0, "Genesis block");
    println!("{:?}", block);
    block.hash = block::Hash::from(block.hash());
    println!("{:?}", &block);
}
