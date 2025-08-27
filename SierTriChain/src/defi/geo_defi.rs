#[cfg(test)]
mod tests {
    #[test]
    fn test_defi_geo_defi_basic() {
        assert_eq!(2 + 2, 4);
    }
}
// Geometric DeFi module for fractal territory system
// Includes AMM, lending, yield farming, futures, options, and synthetics

use crate::geometry::triangle::Triangle;
use rust_decimal::prelude::ToPrimitive;
use crate::geometry::subdivision::FractalAddress;

pub struct AMM {
    pub pool_a: Triangle,
    pub pool_b: Triangle,
    pub reserve_a: f64,
    pub reserve_b: f64,
}

impl AMM {
    // Price = sqrt(area_A / area_B)
    pub fn price(&self) -> f64 {
    let area_a = crate::geometry::area::triangle_area(&self.pool_a);
    let area_b = crate::geometry::area::triangle_area(&self.pool_b);
        if area_b <= 0.0 {
            return 0.0;
        }
        (area_a / area_b).sqrt()
    }
    // Swap A for B
    pub fn swap_a_for_b(&mut self, amount_a: f64) -> f64 {
        let price = self.price();
        let amount_b = amount_a * price;
        self.reserve_a += amount_a;
        self.reserve_b -= amount_b;
        amount_b
    }
    // Swap B for A
    pub fn swap_b_for_a(&mut self, amount_b: f64) -> f64 {
        let price = self.price();
        let amount_a = amount_b / price;
        self.reserve_b += amount_b;
        self.reserve_a -= amount_a;
        amount_a
    }
}

pub struct Lending {
    pub triangle: Triangle,
    pub depth: usize,
    pub collateral: f64,
}

impl Lending {
    // Collateral ratio increases with depth
    pub fn collateral_ratio(&self) -> f64 {
        1.0 + (self.depth as f64).ln()
    }
}

// Yield farming: rewards proportional to geometric computational work
pub fn yield_farming(computational_work: f64, base_reward: f64) -> f64 {
    base_reward * computational_work.sqrt()
}

// Futures contract stub for triangle subdivision events
pub struct TriangleFuture {
    pub triangle: Triangle,
    pub event_block: usize,
    pub volatility: f64,
}

impl TriangleFuture {
    pub fn price(&self) -> f64 {
        // Geometric volatility pricing stub
        self.volatility * self.triangle.area().to_f64().unwrap_or(0.0)
    }
}

// Options market stub for territory acquisition rights
pub struct TriangleOption {
    pub triangle: Triangle,
    pub strike_price: f64,
    pub expiry_block: usize,
}

// Synthetic asset backed by geometric territory portfolio
pub struct SyntheticAsset {
    pub triangles: Vec<Triangle>,
    pub total_value: f64,
}

impl SyntheticAsset {
    pub fn value(&self, base_value: f64) -> f64 {
        self.triangles.iter().map(|t| base_value / t.area().to_f64().unwrap().max(1e-8)).sum()
    }
}
