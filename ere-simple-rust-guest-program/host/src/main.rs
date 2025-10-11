use crate::zkvm_runners::{risc0::run_risc0, sp1::run_sp1};

mod zkvm_runners;

fn main() {
    run_sp1().unwrap();
    run_risc0().unwrap();
}
