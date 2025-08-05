use substrate_bn::Fr;

pub fn init_rands() -> Vec<Fr> {
    let rng = &mut rand::thread_rng();
    let mut rands = Vec::new();
    for _ in 0..100 {
        // println!("{}", Fr::random(rng));
        rands.push(Fr::random(rng));
    }
    rands
}