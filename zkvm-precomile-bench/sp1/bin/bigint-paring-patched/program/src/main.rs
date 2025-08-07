#![no_main]
sp1_zkvm::entrypoint!(main);

use bench_utils::perform_20_bn254_pairings_bigint;

pub fn main() {
    // Compute the sum of the numbers.
    println!("cycle-tracker-start: compute");
    perform_20_bn254_pairings_bigint();
    println!("cycle-tracker-end: compute");
}
