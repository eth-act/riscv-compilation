# RISC-V Compilation Research

This repository contains research projects focused on RISC-V architecture in blockchain contexts, particularly exploring execution environments for Ethereum and Zero-Knowledge Virtual Machines (zkVMs).

## Overview

The RISC-V architecture represents a significant advancement in open-source hardware design. This repository investigates how RISC-V can be leveraged for blockchain applications, with a dual focus:

1. Linux syscalls and execution clients in RISC-V environments
2. Performance benchmarking of zkVM precompiles 

## Project Structure

### Linux Syscalls Analysis (`/linux-syscalls`)

This project focuses on compiling Ethereum execution clients for the RISC-V architecture and analyzing syscalls during stateless execution:

- **Geth (Go Ethereum)**: Compilation and execution of the Go-based Ethereum Virtual Machine
- **Reth (Rust Ethereum)**: Implementation and analysis of the Rust-based Ethereum client
- **Nim**: Experiments with the Nim programming language on RISC-V

The goal is to understand system call patterns and optimize execution for RISC-V environments.

### zkVM Precompile Benchmarking (`/zkvm-precomile-bench`)

This project analyzes the performance impact of precompiles in various Zero-Knowledge Virtual Machines:

- Documents precompiles across multiple zkVMs (RISC0, SP1, Airbender, Zisk, OpenVM, Ziren, Pico)
- Provides detailed benchmarks showing execution and proof generation performance
- Analyzes the efficiency gains from specialized precompiles vs. standard implementations

Key findings show dramatic performance improvements (up to 27.6x faster execution and 7x faster proof generation) when using specialized precompiles.

## Research Goals

This repository serves several research objectives:

1. **Architecture Analysis**: Understanding how RISC-V performs in blockchain contexts
2. **Performance Optimization**: Identifying bottlenecks and optimization opportunities
3. **Standardization**: Working toward more standardized approaches for RISC-V in blockchain applications
4. **zkVM Efficiency**: Documenting and improving the efficiency of Zero-Knowledge proof generation on RISC-V

## Contribution

This is an ongoing research project. Findings, benchmarks, and analysis will be updated as research progresses.