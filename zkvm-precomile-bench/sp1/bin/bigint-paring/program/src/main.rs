#![no_main]
sp1_zkvm::entrypoint!(main);

mod util;

use crate::util::perform_100_bn254_pairings_batched_bn;

pub fn main() {
    perform_100_bn254_pairings_batched_bn();
}
