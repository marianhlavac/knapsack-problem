mod crossover;
mod mutators;
mod selectors;
mod utils;

use bit_vec::BitVec;
use utils::{odds_are, random_individual, inspect};

/// Simulates an evolution with genetic algorithm.
pub fn simulate(pop_size: usize, fitness_fn: Fn(BitVec) -> f32, constr_fn: Fn(BitVec) -> bool) {
    let population = utils::create_population(pop_size);
    let xover_probability = 0.8;
    let mutation_probability = 0.3;
    let elitism = 0.1;
    
    // Run for a number of generations
    for i in 0..10 {
        // Selection
        new_population = selectors::best(population, elitism * pop_size);
        
        // Fill the rest of population with offsprings
        for o in 0..(pop_size - new_population.len()) {
            let mut child: BitVec;
            
            // Mate or select a random individual
            if odds_are(xover_probability) {
                // Select two random individuals
                let in1 = random_individual(population).clone();
                let in2 = random_individual(population).clone();
                
                // Crossover
                child = crossover::simple_half(in1, in2);
            } else {
                child = random_individual(population);
            }
            
            // Mutation
            if odds_are(mutation_probability) {
                mutators::random_single(child);
            }
            
            // Append to the rest of the poulation
            new_population.append(child);
        }
        
        // Replace the current population with new population
        population = new_population;
    }
}