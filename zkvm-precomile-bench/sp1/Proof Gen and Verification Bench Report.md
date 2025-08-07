## **Technical Report: Proof Generation & Verification Benchmark of `bn256` Implementations in the SP1 zkVM**

This report extends the previous benchmark analysis to cover the entire lifecycle of Zero-Knowledge proof generation and verification. It evaluates the performance of two `U256` integer implementations for `bn256` elliptic curve operations within the SP1 zkVM, measuring the impact of specialized precompiles on the resource-intensive proving and verification stages.

**Date:** August 7, 2025
**Platform:** SP1 zkVM (CPU Prover)
**Hardware:** Apple MacBook Pro (M3 Max)


### **Objective**

The primary goal is to benchmark the end-to-end performance difference between the standard `substrate_bn` crate and a modified version using `crypto-bigint`. This analysis moves beyond simple execution cycle counts to measure the real-world time required for:
1.  **Proof Generation:** The computationally expensive process of creating a cryptographic proof of the guest program's execution.
2.  **Verification:** The process of validating the generated proof.

This provides a comprehensive view of how library choice and precompile optimizations affect the practical usability of ZK programs.



### **Methodology**

The experiment utilized the same tripartite Diffie-Hellman key exchange guest program as the previous benchmark to ensure a consistent computational workload. The SP1 toolchain was used to generate a proof for the execution of each of the four guest programs and then verify that proof.

#### **Experimental Configurations**

The four configurations remained identical to the initial report:
1.  **`bn-pairing`**: Standard `substrate_bn` crate, no precompiles.
2.  **`bigint-pairing`**: `substrate_bn` modified to use `crypto-bigint`, no precompiles.
3.  **`bn-pairing-patched`**: `substrate_bn` with the specialized `bn` precompile enabled.
4.  **`bigint-pairing-patched`**: `crypto-bigint` version with the generic `bigint` precompile enabled.



### **Results**

The total time taken for proof generation and verification for each configuration was recorded. The results, derived directly from the prover and verifier logs, are summarized below.

| Configuration | Base Crate | Precompile Enabled | Cycle Count | Proof Generation Time | Verification Time | Total Time |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `bn-pairing` | `substrate_bn` | No | 1,105,498,339 | ~4 hr 57 min | ~1 min 52 sec | **~4 hr 59 min** |
| `bigint-pairing` | `crypto-bigint` | No | 1,523,558,068 | ~4 hr 50 min | ~2 min 29 sec | **~4 hr 52 min** |
| `bigint-pairing-patched` | `crypto-bigint` | Yes (`bigint`) | 518,877,400 | ~3 hr 10 min | ~1 min 18 sec | **~3 hr 11 min** |
| **`bn-pairing-patched`** | **`substrate_bn`** | **Yes (`bn`)** | **40,014,404** | **~41 min 18 sec** | **~21 sec** | **~41 min 39 sec** |




### **Analysis & Discussion**

This end-to-end analysis confirms and amplifies the conclusions from the execution benchmark, revealing the profound impact of precompiles on prover performance.

#### **Without Precompiles: A Heavy Burden**

Both non-patched guest programs required an extremely long time to prove. The `bn-pairing` program took nearly **5 hours**, while the `bigint-pairing` program, despite having a higher cycle count, completed slightly faster at around **4 hours and 52 minutes**. This minor discrepancy suggests that cycle count alone is not a perfect predictor of proving time and that the structure of the execution trace can influence the prover's performance. Regardless, both times are prohibitively long for most practical applications.

#### **With Precompiles: From Impractical to Feasible**

The introduction of precompiles drastically reduced proving times:

* **`bigint` Precompile Impact**: Enabling the generic `bigint` precompile (`bigint-pairing-patched`) cut the total time down to **3 hours and 11 minutes**. While this is a significant **1.5x improvement**, the proving process remains lengthy.

* **`bn` Precompile Impact**: The specialized `bn` precompile (`bn-pairing-patched`) delivered a transformative result. It reduced the total time from nearly 5 hours to just **under 42 minutes**. This represents a staggering **~7x performance improvement** over the non-precompiled version and a **~4.6x improvement** over the version with only the generic `bigint` precompile.

#### **Verification Time**

Verification times followed a similar, though less dramatic, pattern. The `bn-pairing-patched` proof, generated from a much smaller and more efficient execution trace, was the fastest to verify at just **21 seconds**. The other proofs took between one and two-and-a-half minutes to verify, with the time generally correlating with the complexity and size of the original execution trace.



### **Conclusion**

While the initial benchmark demonstrated the execution efficiency of precompiles, this analysis proves their absolute necessity for the entire ZK lifecycle. The results unequivocally show that for complex cryptographic workloads like `bn256` pairings, **failing to use a specialized, high-level precompile renders the proving process practically infeasible on consumer hardware.**

The SP1 `bn` precompile reduces proof generation time from a multi-hour commitment to under an hour, crossing a critical threshold for developer productivity and application viability. For any project leveraging `bn256` operations in the SP1 zkVM, the adoption of the `sp1-patches/bn` precompile is the most critical optimization available and should be considered a mandatory step in development.


_Report structured by an LLM (Gemini)... :)_