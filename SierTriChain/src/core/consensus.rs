
//! Consensus logic for SierTriChain blockchain.
//! Provides block validation and chain scoring mechanisms.

use crate::block::Block;

/// Errors that can occur during block validation.
#[derive(Debug, PartialEq)]
pub enum BlockValidationError {
    InvalidIndex,
    InvalidPreviousHash,
    InvalidHash,
}

/// Trait for consensus algorithms.
pub trait Consensus {
    fn validate_block(&self, block: &Block, previous_block: &Block) -> Result<(), BlockValidationError>;
    fn chain_complexity_score(&self, chain: &[Block]) -> f64;
}


/// Default consensus implementation.
pub struct DefaultConsensus;

impl Consensus for DefaultConsensus {
    /// Calculates the complexity score of a chain.
    fn chain_complexity_score(&self, chain: &[Block]) -> f64 {
        chain.iter().map(|block| block.mining_result.address.0.len() as f64).sum()
    }

    /// Validates a block against its previous block.
    fn validate_block(&self, block: &Block, previous_block: &Block) -> Result<(), BlockValidationError> {
        if block.index != previous_block.index + 1 {
            return Err(BlockValidationError::InvalidIndex);
        }
        if block.previous_hash != previous_block.hash {
            return Err(BlockValidationError::InvalidPreviousHash);
        }
        if block.hash != block.calculate_hash() {
            return Err(BlockValidationError::InvalidHash);
        }
        Ok(())
    }
}

/// Legacy block validation function (use Consensus trait for extensibility).
pub fn validate_block(block: &Block, previous_block: &Block) -> bool {
    DefaultConsensus.validate_block(block, previous_block).is_ok()
}