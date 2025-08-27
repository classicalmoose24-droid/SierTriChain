use rust_decimal::{Decimal, MathematicalOps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: Decimal,
    pub y: Decimal,
}

impl Point {
    pub fn new(x: Decimal, y: Decimal) -> Self {
        Point { x, y }
    }

    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / Decimal::new(2, 0),
            y: (self.y + other.y) / Decimal::new(2, 0),
        }
    }

    pub fn distance(&self, other: &Point) -> Decimal {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_point_creation() {
        let p = Point::new(dec!(1.0), dec!(2.0));
        assert_eq!(p.x, dec!(1.0));
        assert_eq!(p.y, dec!(2.0));
    }

    #[test]
    fn test_midpoint() {
        let p1 = Point::new(dec!(0.0), dec!(0.0));
        let p2 = Point::new(dec!(2.0), dec!(2.0));
        let mid = p1.midpoint(&p2);
        assert_eq!(mid, Point::new(dec!(1.0), dec!(1.0)));
    }

    #[test]
    fn test_distance() {
        let p1 = Point::new(dec!(0.0), dec!(0.0));
        let p2 = Point::new(dec!(3.0), dec!(4.0));
        assert_eq!(p1.distance(&p2), dec!(5.0));
    }
}