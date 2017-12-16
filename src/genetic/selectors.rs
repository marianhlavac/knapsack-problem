use genetic::utils;

pub fn tournament<F>(population: &Vec<Vec<bool>>, tournaments: usize, 
    pool_size: usize, fitness_fn: &F) -> Vec<Vec<bool>> 
where F: Fn(&Vec<bool>) -> usize {
    let mut new_pop = Vec::new();
    let pop_size = population.len();
    let mut pool = Vec::new();
    
    // Run tournaments
    for _ in 0..tournaments {
        // Select random individuals
        for _ in 0..pool_size {
            pool.push(utils::random_individual(population));
        }
        
        // Let them fight! And select the best.
        new_pop.push(utils::sort_population(&pool, fitness_fn).first().unwrap().clone());
    }
    
    new_pop
}