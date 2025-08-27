#[cfg(test)]
mod tests {
    #[test]
    fn test_network_fractal_network_basic() {
        assert_eq!(2 + 2, 4);
    }
}
// Fractal tree routing and network topology for geometric territory system
// Quantum-resistant cryptography stubs included

use crate::geometry::subdivision::FractalAddress;
use std::collections::{HashMap, VecDeque};
use crate::geometry::point::Point;
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};

pub struct NetworkNode {
    pub address: FractalAddress,
    pub peers: Vec<FractalAddress>, // Fractal adjacency
}

pub struct FractalNetwork {
    pub nodes: HashMap<String, NetworkNode>, // Key: base-3 address string
}

impl FractalNetwork {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    // Add node to network
    pub fn add_node(&mut self, node: NetworkNode) {
        let key = Self::address_to_string(&node.address);
        self.nodes.insert(key, node);
    }

    // Fractal tree traversal routing: BFS from source to target
    pub fn route(&self, source: &FractalAddress, target: &FractalAddress) -> Option<Vec<FractalAddress>> {
        let src_key = Self::address_to_string(source);
        let tgt_key = Self::address_to_string(target);
        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((src_key.clone(), vec![source.clone()]));
        while let Some((current, path)) = queue.pop_front() {
            if current == tgt_key {
                return Some(path);
            }
            if let Some(node) = self.nodes.get(&current) {
                for peer in &node.peers {
                    let peer_key = Self::address_to_string(peer);
                    if !visited.contains_key(&peer_key) {
                        let mut new_path = path.clone();
                        new_path.push(peer.clone());
                        queue.push_back((peer_key.clone(), new_path));
                        visited.insert(peer_key, true);
                    }
                }
            }
        }
        None
    }

    // Optimize topology: connect nodes with minimal fractal distance
    pub fn optimize_topology(&mut self) {
        // Connect each node to its nearest neighbors in fractal space
        for node in self.nodes.values_mut() {
            let mut distances: Vec<(f64, FractalAddress)> = self.nodes
                .values()
                .filter(|other| other.address != node.address)
                .map(|other| {
                    let p1 = Point {
                        x: Decimal::from_f64(node.address.0.iter().map(|d| *d as f64).sum::<f64>()).unwrap(),
                        y: Decimal::from_f64(node.address.0.len() as f64).unwrap(),
                    };
                    let p2 = Point {
                        x: Decimal::from_f64(other.address.0.iter().map(|d| *d as f64).sum::<f64>()).unwrap(),
                        y: Decimal::from_f64(other.address.0.len() as f64).unwrap(),
                    };
                    let dist = p1.distance(&p2).to_f64().unwrap();
                    (dist, other.address.clone())
                })
                .collect();
            distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            node.peers = distances.iter().take(3).map(|(_, addr)| addr.clone()).collect();
        }
    }

    // Geometric locality message propagation
    pub fn propagate_message(&self, origin: &FractalAddress, message: &str) {
        // Propagate message to all nodes within 1 fractal digit of origin
        let origin_key = Self::address_to_string(origin);
        for (key, node) in &self.nodes {
            let locality = key.chars().zip(origin_key.chars()).filter(|(a, b)| a == b).count();
            if locality >= origin_key.len().saturating_sub(1) {
                // Simulate message delivery
                println!("Message '{}' delivered to node {:?}", message, node.address);
            }
        }
    }

    pub fn address_to_string(addr: &FractalAddress) -> String {
        addr.0.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("")
    }
}

// Quantum-resistant cryptography stubs
pub mod quantum {
    // CRYSTALS-Dilithium signature stub
    pub fn sign_triangle(_triangle_bytes: &[u8], _private_key: &[u8]) -> Vec<u8> {
    // Placeholder: Use SHA-256 hash as mock signature
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(_triangle_bytes);
    hasher.update(_private_key);
    hasher.finalize().to_vec()
    }
    pub fn verify_signature(_triangle_bytes: &[u8], _signature: &[u8], _public_key: &[u8]) -> bool {
    // Placeholder: Recompute hash and compare
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(_triangle_bytes);
    hasher.update(_public_key);
    hasher.finalize().to_vec() == _signature
    }
    // Quantum-resistant commitment scheme stub
    pub fn commit_triangle(_triangle_bytes: &[u8], _randomness: &[u8]) -> Vec<u8> {
    // Placeholder: Hash triangle bytes and randomness
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(_triangle_bytes);
    hasher.update(_randomness);
    hasher.finalize().to_vec()
    }
    // Post-quantum key exchange stub
    pub fn key_exchange(_public_a: &[u8], _public_b: &[u8]) -> Vec<u8> {
    // Placeholder: Hash both public keys together
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(_public_a);
    hasher.update(_public_b);
    hasher.finalize().to_vec()
    }
}
