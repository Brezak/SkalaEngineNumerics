#![deny(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

pub mod vector;

use fixed::types::I32F32;
pub use vector::{Vec2, Vec3};

pub type SignedFractional = I32F32;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
