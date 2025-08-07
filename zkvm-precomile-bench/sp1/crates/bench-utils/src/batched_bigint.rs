use bn_with_bigint_operational::{Fr, G1, G2, Group, pairing};

use crate::init_rands_biginit_batched;


pub fn perform_20_bn254_pairings_bigint() {
    let rands = init_rands_biginit_batched();
    
    
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
    
        // assert!(alice_ss == bob_ss && bob_ss == carol_ss);
    }
}