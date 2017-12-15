use rand::Rng;

let mut grng = rand::thread_rng();

/// Creates a random population of specified size.
fn create_population(size: usize, in_size: usize) -> Vec<BitVec> {
    (0..size).map(|_| { create_individual(in_size) }).collect();
}

fn sort_population(population: Vec<BitVec>, fitness_fn: Fn(BitVec) -> f32) -> BitVec {
    population.clone().sort_by(|a, b| compare_fitness(a, b, fitness_fn))
}

/// Creates a random individual of specified size.
fn create_individual(size: usize) -> BitVec {
    BitVec::from_fn(size, |i| { grng.gen() })
}

/// Compares the fitness rank of two individuals.
fn compare_fitness(a: BitVec, b: BitVec, fitness_fn: Fn(BitVec) -> f32) -> bool {
    fitness_fn(a).cmp(fitness_fn(b))
}

/// Chooses a random individual from the population.
fn random_individual(population: Vec<BitVec>) -> BitVec {
    let pop_size = population.len();
    population[grng.gen_range(0, pop_size)]
}

/// Probability operator
fn odds_are(probability: f32) -> bool {
    let randomness: f32 = grng.gen_range(0, 1);
    randomness <= probability
}

/// Inspects the population in the current state,
/// outputing the results to the console.
fn inspect(generation: usize, population: Vec<BitVec>) {
    
}