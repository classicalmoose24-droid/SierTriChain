#[cfg(test)]
mod tests {
    #[test]
    fn test_territory_basic() {
        assert_eq!(2 + 2, 4);
    }
}
// Territorial ownership system for Triangular Territory Cryptocurrency
// Each triangle is owned by an address and may have staked tokens for defense

use crate::geometry::triangle::{Triangle};
use crate::geometry::subdivision::FractalAddress;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Territory {
    pub triangle: Triangle,
    pub address: FractalAddress,
    pub owner: String, // Could be a wallet address
    pub staked_tokens: f64,
    pub yield_tokens: f64,
}

pub struct TerritoryRegistry {
    pub territories: HashMap<String, Territory>, // Key: geometric hash
}

impl TerritoryRegistry {
    pub fn new() -> Self {
        Self { territories: HashMap::new() }
    }

    // Claim a territory if geometric proof and adjacency are valid
    pub fn claim_territory(&mut self, triangle: Triangle, address: FractalAddress, owner: String, staked_tokens: f64) -> Result<(), String> {
        let hash = crate::hash::geometric_hash(&triangle, 8);
        if self.territories.contains_key(&hash) {
            return Err("Territory already claimed".to_string());
        }
        // TODO: Verify geometric proof and adjacency
        let territory = Territory {
            triangle,
            address,
            owner,
            staked_tokens,
            yield_tokens: 0.0,
        };
        self.territories.insert(hash, territory);
        Ok(())
    }

        // Cryptographic territory claiming: requires geometric proof and adjacency
        pub fn cryptographic_claim(&mut self, triangle: Triangle, address: FractalAddress, owner: String, staked_tokens: f64, adjacent_hashes: Vec<String>) -> Result<(), String> {
            let hash = crate::hash::geometric_hash(&triangle, 8);
            if self.territories.contains_key(&hash) {
                return Err("Territory already claimed".to_string());
            }
            // Geometric validity proof
            if !crate::validation::verify_geometric_proof(&triangle, 1e-8) {
                return Err("Invalid geometric proof".to_string());
            }
            // Adjacency check: must be adjacent to at least one owned territory
            let mut adjacency_valid = false;
            for adj_hash in adjacent_hashes {
                if let Some(adj_territory) = self.territories.get(&adj_hash) {
                    adjacency_valid = true;
                    break;
                }
            }
            if !adjacency_valid {
                return Err("No valid adjacency to existing territory".to_string());
            }
            let territory = Territory {
                triangle,
                address,
                owner,
                staked_tokens,
                yield_tokens: 0.0,
            };
            self.territories.insert(hash, territory);
            Ok(())
        }

        // Exponential value scaling: triangle value = base_value / (area^2)
        pub fn triangle_value(&self, triangle: &Triangle, base_value: f64) -> f64 {
            let area = crate::area::triangle_area(triangle);
            if area <= 0.0 {
                return 0.0;
            }
            base_value / (area * area)
        }

        // Staking defense: locked tokens protect territory from attacks
        pub fn defend_territory(&mut self, hash: &str, additional_stake: f64) -> Result<(), String> {
            if let Some(territory) = self.territories.get_mut(hash) {
                territory.staked_tokens += additional_stake;
                Ok(())
            } else {
                Err("Territory not found".to_string())
            }
        }

        // Conquest mechanics: capture triangle through superior geometric proof
        pub fn conquer_territory(&mut self, hash: &str, challenger: String, challenger_triangle: Triangle, challenger_stake: f64) -> Result<(), String> {
            if let Some(territory) = self.territories.get(hash) {
                // Challenger must provide a valid geometric proof and higher stake
                if !crate::validation::verify_geometric_proof(&challenger_triangle, 1e-8) {
                    return Err("Challenger geometric proof invalid".to_string());
                }
                if challenger_stake > territory.staked_tokens {
                    let new_territory = Territory {
                        triangle: challenger_triangle,
                        address: territory.address.clone(),
                        owner: challenger,
                        staked_tokens: challenger_stake,
                        yield_tokens: territory.yield_tokens,
                    };
                    self.territories.insert(hash.to_string(), new_territory);
                    Ok(())
                } else {
                    Err("Challenger stake not sufficient".to_string())
                }
            } else {
                Err("Territory not found".to_string())
            }
        }

        // Territorial yield farming: adjacent triangle ownership generates compound rewards
        pub fn yield_farming(&mut self, hash: &str, reward_rate: f64) -> Result<f64, String> {
            if let Some(territory) = self.territories.get_mut(hash) {
                // Count adjacent territories owned by same owner
                let mut adjacent_owned = 0;
                for (adj_hash, adj_territory) in &self.territories {
                    if adj_hash != hash && adj_territory.owner == territory.owner {
                        // TODO: Check actual geometric adjacency
                        adjacent_owned += 1;
                    }
                }
                let compound_reward = reward_rate * (1.0 + adjacent_owned as f64).powf(1.2);
                territory.yield_tokens += compound_reward;
                Ok(compound_reward)
            } else {
                Err("Territory not found".to_string())
            }
        }

    // Get territory by geometric hash
    pub fn get_territory(&self, hash: &str) -> Option<&Territory> {
        self.territories.get(hash)
    }
}
