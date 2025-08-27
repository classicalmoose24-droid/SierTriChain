use crate::geometry::triangle::Triangle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FractalAddress(pub Vec<u8>);

pub fn fractal_subdivide(
    triangle: &Triangle,
    depth: usize,
    address: FractalAddress,
    triangles: &mut Vec<(Triangle, FractalAddress)>,
) {
    if depth == 0 {
        triangles.push((*triangle, address));
        return;
    }

    let subdivided = triangle.subdivide();

    for i in 0..4 {
        let mut new_address = address.0.clone();
        new_address.push(i as u8);
        fractal_subdivide(&subdivided[i], depth - 1, FractalAddress(new_address), triangles);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::point::Point;
    use crate::geometry::triangle::Triangle;
    use rust_decimal_macros::dec;

    #[test]
    fn test_fractal_subdivide() {
        let a = Point::new(dec!(0.0), dec!(0.0));
        let b = Point::new(dec!(1.0), dec!(0.0));
        let c = Point::new(dec!(0.5), dec!(0.86602540378));
        let t = Triangle::new(a, b, c);

        let mut triangles = Vec::new();
        fractal_subdivide(&t, 1, FractalAddress(vec![]), &mut triangles);
        assert_eq!(triangles.len(), 4);

        let mut triangles = Vec::new();
        fractal_subdivide(&t, 2, FractalAddress(vec![]), &mut triangles);
        assert_eq!(triangles.len(), 16);
    }
}