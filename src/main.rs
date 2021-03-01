use blk::{block, Block, Hashable};

fn main() {
    let block = Block::new(0, 0, block::Hash::from(vec![0; 32]), 0, "Genesis block");
    println!("{:?}", block);
    let hash = block.hash();
    println!("{:?}", &hash);
}
