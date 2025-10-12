use crate::zkvm_runners::{openvm::run_openvm, pico::run_pico, risc0::run_risc0, sp1::run_sp1};
use std::env;

mod zkvm_runners;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <zkvm>", args[0]);
        eprintln!("Available zkvm options: sp1, risc0, openvm, pico");
        std::process::exit(1);
    }

    let zkvm = args[1].to_lowercase();

    let result = match zkvm.as_str() {
        "sp1" => run_sp1(),
        "risc0" => run_risc0(),
        "openvm" => run_openvm(),
        "pico" => run_pico(),
        _ => {
            eprintln!("Unknown zkvm: {}", zkvm);
            eprintln!("Available options: sp1, risc0, openvm, pico");
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error running {}: {:?}", zkvm, e);
        std::process::exit(1);
    }

    println!("Successfully executed {} zkvm", zkvm);
}
