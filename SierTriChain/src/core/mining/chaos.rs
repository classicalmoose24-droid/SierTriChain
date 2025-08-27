use crate::geometry::subdivision::{fractal_subdivide, FractalAddress};
use crate::geometry::triangle::Triangle;
use rust_decimal::Decimal;
use rand::Rng;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChaoticMiningResult {
    pub address: FractalAddress,
    pub triangle: Triangle,
    pub chaos_factor: f64,
}

/// Perform chaotic mining by introducing randomness into the mining process.
pub fn mine_chaos(depth: usize, threshold: Decimal) -> Option<ChaoticMiningResult> {
    let a = crate::geometry::point::Point::new(Decimal::ZERO, Decimal::ZERO);
    let b = crate::geometry::point::Point::new(Decimal::ONE, Decimal::ZERO);
    let c = crate::geometry::point::Point::new(Decimal::from_f64(0.5).unwrap(), Decimal::from_f64(0.86602540378).unwrap());
    let t = Triangle::new(a, b, c);

    let mut triangles = Vec::new();
    fractal_subdivide(&t, depth, FractalAddress(vec![]), &mut triangles);

    let mut rng = rand::thread_rng();
    for (triangle, address) in triangles {
        let chaos_factor: f64 = rng.gen();
        let effective_area = triangle.area() * Decimal::from_f64(chaos_factor).unwrap_or(Decimal::ONE);
        if effective_area < threshold {
            return Some(ChaoticMiningResult { address, triangle, chaos_factor });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    #[test]
    fn test_mine_chaos_returns_some() {
        let result = mine_chaos(2, dec!(0.5));
        assert!(result.is_some());
    }
    #[test]
    fn test_chaos_factor_in_range() {
        if let Some(res) = mine_chaos(2, dec!(0.5)) {
            assert!(res.chaos_factor >= 0.0 && res.chaos_factor <= 1.0);
        }
    }
}
