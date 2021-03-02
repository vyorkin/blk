use blk::{blockchain, Block, Blockchain, ByteHash, Hashable, Timestamp};

fn main() -> Result<(), blockchain::ValidationError> {
    let difficulty = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFF;
    let mut block = Block::new(
        0,
        0,
        ByteHash::from(vec![0; 32]),
        0,
        String::from("Genesis block"),
        difficulty,
    );
    block.hash = block.hash();
    block.mine();
    println!("{:?}", &block);

    let mut parent_hash = block.hash.clone();
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };
    for i in 1..=10 {
        let mut block = Block::new(
            i,
            Timestamp::now(),
            parent_hash.clone(),
            0,
            format!("block #{}", i),
            difficulty,
        );
        block.mine();
        println!("{:?}", &block);
        parent_hash = block.hash.clone();
        blockchain.blocks.push(block);
    }
    blockchain.validate()
}
