### **Technical Report: End-to-End ZK Proof Benchmark of bn256 Implementations in the SP1 zkVM**

This report extends the previous benchmark analyses to cover the entire Zero-Knowledge proof lifecycle, from execution to verification. It evaluates the performance of three distinct `bn256` libraries within the SP1 zkVM, measuring the impact of cryptographic library choice and specialized precompiles on the resource-intensive proving and verification stages.

**Date:** August 8, 2025
**Platform:** SP1 zkVM (CPU Prover)
**Hardware:** Apple MacBook Pro (M3 Max)



### **Objective**

The goal is to benchmark the end-to-end performance of ZK applications built with different underlying cryptographic libraries. This analysis moves beyond execution cycle counts to measure the real-world time required for:
* **Proof Generation:** The computationally expensive process of creating a cryptographic proof of the guest program's execution.
* **Verification:** The process of validating the generated proof.

This provides a comprehensive view of how library optimization and zkVM precompiles affect the practical viability and developer experience of building with Zero-Knowledge proofs.


### **Methodology**

The experiment used the same tripartite Diffie-Hellman key exchange guest program as the previous benchmarks to ensure a consistent computational workload. The SP1 toolchain was used to execute each guest program, generate a proof of that execution, and subsequently verify the proof.

#### **Experimental Configurations**

The five configurations from the updated execution benchmark were used:
1.  **bn-pairing:** Standard `substrate_bn` crate, no precompiles.
2.  **bigint-pairing:** `substrate_bn` modified to use `crypto-bigint`, no precompiles.
3.  **ark-pairing:** Uses the `ark_bn254` crate from the `arkworks` ecosystem, no precompiles.
4.  **bn-pairing-patched:** `substrate_bn` with the specialized `bn` precompile enabled.
5.  **bigint-pairing-patched:** `crypto-bigint` version with the generic `bigint` precompile enabled.
6.  **ark-pairing-patched**: This is Identical to the `ark-pairing` guest program, but the bigint operation swapped with `sp1::mul_mod` precompiles where is can be applied, this should cut do the execution cycle count. 



### **Results**

The total time for proof generation and verification was recorded for each configuration. The results, derived from the prover and verifier logs, are summarized below. **Shorter times indicate better performance.**

| Configuration | Base Crate | Precompile Enabled | Cycle Count | Proof Generation Time | Verification Time | Total Time |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `bigint-pairing` | `crypto-bigint` | No | 1,523,558,068 | ~4 hr 50 min | ~2 min 29 sec | ~4 hr 52 min |
| `bn-pairing` | `substrate_bn` | No | 1,105,498,339 | ~4 hr 56 min | ~1 min 52 sec | ~4 hr 58 min |
| `bigint-pairing-patched` | `crypto-bigint` | Yes (`bigint`) | 518,877,400 | ~3 hr 16 min | ~1 min 19 sec | ~3 hr 17 min |
| `ark-pairing-patched` | `ark_bn254` | **Yes** | **422,122,898** | **~1 hr 23 min** | **~44 sec** | **~1 hr 24 min** |
| `ark-pairing` | `ark_bn254` | **No** | **428,207,591** | **~1 hr 17 min** | **~44 sec** | **~1 hr 18 min** |
| `bn-pairing-patched` | `substrate_bn` | Yes (`bn`) | **40,014,404** | **~40 min 47 sec** | **~21 sec** | **~41 min 8 sec** 🏆 |


### **Analysis & Discussion**

This end-to-end analysis confirms and amplifies the conclusions from the execution benchmarks, revealing the profound impact of both library choice and precompiles on prover performance.

#### **Without Precompiles: Prohibitively Expensive**
Both `bn-pairing` and `bigint-pairing` required an extremely long time to prove, taking nearly **5 hours** each. These times are prohibitively long for almost any practical application or development cycle. Interestingly, `bigint-pairing`, despite having ~38% more execution cycles, proved slightly faster, suggesting that the raw instruction count is not the only factor influencing prover performance; the structure of the execution trace also plays a role.

#### **The Impact of Optimizations: A Tale of Two Strategies**
The introduction of `arkworks` and SP1 precompiles showcases two powerful but distinct optimization strategies.

* **"Fat" `bn` Precompile (The Winner 🏆):** The `bn-pairing-patched` configuration delivered a transformative result. It reduced the total time from nearly 5 hours to just **~41 minutes**. This represents a staggering **~7.2x performance improvement** over its non-precompiled version. This is the gold standard, demonstrating that a specialized, high-level precompile that accelerates the entire cryptographic protocol is the ultimate performance unlock.

* **Optimized Library (`arkworks`) 🤯:** The `ark-pairing` result is the most significant new finding. With **no precompiles**, it generated a proof in just **~1 hour and 17 minutes**. This is over **3.8x faster** than the other non-precompiled libraries. This proves that a well-engineered, algorithmically-optimized cryptographic library can drastically reduce the proving burden on its own.

#### **Key Insight: Optimized Library vs. Generic Precompile**

The most crucial comparison is between `ark-pairing` and `bigint-pairing-patched`.

* The `ark-pairing` program (**no precompile**) was approximately **2.7x faster to prove** than `bigint-pairing-patched` (**with a generic `bigint` precompile**).



This result is unequivocal: a superior, general-purpose library is far more valuable than a less-optimized library augmented with only low-level, generic precompiles. Accelerating just the basic integer math is not enough; the high-level optimizations within the `arkworks` library had a much larger impact on reducing the overall complexity of the execution trace for the prover.



### **Conclusion**

This end-to-end analysis proves that both specialized precompiles and highly-optimized libraries are essential for the practical application of Zero-Knowledge proofs.

1.  **Specialized Precompiles Are Mandatory for Peak Performance:** For complex cryptographic workloads like `bn256` pairings, the SP1 `bn` precompile is not just an optimization—it's what makes the technology feasible on consumer hardware. It reduces proof generation from a multi-hour commitment to under an hour, crossing a critical threshold for usability.

2.  **A High-Quality Library is a "Game-Changer":** In the absence of a specialized, high-level precompile, the choice of cryptographic library is the single most important factor. The `arkworks` library provided a massive performance boost that **surpassed even the gains from a generic, low-level precompile.**

For developers building in the SP1 zkVM, the recommendation is clear: prioritize the use of specialized precompiles like `sp1-patches/bn` whenever possible. If one is not available for your specific use case, selecting a modern, highly-optimized library like `arkworks` is the next most critical step to ensure manageable and efficient proof generation.