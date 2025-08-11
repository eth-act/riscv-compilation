#![no_main]

use ark_bn254::Bn254;

use crate::utils::perform_20_bn254_pairings_arks_patched;
sp1_zkvm::entrypoint!(main);

mod utils;

pub fn main() {
    // Compute the sum of the numbers.
    println!("cycle-tracker-start: compute");
    perform_20_bn254_pairings_arks_patched::<Bn254>();
    println!("cycle-tracker-end: compute");
}
