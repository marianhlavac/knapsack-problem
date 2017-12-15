use rand::{thread_rng, Rng};

/// Creates a random population of specified size.
pub fn create_population(size: usize, in_size: usize) -> Vec<Vec<bool>> {
    (0..size).map(|_| { create_individual(in_size) }).collect()
}

pub fn sort_population<F>(population: &Vec<Vec<bool>>, fitness_fn: &F) -> Vec<Vec<bool>> 
where F: Fn(&Vec<bool>) -> usize {
    let mut sorted = population.clone();
    sorted.sort_by(|a, b| fitness_fn(a).cmp(&fitness_fn(b)));
    sorted
}

/// Creates a random individual of specified size.
pub fn create_individual(size: usize) -> Vec<bool> {
    (0..size).map(|_| { thread_rng().gen() }).collect()
}

/// Chooses a random individual from the population.
pub fn random_individual(population: &Vec<Vec<bool>>) -> Vec<bool> {
    let pop_size = population.len();
    population[thread_rng().gen_range(0, pop_size)].clone()
}

/// Probability operator
pub fn odds_are(probability: f32) -> bool {
    let randomness: f32 = thread_rng().gen_range(0.0, 1.0);
    randomness <= probability
}

/// Inspects the population in the current state,
/// outputing the results to the console.
pub fn inspect(generation: usize, population: Vec<Vec<bool>>) {
    
}