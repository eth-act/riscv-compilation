## **Technical Report: Performance Benchmark of `bn254` Implementations in the SP1 zkVM**

This report details a performance comparison between two underlying `U256` integer implementations for `bn254` elliptic curve operations within the Succinct SP1 Zero-Knowledge Virtual Machine (zkVM). The experiment benchmarks the performance with and without specialized SP1 precompiles to quantify their impact on execution efficiency.

**Date:** August 7, 2025
**Platform:** SP1 zkVM
**Hardware:** Apple MacBook Pro (M3 Max)



### **Objective**

The primary goal of this experiment is to benchmark the performance difference between:

1.  The standard `substrate_bn` crate, which uses its own internal `U256` type for `bn254` curve operations.
2.  A modified version of `substrate_bn` that uses the `U256` type from the popular `crypto-bigint` crate.

This comparison is conducted in two scenarios: one relying on standard RISC-V execution and another leveraging SP1's specialized precompiles for cryptographic operations. The key metric for performance is the **total cycle count** required to execute the guest program.



### **Methodology**

A test program simulating a tripartite Diffie-Hellman key exchange was developed to provide a consistent workload involving common elliptic curve operations.

#### **Guest Program Logic**

The core logic, executed inside the zkVM, performs the following steps in a loop:

1.  Generates three private keys ($sk\_A, sk\_B, sk\_C$).
2.  Calculates the corresponding public keys in groups $G\_1$ and $G\_2$ via scalar multiplication (e.g., $pk\_{A1} = G\_1 \\cdot sk\_A$).
3.  Computes a shared secret using a combination of pairing operations and exponentiation in the target group $G\_T$. For example, Alice computes her shared secret as $ss\_A = e(pk\_{B1}, pk\_{C2})^{sk\_A}$.
4.  Asserts that all three computed shared secrets are equal, verifying the correctness of the cryptography.

<!-- end list -->

```rust
// General Guest Program for bn254 Key Exchange
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

Four distinct guest programs were benchmarked:

1.  **`bn-pairing`**: Uses the standard `substrate_bn` crate without any precompile optimizations.
2.  **`bigint-pairing`**: Uses the modified `substrate_bn` crate, which relies on `crypto-bigint` for `U256` operations, also without precompiles.
3.  **`bn-pairing-patched`**: Identical to `bn-pairing` but enables SP1's "fat" `bn` precompile, designed to accelerate the full suite of `bn254` operations.
      * `Cargo.toml` patch: `substrate-bn = { git = "https://github.com/sp1-patches/bn", ... }`
4.  **`bigint-pairing-patched`**: Identical to `bigint-pairing` but enables a SP1 `bigint` precompile to accelerate low-level `U256` arithmetic.
      * `Cargo.toml` patch: `crypto-bigint = { git = "https://github.com/CoinDao/RustCrypto-bigint", ... }`


### **Results**

The execution cycle counts for each of the four configurations are summarized below. Lower cycle counts indicate higher performance.

| Configuration              | Base Crate      | Precompile Enabled | Cycle Count     | Execution Time |
| -------------------------- | --------------- | ------------------ | --------------- | -------------- |
| `bn-pairing`               | `substrate_bn`  | No                 | 1,105,498,339   | 26.8 s         |
| `bigint-pairing`           | `crypto-bigint` | No                 | 1,523,558,068   | 26.0 s         |
| **`bn-pairing-patched`** | **`substrate_bn`** | **Yes (`bn`)** | **40,014,404** | **2.16 s** |
| `bigint-pairing-patched`   | `crypto-bigint` | Yes (`bigint`)     | 518,877,400     | 11.8 s         |


### **Analysis & Discussion**

The results provide clear insights into the performance characteristics of both the underlying libraries and the effectiveness of specialized precompiles.

#### **Without Precompiles: `substrate_bn` vs. `crypto-bigint`**

In the standard execution environment, the `bn-pairing` program (1.11 billion cycles) was approximately **38% faster** than the `bigint-pairing` program (1.52 billion cycles). This suggests that the native `U256` implementation within `substrate_bn` is more efficient for this specific workload when compiled to standard RISC-V instructions, possibly due to fewer abstractions or a more direct implementation.

#### **With Precompiles: The Power of Specialization**

The impact of precompiles is dramatic:

  * **`bn` Precompile Impact**: Enabling the `bn` precompile (`bn-pairing-patched`) reduced the cycle count from 1.11 billion to just **40 million**, a **\~27.6x performance improvement**. This demonstrates the immense value of a "fat" precompile that accelerates not just integer math, but the entire high-level elliptic curve group and field logic.

  * **`bigint` Precompile Impact**: The generic `bigint` precompile (`bigint-pairing-patched`) also provided a benefit, reducing the cycle count from 1.52 billion to 519 million—a **\~2.9x improvement**. However, its impact is far more limited because it only accelerates the fundamental `U256` arithmetic, leaving the complex and expensive curve-specific logic to be executed as regular, less efficient instructions.

When comparing the two patched versions, the `bn-pairing-patched` configuration is nearly **13 times faster** than `bigint-pairing-patched` (40M vs. 519M cycles). This is the most crucial finding of the experiment.



### **Conclusion**

The benchmark results lead to a clear and decisive conclusion: **specialized, high-level precompiles are critical for achieving performance in zkVMs.**

While the choice of the underlying big integer library has a moderate impact on performance in a vanilla environment, it is completely overshadowed by the use of an appropriate precompile. The SP1 `bn` precompile, which is specifically optimized for the `bn254` curve, offers an order-of-magnitude performance gain over both non-precompiled code and code using only generic `bigint` acceleration.

For developers building ZK applications on SP1 that involve `bn254` cryptography, **employing the `sp1-patches/bn` precompile is not just an optimization but a necessity for creating efficient and practical programs.**




_Report structured by an LLM (Gemini)... :)_