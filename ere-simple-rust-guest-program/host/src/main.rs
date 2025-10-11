use std::path::Path;
use crate::zkvm_runners::sp1::run_sp1;

mod zkvm_runners;
mod utils;

fn main() {
    let guest_program_path =
        "../guest/target/riscv32im-unknown-none-elf/release/general-rust-guest-program";
    let path = Path::new(guest_program_path);
    run_sp1(&path).unwrap();
}
