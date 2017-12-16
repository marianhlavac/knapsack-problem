mod crossover;
mod mutators;
mod selectors;
mod utils;

use self::utils::{odds_are, random_individual, inspect, sort_population};

/// Simulates an evolution with genetic algorithm.
pub fn simulate<F, C>(pop_size: usize, chrom_size: usize, fitness_fn: &F, constr_fn: &C) -> Vec<bool>
where F: Fn(&Vec<bool>) -> usize, C: Fn(&Vec<bool>) -> bool {
    let mut population = utils::create_population(pop_size, chrom_size);
    let xover_probability = 0.9;
    let mutation_probability = 0.2;
    let elitism = 0.05;
    
    // Run for a number of generations
    for i in 0..250 {
        // Selection
        let mut sorted_population = sort_population(&population, fitness_fn);
        //inspect(i, &sorted_population, &fitness_fn);
        let elitism_mark: usize = (elitism * pop_size as f32) as usize;
        
        // Selection
        let mut new_population = selectors::tournament(&population, 2, 12, fitness_fn);
        
        // Elitism
        sorted_population.truncate(2);
        new_population.append(&mut sorted_population);
        
        // Fill the rest of population with offsprings
        while new_population.len() != pop_size {
            let mut child: Vec<bool>;
            
            // Mate or select a random individual
            if odds_are(xover_probability) {
                // Select two random individuals
                let in1 = random_individual(&new_population).clone();
                let in2 = random_individual(&new_population).clone();
                
                // Crossover
                child = crossover::single_point(in1, in2);
            } else {
                child = random_individual(&population);
            }
            
            // Mutation
            mutators::random_inverse(&mut child, mutation_probability);
            
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