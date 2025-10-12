## ERE Simple Rust Guest Program

A minimal project demonstrating how to run **Rust programs as RISC-V guests** inside **zkVMs** (specifically **SP1**, **PICO**, **OPENVM** and **RISC0**) using the ERE framework.


### Key Concepts

  * **Goal**: Run a `no_std` Rust application within a zero-knowledge virtual machine.
  * **Components**:
      * **Guest (`guest/`)**: Minimal Rust program compiled to **RISC-V** (`riscv32im-unknown-none-elf`). Features **atomic operations** and heap **vector allocations**.
      * **Host (`host/`)**: Loads the RISC-V binary, executes it in the chosen zkVM (via ERE), and generates a **cryptographic proof**.
  * **Support**: Targets **SP1**, **PICO**, **OPENVM** and **RISC0** using feature flags (`sp1-guest`, `pico-guest`, `openvm-guest`, `risc0-guest`).
  * **Runtime**: Includes a custom platform-specific runtime (entry point, allocator, ECALLs) for the `no_std` environment.


### Quick Start

1.  **Prerequisite**: 
    - Install RISC-V target:

    ```bash
    rustup target add riscv32im-unknown-none-elf
    ```
    
    - Install ERE framework:

    ```bash
    curl -L https://raw.githubusercontent.com/eth-act/ere/master/scripts/sdk_installers/install_risc0_sdk.sh | bash
    curl -L https://raw.githubusercontent.com/eth-act/ere/master/scripts/sdk_installers/install_sp1_sdk.sh | bash
    curl -L https://raw.githubusercontent.com/eth-act/ere/master/scripts/sdk_installers/install_pico_sdk.sh | bash
    curl -L https://raw.githubusercontent.com/eth-act/ere/master/scripts/sdk_installers/install_openvm_sdk.sh | bash
    ```
    

2.  **Build Guest sample command (e.g., for SP1)**:

    ```bash
    cd guest_programs/sp1
    cargo build --release
    ```

3.  **Run Host**:

    ```bash
    cd host
    cargo run --release sp1
    ```