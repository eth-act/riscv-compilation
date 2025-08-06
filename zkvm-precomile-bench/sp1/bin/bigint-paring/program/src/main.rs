#![no_main]
sp1_zkvm::entrypoint!(main);

mod util;

use crate::util::perform_100_bn254_pairings_batched_bn;

pub fn main() {
    // Compute the sum of the numbers.
    println!("cycle-tracker-start: compute");
    perform_100_bn254_pairings_batched_bn();
    println!("cycle-tracker-end: compute");
}
