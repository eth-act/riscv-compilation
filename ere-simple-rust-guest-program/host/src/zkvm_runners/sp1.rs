use std::path::Path;
use ere_sp1::EreSP1;
use zkvm_interface::{Input, ProverResourceType, zkVM};

use crate::utils::load_compiled_guest_program;

pub(crate) fn run_sp1(guest_program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Guest program loading
    let program = load_compiled_guest_program(guest_program_path)?;

    // Create Ere(SP1) zkVM instance
    let zkvm = EreSP1::new(program, ProverResourceType::Cpu);

    // Prepare inputs
    let io = Input::new();

    // Execute
    let _report = zkvm.execute(&io)?;

    // Prove
    let (_public_values, _proof, _report) = zkvm.prove(&io)?;

    Ok(())
}
