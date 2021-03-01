use crate::{block, Block};

#[derive(Debug)]
pub enum ValidationError {
    IndexMismatch,
    InvalidHash,
    InvalidGenesisBlock,
    AchronologicalTimestamp,
    ParentHashMismatch,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn validate(&self) -> Result<(), ValidationError> {
        for (i, block) in self.blocks.iter().enumerate() {
            self.validate_block(&block, i as u32)?;
        }
        Ok(())
    }

    fn validate_block(&self, block: &Block, ix: u32) -> Result<(), ValidationError> {
        if block.index != ix {
            return Err(ValidationError::IndexMismatch);
        }
        if !block.hash.check_difficulty(block.difficulty) {
            return Err(ValidationError::InvalidHash);
        }

        if block.is_genesis() {
            self.validate_genesis_block(&block)?;
        } else {
            self.validate_normal_block(&block)?;
        }

        Ok(())
    }

    fn validate_genesis_block(&self, block: &Block) -> Result<(), ValidationError> {
        if block.parent_hash.to_bytes() != vec![0; 32] {
            Err(ValidationError::InvalidGenesisBlock)
        } else {
            Ok(())
        }
    }

    fn validate_normal_block(&self, block: &Block) -> Result<(), ValidationError> {
        let parent_block = &self.blocks[block.index as usize - 1];
        if block.timestamp <= parent_block.timestamp {
            return Err(ValidationError::AchronologicalTimestamp)
        }
        if block.parent_hash != parent_block.hash {
            return Err(ValidationError::ParentHashMismatch)
        }
        Ok(())
    }
}
