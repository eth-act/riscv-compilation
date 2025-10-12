pub mod openvm;
pub mod pico;
pub mod risc0;
pub mod sp1;

use zkvm_interface::{Input, zkVM};

pub(crate) fn run_guest_program(zkvm: impl zkVM) -> Result<(), Box<dyn std::error::Error>> {
    let io = Input::new();
    let _report = zkvm.execute(&io)?;
    let (_public_values, _proof, _report) = zkvm.prove(&io)?;

    Ok(())
}
