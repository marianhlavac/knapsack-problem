use bit_vec::BitVec;
use rand::Rng;

fn tournament(population: Vec<BitVec>, pool_size: usize = 4) {
    let pop_size = population.len();
    let mut pool: BitVec;
    
    // Select random individuals
    for _ in 0..pool_size {
        pool.push(random_individual(population));
    }
    
    // Let them fight! (sort individuals by fitness rank)
    ///...
}

fn best(population: Vec<BitVec>, count: usize, fitness_fn: Fn(BitVec) -> f32) -> Vec<BitVec> {
    let mut sorted_pop = sort_population(population, fitness_fn);
    sorted_pop.split_off(count);
    sorted_pop
}