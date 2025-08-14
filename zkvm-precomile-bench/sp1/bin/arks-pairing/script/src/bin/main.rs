//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "20")]
    n: u32,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        // println!("n: {}", n);
        // println!("a: {}", a);
        // println!("b: {}", b);

        // let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        // assert_eq!(a, expected_a);
        // assert_eq!(b, expected_b);
        // println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
        
        
        let total_compute_cycles_inv = report.cycle_tracker.get("compute-inverse").unwrap();
        let compute_invocation_count_inv = report.invocation_tracker.get("compute-inverse").unwrap();
        
        let total_compute_cycles_add = report.cycle_tracker.get("compute-add").unwrap();
        let compute_invocation_count_add = report.invocation_tracker.get("compute-add").unwrap();
        
        let total_compute_cycles_sub = report.cycle_tracker.get("compute-sub").unwrap();
        let compute_invocation_count_sub = report.invocation_tracker.get("compute-sub").unwrap();
        
        let total_compute_cycles_mul = report.cycle_tracker.get("compute-mul").unwrap();
        let compute_invocation_count_mul = report.invocation_tracker.get("compute-mul").unwrap();
        
        println!("Total compute cycles [inv]: {}", total_compute_cycles_inv);
        println!("Compute invocation count [inv]: {}", compute_invocation_count_inv);
        
        println!("Total compute cycles [add]: {}", total_compute_cycles_add);
        println!("Compute invocation count [add]: {}", compute_invocation_count_add);
        
        println!("Total compute cycles [sub]: {}", total_compute_cycles_sub);
        println!("Compute invocation count [sub]: {}", compute_invocation_count_sub);
        
        println!("Total compute cycles [mul]: {}", total_compute_cycles_mul);
        println!("Compute invocation count [mul]: {}", compute_invocation_count_mul);
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
