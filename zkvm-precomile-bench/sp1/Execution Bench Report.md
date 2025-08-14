### **Technical Report: Performance Benchmark of bn256 Implementations in the SP1 zkVM**

This report details a performance comparison of different Rust libraries for `bn256` elliptic curve operations within the Succinct SP1 Zero-Knowledge Virtual Machine (zkVM). The experiment benchmarks the performance of three distinct cryptographic libraries (`substrate_bn`, `crypto-bigint`, and `arkworks`) and quantifies the impact of specialized SP1 precompiles on execution efficiency.

**Date:** August 8, 2025
**Platform:** SP1 zkVM
**Hardware:** Apple MacBook Pro (M3 Max)



### **Objective**

The primary goal of this expanded experiment is to benchmark the performance difference between:

  * The standard `substrate_bn` crate.
  * A modified `substrate_bn` that uses the `U256` type from `crypto-bigint`.
  * The `ark_bn254` crate from the `arkworks` ecosystem.

This comparison is conducted with and without SP1's specialized precompiles to measure their impact. The key performance metric is the total cycle count required to execute the guest program.



### **Methodology**

A test program simulating a tripartite Diffie-Hellman key exchange was used to create a consistent workload involving common elliptic curve operations like scalar multiplication and pairings.

#### **Guest Program Logic**

The core logic, executed inside the zkVM, performs the following steps in a loop:

1.  Generates three private keys ($sk\_A, sk\_B, sk\_C$).
2.  Calculates the corresponding public keys in groups $G\_1$ and $G\_2$ via scalar multiplication (e.g., $pk\_{A1} = G\_1 \\cdot sk\_A$).
3.  Computes a shared secret using a combination of pairing operations and exponentiation in the target group $G\_T$. For example, Alice computes her shared secret as $ss\_A = e(pk\_{B1}, pk\_{C2})^{sk\_A}$.
4.  Asserts that all three computed shared secrets are equal, verifying the correctness of the cryptography.



```rust
// General Guest Program for bn256 Key Exchange
pub fn general_guest_program() {
    let rands = init_rands_bn_batched();

    for rand in rands {
        // Generate private keys
        let alice_sk = rand;
        let bob_sk = rand + Fr::one();
        let carol_sk = bob_sk + Fr::one();

        // Generate public keys in G1 and G2
        let (alice_pk1, alice_pk2) = (G1::one() * alice_sk, G2::one() * alice_sk);
        let (bob_pk1, bob_pk2) = (G1::one() * bob_sk, G2::one() * bob_sk);
        let (carol_pk1, carol_pk2) = (G1::one() * carol_sk, G2::one() * carol_sk);

        // Each party computes the shared secret
        let alice_ss = pairing(bob_pk1, carol_pk2).pow(alice_sk);
        let bob_ss = pairing(carol_pk1, alice_pk2).pow(bob_sk);
        let carol_ss = pairing(alice_pk1, bob_pk2).pow(carol_sk);

        assert!(alice_ss == bob_ss && bob_ss == carol_ss);
    }
}
```

#### **Experimental Configurations**

Five distinct guest programs were benchmarked:

1.  **bn-pairing**: Uses the standard `substrate_bn` crate without precompile optimizations.
2.  **bigint-pairing**: Uses a modified `substrate_bn` crate that relies on `crypto-bigint` for $U256$ operations, also without precompiles.
3.  **ark-pairing**: Uses the `ark_bn254` crate from `arkworks` without precompile optimizations.
4.  **bn-pairing-patched**: Identical to `bn-pairing` but enables SP1's "fat" `bn` precompile, which accelerates the full suite of `bn256` operations.
5.  **bigint-pairing-patched**: Identical to `bigint-pairing` but enables a generic `bigint` precompile to accelerate low-level $U256$ arithmetic.
6.  **ark-pairing-patched**: This is Identical to the `ark-pairing` guest program, but the bigint operation swapped with `sp1::mul_mod` precompiles where is can be applied, this should cut do the execution cycle count. 


### **Results**

The execution cycle counts and times for each configuration are summarized below. **Lower cycle counts indicate higher performance.**

