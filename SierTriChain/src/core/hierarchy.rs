
//! Hierarchy utilities for fractal-based blockchain addressing.

use crate::geometry::subdivision::FractalAddress;

/// Calculate depth in fractal hierarchy from base-3 address
pub fn fractal_depth(address: &FractalAddress) -> usize {
	address.0.len()
}

/// Get position in hierarchy (as path string)
pub fn fractal_position(address: &FractalAddress) -> String {
	address.0.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("-")
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_fractal_depth() {
		let addr = FractalAddress(vec![0, 1, 2, 0]);
		assert_eq!(fractal_depth(&addr), 4);
	}
	#[test]
	fn test_fractal_position() {
		let addr = FractalAddress(vec![0, 1, 2]);
		assert_eq!(fractal_position(&addr), "0-1-2");
	}
}
