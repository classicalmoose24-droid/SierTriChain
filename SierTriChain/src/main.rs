pub mod block;
pub mod blockchain;
pub mod core;
pub mod crypto;
pub mod defi;
pub mod geometry;
pub mod network;
pub mod protocol;
pub mod vm;
pub mod wallet;


use blockchain::Blockchain;
use rust_decimal_macros::dec;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("SierTriChain: Geometric blockchain engine initialized.");

    let mut blockchain = Blockchain::new();
    println!("Genesis block created: {:?}", blockchain.blocks[0]);

    let initial_depth = 1;
    let adjustment_interval = 10;

    for i in 1..4 {
        let block_height = i as usize;
        let depth = core::mining::required_fractal_depth(block_height, initial_depth, adjustment_interval);
        println!("Mining block {} at depth {}", block_height, depth);

        if let Some(mining_result) = core::mining::mine_genesis(depth, dec!(1e-6)) {
            let previous_block = blockchain.blocks.last().unwrap();
            let mut new_block = block::Block {
                index: previous_block.index + 1,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                transactions: vec![],
                previous_hash: previous_block.hash.clone(),
                hash: "".to_string(),
                mining_result,
            };
            new_block.hash = new_block.calculate_hash();
            println!("New block found: {:?}", new_block);
            blockchain.add_block(new_block);
        } else {
            println!("No triangle found at depth {}", depth);
        }
    }

    println!("\nChain complexity score: {}", core::consensus::chain_complexity_score(&blockchain.blocks));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_basic() {
        assert_eq!(2 + 2, 4);
    }
}
