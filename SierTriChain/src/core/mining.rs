use crate::geometry::subdivision::{fractal_subdivide, FractalAddress};
use crate::geometry::triangle::Triangle;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MiningResult {
    pub address: FractalAddress,
    pub triangle: Triangle,
}

pub fn mine_genesis(depth: usize, threshold: Decimal) -> Option<MiningResult> {
    let a = crate::geometry::point::Point::new(dec!(0.0), dec!(0.0));
    let b = crate::geometry::point::Point::new(dec!(1.0), dec!(0.0));
    let c = crate::geometry::point::Point::new(dec!(0.5), dec!(0.86602540378));
    let t = Triangle::new(a, b, c);

    let mut triangles = Vec::new();
    fractal_subdivide(&t, depth, FractalAddress(vec![]), &mut triangles);

    for (triangle, address) in triangles {
        if triangle.area() < threshold {
            return Some(MiningResult { address, triangle });
        }
    }
    None
}

pub fn required_fractal_depth(block_height: usize, initial_depth: usize, adjustment_interval: usize) -> usize {
    initial_depth + (block_height / adjustment_interval)
}