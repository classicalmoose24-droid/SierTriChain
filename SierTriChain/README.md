# SierTriChain

SierTriChain is a geometric blockchain engine leveraging triangle subdivision, lattice-based cryptography, and DeFi primitives. It organizes blockchain data as geometric structures and supports advanced cryptographic features for secure, scalable consensus and asset management.

## Overview
The Rust Geometry Engine is a precise geometric computation library built in Rust, focusing on triangle geometry. It utilizes the `rust_decimal` crate for exact calculations, ensuring mathematical perfection in geometric operations. The engine implements the Sierpinski subdivision algorithm, hierarchical triangle addressing, and various geometric validation functions.

## Features
- **Point Struct**: Represents a point in a 2D space with `rust_decimal` coordinates. Includes methods for calculating distances and midpoints.
- **Triangle Struct**: Represents a triangle defined by three vertices. Provides methods for calculating area, perimeter, and other geometric properties.
- **Sierpinski Subdivision**: Implements the Sierpinski triangle subdivision algorithm, generating child triangles while maintaining precision.
- **Hierarchical Addressing**: Supports hierarchical triangle addressing using dot notation (e.g., "0.1.2.3") for easy navigation and management.
- **Geometric Validation**: Includes functions to validate triangle integrity, check for equilateral properties, and ensure fractal ratios.
- **Area Calculations**: Implements area calculations using the cross product method and Heron's formula for accurate measurements.
- **Coordinate Transformations**: Provides functions for scaling and rotating triangles.
- **Geometric Hashing**: Implements deterministic hash functions to generate unique triangle IDs based on vertex coordinates.

## Getting Started

### Prerequisites
- Rust (version 1.50 or later)
- Cargo (Rust package manager)

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-geometry-engine.git
   cd rust-geometry-engine
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the project:
   ```
   cargo run
   ```

## Usage
To use the Rust Geometry Engine, you can import the necessary modules in your Rust application. The engine provides a comprehensive API for geometric operations, including creating points, triangles, and performing subdivisions.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for details.