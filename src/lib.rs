#![deny(unsafe_code)]
#![warn(missing_docs)]

mod macros;
pub mod vectors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
