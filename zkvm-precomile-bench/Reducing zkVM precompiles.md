# Reducing the number of zkVM precompiles

- The goal of this project is to document the precomiles using in zkVMs today. 
- In terms of functionality, mark the precompile that is redundant.
- Show how much computational gains precomiples introduced compared to doing this the old fahions way.


These are the zkVM we would be consentration this efforts on;
1. RICS0
2. SP1 
3. Airbender 
4. Zisk
5. OpenVM 
6. Ziren
7. Pico



## RICS0
- BigInt
- SHA256

## SP1
- edwards add
- edwards decompress
- fp
- fp2 add sub
- fp2 mul
- keccak256
- sha256 (compress and extended)
- u256x2048
- u256
- weierstrass add
- weierstrass decompress
- weierstrass double


## Airbender
- BigInt (u256)
- Blake


## Zisk
- arith256
- arith256_mod
- secp256k1_add
- secp256k1_dbl
- secp256k1_x3
- secp256k1_y3
- keccakf
- sha256f

## OpenVM
- fp2 (addsub, muldiv)
- modular (addsub, muldiv)
- fp2 extension
- modular extension
- Bigint
- weierstrass (add_ne, dobule)
- weierstrass extension
- keccak256
- pairings
- sha256

## Ziren
- shaExtend
- shaCompress
- edAdd
- edDecompress
- keccakSponge
- secp256k1Add
- secp256k1Double
- secp256k1Decompress
- bn254Add
- bn254Double
- bls12381Decompress
- uint256Mul
- u256xu2048Mul
- bls12381Add
- bls12381Double
- bls12381FpAdd
- bls12381FpSub
- bls12381FpMul
- bls12381Fp2Add
- bls12381Fp2Sub
- bls12381Fp2Mul
- bn254FpAdd
- bn254FpSub
- bn254FpMul
- bn254Fp2Add
- bn254Fp2Sub
- bn254Fp2Mul
- secp256r1Add
- secp256r1Double
- secp256r1Decompress
- poseidon2Permute

## Pico
- shaExtend
- shaCompress
- edAdd
- edDecompress
- keccakPermute
- secp256k1Add
- secp256k1Double
- secp256k1Decompress
- bn254Add
- bn254Double
- bls12381Decompress
- uint256Mul
- bls12381Add
- bls12381Double
- bls12381FpAdd
- bls12381FpSub
- bls12381FpMul
- bls12381Fp2Add
- bls12381Fp2Sub
- bls12381Fp2Mul
- bn254FpAdd
- bn254FpSub
- bn254FpMul
- bn254Fp2Add
- bn254Fp2Sub
- bn254Fp2Mul
- secp256k1FpAdd
- secp256k1FpSub
- secp256k1FpMul
- poseidon2Permute