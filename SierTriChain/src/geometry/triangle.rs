use crate::geometry::point::Point;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub const GOLDEN_RATIO: Decimal = dec!(1.61803398875);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }

    pub fn area(&self) -> Decimal {
        let area = (self.a.x * (self.b.y - self.c.y) + self.b.x * (self.c.y - self.a.y) + self.c.x * (self.a.y - self.b.y)) / Decimal::new(2, 0);
        area.abs()
    }

    pub fn subdivide(&self) -> [Triangle; 4] {
        let mid_ab = self.a.midpoint(&self.b);
        let mid_bc = self.b.midpoint(&self.c);
        let mid_ca = self.c.midpoint(&self.a);

        [
            Triangle::new(self.a, mid_ab, mid_ca),
            Triangle::new(mid_ab, self.b, mid_bc),
            Triangle::new(mid_ca, mid_bc, self.c),
            Triangle::new(mid_ab, mid_bc, mid_ca),
        ]
    }
}

pub fn is_equilateral(tri: &Triangle, epsilon: Decimal) -> bool {
    let side1 = tri.a.distance(&tri.b);
    let side2 = tri.b.distance(&tri.c);
    let side3 = tri.c.distance(&tri.a);

    (side1 - side2).abs() < epsilon && (side2 - side3).abs() < epsilon
}

pub fn genesis_triangle() -> Triangle {
    Triangle::new(
        Point::new(dec!(0.0), dec!(0.0)),
        Point::new(dec!(1.0), dec!(0.0)),
        Point::new(dec!(0.5), dec!(0.86602540378)), // approx sqrt(3)/2
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::point::Point;
    use rust_decimal_macros::dec;

    #[test]
    fn test_triangle_creation() {
        let a = Point::new(dec!(0.0), dec!(0.0));
        let b = Point::new(dec!(1.0), dec!(0.0));
        let c = Point::new(dec!(0.5), dec!(1.0));
        let t = Triangle::new(a, b, c);
        assert_eq!(t.a.x, dec!(0.0));
        assert_eq!(t.b.x, dec!(1.0));
        assert_eq!(t.c.y, dec!(1.0));
    }

    #[test]
    fn test_triangle_area() {
        let a = Point::new(dec!(0.0), dec!(0.0));
        let b = Point::new(dec!(1.0), dec!(0.0));
        let c = Point::new(dec!(0.5), dec!(1.0));
        let t = Triangle::new(a, b, c);
        assert_eq!(t.area(), dec!(0.5));
    }

    #[test]
    fn test_subdivide() {
        let a = Point::new(dec!(0.0), dec!(0.0));
        let b = Point::new(dec!(2.0), dec!(0.0));
        let c = Point::new(dec!(1.0), dec!(2.0));
        let t = Triangle::new(a, b, c);
        let subdivided = t.subdivide();

        let mid_ab = a.midpoint(&b);
        let mid_bc = b.midpoint(&c);
        let mid_ca = c.midpoint(&a);

        assert_eq!(subdivided[0], Triangle::new(a, mid_ab, mid_ca));
        assert_eq!(subdivided[1], Triangle::new(mid_ab, b, mid_bc));
        assert_eq!(subdivided[2], Triangle::new(mid_ca, mid_bc, c));
        assert_eq!(subdivided[3], Triangle::new(mid_ab, mid_bc, mid_ca));
    }

    #[test]
    fn test_is_equilateral() {
        let a = Point::new(dec!(0.0), dec!(0.0));
        let b = Point::new(dec!(1.0), dec!(0.0));
        let c = Point::new(dec!(0.5), dec!(0.86602540378));
        let t = Triangle::new(a, b, c);
        assert!(is_equilateral(&t, dec!(1e-9)));
    }
}