use ere_sp1::{EreSP1, compiler::RustRv32ima};
use std::path::Path;
use zkvm_interface::{Compiler, ProverResourceType};

use crate::zkvm_runners::run_guest_program;

pub(crate) fn run_sp1() -> Result<(), Box<dyn std::error::Error>> {
    let guest_directory = Path::new("../guest_programs/sp1");
    let program = RustRv32ima.compile(guest_directory)?;
    let zkvm = EreSP1::new(program, ProverResourceType::Cpu);
    run_guest_program(zkvm)
}
