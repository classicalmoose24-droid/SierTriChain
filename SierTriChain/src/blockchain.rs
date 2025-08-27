use crate::block::Block;
use crate::core::mining::mine_genesis;
use rust_decimal_macros::dec;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Self::create_genesis_block();
        Blockchain { blocks: vec![genesis_block] }
    }

    fn create_genesis_block() -> Block {
        loop {
            if let Some(mining_result) = mine_genesis(1, dec!(1e-6)) {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let mut block = Block {
                    index: 0,
                    timestamp,
                    transactions: vec![],
                    previous_hash: "0".to_string(),
                    hash: "".to_string(),
                    mining_result,
                };
                block.hash = block.calculate_hash();
                return block;
            }
        }
    }

    pub fn add_block(&mut self, new_block: Block) {
        self.blocks.push(new_block);
    }
}