| Configuration | Base Crate | Precompile Enabled | Cycle Count | Execution Time |
| :--- | :--- | :--- | :--- | :--- |
| `bn-pairing` | `substrate_bn` | No | 1,105,498,339 | 26.8 s |
| `bigint-pairing` | `crypto-bigint` | No | 1,523,558,068 | 26.0 s |
| `ark-pairing` | `ark_bn254` | No | **428,207,591** | **7.96 s** |
| `bigint-pairing-patched` | `crypto-bigint` | Yes (`bigint`) | 518,877,400 | 11.8 s |
| `ark-pairing-patched` | `ark_bn254` | Yes (`ff-bigint`) | **466,770,300** | **9.03 s** |
| `bn-pairing-patched` | `substrate_bn` | Yes (`bn`) | **40,014,404** | **2.16 s** |


### **Analysis & Discussion**

The results provide clear insights into the performance characteristics of the libraries and the effectiveness of different optimization strategies within the zkVM.

#### **Performance Without Precompiles**

In the standard RISC-V execution environment, the choice of library has a profound impact on performance:

  * The **`ark-pairing`** program (428M cycles) was the clear winner, proving to be approximately **2.6x faster** than `bn-pairing` (1.1B cycles) and **3.5x faster** than `bigint-pairing` (1.5B cycles). This indicates that the `arkworks` implementation of `bn254` is highly optimized for this workload, even without zkVM-specific accelerations.
  * The original `substrate_bn` implementation was \~38% faster than the version modified to use `crypto-bigint`, suggesting its native integer arithmetic is more efficient for this use case.

#### **The Power of Precompiles: Specialization vs. Generalization**

The impact of precompiles is dramatic and highlights a key tradeoff between specialized and generalized acceleration.

  * **"Fat" `bn` Precompile:** Enabling the `bn` precompile (`bn-pairing-patched`) reduced the cycle count from 1.1B to just **40 million**, a staggering **\~27.6x performance improvement**. This demonstrates the immense value of a high-level, "fat" precompile that accelerates not just integer math, but the entire elliptic curve group and field logic.

  * **Generic `bigint` Precompile:** The generic `bigint` precompile (`bigint-pairing-patched`) also provided a benefit, cutting cycles from 1.5B to 519M—a **\~2.9x improvement**. However, its impact is limited because it only accelerates the fundamental $U256$ arithmetic, leaving the complex and expensive curve-specific logic to be executed as regular, less efficient instructions.

#### **Key Insight: Optimized Library vs. Generic Precompile**

The most surprising and crucial finding of this expanded research is the comparison between `ark-pairing` and `bigint-pairing-patched`.

  * The `ark-pairing` program, with **no precompiles** (428M cycles), was **\~1.2x faster** than the `bigint-pairing-patched` program (519M cycles), which **used a precompile**.

This result strongly suggests that a well-optimized, general-purpose cryptographic library (`arkworks`) can be more performant than a less-optimized library that is only aided by generic, low-level precompiles. High-level algorithmic optimizations within the library itself proved more valuable than just accelerating the underlying integer math.


### **Conclusion**

This benchmark analysis leads to two primary conclusions for developers building ZK applications on SP1:

1.  **Specialized Precompiles are Supreme:** The most effective path to performance is using specialized, high-level precompiles. The SP1 `bn` precompile delivered an order-of-magnitude performance gain that is unmatched by any other method. For applications involving `bn256`, using this precompile is essential for efficiency.

2.  **Library Choice is Critical:** In the absence of a specialized precompile, the choice of the underlying cryptographic library is paramount. A highly optimized, general-purpose library like **`arkworks`** can provide significant performance benefits, even outperforming other libraries that have been augmented with generic, low-level arithmetic precompiles.

Therefore, the recommended strategy for developers is to **prioritize specialized, high-level precompiles** whenever available. If such a precompile does not exist for a required cryptographic primitive, selecting a modern, highly-optimized library is the next best step for achieving performant and practical ZK programs.



_Report structured by an LLM (Gemini)... :)_