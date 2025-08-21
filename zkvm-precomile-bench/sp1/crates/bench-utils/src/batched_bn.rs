use crate::init_rands_bn_batched;
use substrate_bn::{pairing, Fr, Group, G1, G2};

pub fn perform_20_bn254_pairings_bn() {
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
        let _alice_ss = pairing(bob_pk1, carol_pk2).pow(alice_sk);
        let _bob_ss = pairing(carol_pk1, alice_pk2).pow(bob_sk);
        let _carol_ss = pairing(alice_pk1, bob_pk2).pow(carol_sk);
    }
}
