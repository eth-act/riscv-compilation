# zkVM Precompile Benchmarking Project

## Overview

This repository contains research and benchmarking information about precompiles used in various Zero-Knowledge Virtual Machines (zkVMs). The primary goal is to analyze the performance impact of precompiles and identify opportunities for optimization and standardization across different zkVM implementations.

## Project Structure

- `Reducing zkVM precompiles.md`: Documents the precompiles used across various zkVMs (RISC0, SP1, Airbender, Zisk, OpenVM, Ziren, Pico), analyzing their functionality and identifying potential redundancies.

- `sp1/`: Contains benchmark reports for the SP1 zkVM:
  - `Execution Bench Report.md`: Performance comparison of different `bn254` implementations in SP1 zkVM, focusing on execution cycle counts.
  - `Proof Gen and Verification Bench Report.md`: Analysis of proof generation and verification times for different implementations.

## Key Findings

### Precompile Performance Impact

The benchmarks demonstrate the critical importance of specialized precompiles for zkVM performance:

1. **Execution Performance**: Specialized precompiles can improve execution speed by up to 27.6x compared to standard RISC-V execution.

2. **Proof Generation**: The difference becomes even more dramatic for proof generation, with specialized precompiles reducing time from nearly 5 hours to under 42 minutes (a 7x improvement).

3. **Implementation Choices**: The choice between different big integer libraries (e.g., `substrate_bn` vs. `crypto-bigint`) has some impact, but is overshadowed by the benefits of using appropriate precompiles.

### Precompile Landscape

The project maps out precompiles across seven different zkVMs, highlighting common functionality areas:

- Cryptographic primitives (SHA256, Keccak, Poseidon, Blake)
- Elliptic curve operations (various curves including secp256k1, bn254, BLS12-381)
- Big integer arithmetic
- Finite field arithmetic

## Research Objective

The ultimate goal of this project is to identify a minimal, standardized set of precompiles that would offer optimal performance while reducing implementation complexity across the zkVM ecosystem.