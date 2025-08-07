# RISC-V Compilation

This repository contains projects focused on compiling Ethereum execution clients for the RISC-V architecture and executing transactions/blocks statelessly. The primary goal is to analyze the syscalls employed during stateless execution on RISC-V hardware.

## Project Structure

This repository contains two main projects:

### 1. Geth (Go Ethereum)

Located in the `/geth` directory, this project focuses on:
- Compiling the `evm` binary for RISC-V architecture
- Executing transactions statelessly
- Analyzing syscalls during execution

See the [Geth README](./geth/README.md) for detailed instructions.

### 2. Reth (Rust Ethereum)

Located in the `/reth` directory, this project focuses on:
- Compiling a Rust binary for RISC-V architecture
- Providing a stateless block execution environment
- Analyzing syscalls during execution

See the [Reth README](./reth/README.md) for detailed instructions.

## Getting Started

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/riscv-compilation.git
   cd riscv-compilation
   ```

2. Choose which project to work with:
   - For Geth EVM: Follow instructions in [geth/README.md](./geth/README.md)
   - For Reth: Follow instructions in [reth/README.md](./reth/README.md)

## Running on Actual RISC-V Hardware

Since actual RISC-V hardware is still relatively uncommon, most development and testing is done using emulation.

For emulating a RISC-V environment, see:
- [Ubuntu RISC-V Boards Documentation](https://canonical-ubuntu-boards.readthedocs-hosted.com/en/latest/how-to/qemu-riscv/)

## Analyzing Syscalls

Both projects include instructions for analyzing syscalls during execution:

```bash
# For Geth
strace -o geth_trace.log ./evm [command]

# For Reth
strace -o reth_trace.log ./exec-block
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.