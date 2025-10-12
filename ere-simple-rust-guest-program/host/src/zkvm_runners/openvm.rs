use ere_openvm::{EreOpenVM, compiler::RustRv32ima};
use std::path::Path;
use zkvm_interface::{Compiler, ProverResourceType};

use crate::zkvm_runners::run_guest_program;

pub(crate) fn run_openvm() -> Result<(), Box<dyn std::error::Error>> {
    let guest_directory = Path::new("../guest_programs/openvm");
    let program = RustRv32ima.compile(guest_directory)?;
    let zkvm = EreOpenVM::new(program, ProverResourceType::Cpu)?;
    run_guest_program(zkvm)
}
