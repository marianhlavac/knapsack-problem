mod crossover;
mod mutators;
mod selectors;
mod utils;

use self::utils::{odds_are, random_individual, inspect, sort_population};

/// Simulates an evolution with genetic algorithm.
pub fn simulate<F, C>(pop_size: usize, chrom_size: usize, fitness_fn: &F, constr_fn: &C) -> Vec<bool>
where F: Fn(&Vec<bool>) -> usize, C: Fn(&Vec<bool>) -> bool {
    let mut population = utils::create_population(pop_size, chrom_size);
    let xover_probability = 0.8;
    let mutation_probability = 0.3;
    let elitism = 0.1;
    
    // Run for a number of generations
    for i in 0..100 {
        // Selection
        let mut sorted_population = sort_population(&population, fitness_fn);
        let elitism_mark: usize = (elitism * pop_size as f32) as usize;
        let mut new_population = selectors::best(sorted_population, elitism_mark);
        
        // Fill the rest of population with offsprings
        while new_population.len() != pop_size {
            let mut child: Vec<bool>;
            
            // Mate or select a random individual
            if odds_are(xover_probability) {
                // Select two random individuals
                let in1 = random_individual(&population).clone();
                let in2 = random_individual(&population).clone();
                
                // Crossover
                child = crossover::simple_half(in1, in2);
            } else {
                child = random_individual(&population);
            }
            
            // Mutation
            if odds_are(mutation_probability) {
                mutators::random_single(&mut child);
            }
            
            // Append to the rest of the population, if valid
            if constr_fn(&child) {
                new_population.push(child);
            }
        }
        
        // Replace the current population with new population
        population = new_population;
    }
    
    // Return the best result
    let sorted_population = sort_population(&population, fitness_fn);
    sorted_population.first().unwrap().clone()
}