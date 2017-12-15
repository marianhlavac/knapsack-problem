use rand::{thread_rng, Rng};

pub fn random_single(individual: &mut Vec<bool>) {
    let idx = thread_rng().gen_range(0, individual.len());
    
    individual[idx] = !individual[idx];
}