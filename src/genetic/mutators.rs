use rand::Rng;

fn random_single(individual: mut BitVec) {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0, individual.len());
    
    individual.set(idx, !individual.get(idx));
}