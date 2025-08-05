use substrate_bn::{Group, Fr, G1, G2, pairing};

pub fn init_rands() -> Vec<Fr> {
    let rng = &mut rand::thread_rng();
    let mut rands = Vec::new();
    for _ in 0..100 {
        rands.push(Fr::random(rng));
    }
    rands
}

pub fn perform_100_bn254_pairings() {
    let rands = init_rands();
    
    for r in rands {
        // initializing test private keys
        let alice_pk = r;
        let bob_pk = r + Fr::one();
        let charlie_pk = bob_pk + Fr::one();
        
        
        // Generate public keys in G1 and G2
        let (alice_pubk_g1, alice_pubk_g2) = (G1::one() * alice_pk, G2::one() * alice_pk);
        let (bob_pubk_g1, bob_pubk_g2) = (G1::one() * bob_pk, G2::one() * bob_pk);
        let (charlie_pubk_g1, charlie_pubk_g2) = (G1::one() * charlie_pk, G2::one() * charlie_pk);
        
        // Perform pairings
        let alice_pairing = pairing(alice_pubk_g1, alice_pubk_g2).pow(alice_pk);
        let bob_pairing = pairing(bob_pubk_g1, bob_pubk_g2).pow(bob_pk);
        let charlie_pairing = pairing(charlie_pubk_g1, charlie_pubk_g2).pow(charlie_pk);
        
        // Check if pairings are equal
        assert!(alice_pairing == bob_pairing && bob_pairing == charlie_pairing);
    }
}
