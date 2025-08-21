#![no_main]

use ark_bn254::{Bn254, Fr};
use std::str::FromStr;


use crate::utils::perform_20_bn254_pairings_arks_patched;
sp1_zkvm::entrypoint!(main);

mod utils;

pub fn main() {
    // Compute the sum of the numbers.
    perform_20_bn254_pairings_arks_patched::<Bn254>();
}