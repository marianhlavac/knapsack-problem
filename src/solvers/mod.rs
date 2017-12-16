mod branchandbound;
mod dynamic;
mod fptas;
mod heuristic;
mod recursive;

use parser::{sum_of_prices, bit_substructures, Knapsack, SolutionType};
use genetic;
use time::PreciseTime as ptime;

/// Solves an instance of knapsack problem, returning solved knapsack.
pub fn solve(knap: &Knapsack, soltype: SolutionType) -> Knapsack {
    let start = ptime::now();
    
    // Start solving
    let mut solved = match soltype {
        SolutionType::Recursive => recursive::solve(knap.clone()),
        SolutionType::BranchAndBound => branchandbound::solve(knap.clone()),
        SolutionType::Heuristic => heuristic::solve(knap.clone()),
        SolutionType::Dynamic => dynamic::solve(knap.clone()),
        SolutionType::FPTAS25 => fptas::solve(knap.clone(), 0.25),
        SolutionType::FPTAS50 => fptas::solve(knap.clone(), 0.5),
        SolutionType::FPTAS75 => fptas::solve(knap.clone(), 0.75),
        SolutionType::Evolution => solve_by_evolution(knap.clone(), 150),
        SolutionType::None => knap.clone(),
    };
    
    let elapsed_t = start.to(ptime::now());
    solved.elapsed = if elapsed_t.num_milliseconds() <= 1 { elapsed_t.num_nanoseconds().unwrap() as f32 / 1000000.0 } else { elapsed_t.num_milliseconds() as f32 };
    
    solved
}

/// Starts an evolution algorithm for solving the problem.
fn solve_by_evolution(knap: Knapsack, pop_size: usize) -> Knapsack {
    // Get values substructures
    let (prices, weights) = bit_substructures(&knap);
    
    // Define a fitness function
    let fitness_fn = |x: &Vec<bool>| {
        prices.iter().enumerate().fold(0, |sum, (i, price)| {
            if x[i] { sum + price } else { sum }
        }) as usize
    };
    
    // Define a constraint function
    let constraint_fn = |x: &Vec<bool>| {
        let weight = weights.iter().enumerate().fold(0, |sum, (i, weight)| {
            if x[i] { sum + weight } else { sum }
        });
        weight <= knap.capacity
    };
    
    let best = genetic::simulate(pop_size, prices.len(), &fitness_fn, &constraint_fn);
    bits_to_knap(best, knap.clone())
}

/// Converts bit string to actual knap items
fn bits_to_knap(bits: Vec<bool>, knap: Knapsack) -> Knapsack {
    let new_items = knap.items.clone().iter().enumerate().filter(|&(i, x)| bits[i])
                    .map(|(i, x)| *x).collect();
    let mut new_knap = knap.clone();
    new_knap.items = new_items;
    new_knap.price = sum_of_prices(&new_knap.items);
    new_knap
}
