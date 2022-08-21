#![deny(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

//! Small numerics library
//!
//! SkalaEngineNumerics is a 'small' library of numeric types for use in [`SkalaEngine`](https://github.com/Brezak/SkalaEngine)

/// Vector types
pub mod vector;

use fixed::types::I32F32;
pub use vector::{Vec2, Vec3};

/// The current type backing all the numbers in the crate (may switch to a floats in the future)
pub type SignedFractional = I32F32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
