pub mod crypto;
pub mod compound_proof;
pub mod drgraph;
pub mod error;
pub mod fr32;
pub mod gadgets;
pub mod hasher;
pub mod merkle;
pub mod parameter_cache;
pub mod proof;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
