use genetic::utils;

pub fn tournament(population: Vec<Vec<bool>>, pool_size: usize) {
    let pop_size = population.len();
    let mut pool: Vec<Vec<bool>> = Vec::new();
    
    // Select random individuals
    for _ in 0..pool_size {
        pool.push(utils::random_individual(&population));
    }
    
    // Let them fight! (sort individuals by fitness rank)
    // ...
}

pub fn best(sorted_population: Vec<Vec<bool>>, count: usize) -> Vec<Vec<bool>> {
    // TODO: How to do this with _split ?
    let mut r: Vec<Vec<bool>> = Vec::new();
    for i in 0..count {
        r.push(sorted_population[i].clone());
    }
    
    r
}