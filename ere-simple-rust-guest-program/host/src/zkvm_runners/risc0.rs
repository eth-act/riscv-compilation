use ere_risc0::{EreRisc0, compiler::RustRv32ima};
use std::path::Path;
use zkvm_interface::{Compiler, ProverResourceType};

use crate::zkvm_runners::run_guest_program;

pub(crate) fn run_risc0() -> Result<(), Box<dyn std::error::Error>> {
    let guest_directory = Path::new("../guest_programs/risc0");
    let program = RustRv32ima.compile(guest_directory)?;
    let zkvm = EreRisc0::new(program, ProverResourceType::Cpu)?;
    run_guest_program(zkvm)
}
