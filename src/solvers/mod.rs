mod branchandbound;
mod dynamic;
mod fptas;
mod heuristic;
mod recursive;

use parser::{bit_substructures, Knapsack, SolutionType};
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
        SolutionType::Evolution => solve_by_evolution(knap.clone(), 8),
        SolutionType::None => knap.clone(),
    };
    
    let elapsed_t = start.to(ptime::now());
    solved.elapsed = if elapsed_t.num_milliseconds() <= 1 { elapsed_t.num_nanoseconds().unwrap() as f32 / 1000000.0 } else { elapsed_t.num_milliseconds() as f32 };
    
    solved
}

/// Starts an evolution algorithm for solving the problem.
fn solve_by_evolution(knap: Knapsack, pop_size: usize) -> Knapsack {
    // Get values substructures
    let (prices, weights) = parser::bit_substructures(&knap);
    
    // Define a fitness function
    let fitness_fn = |x: BitVec| {
        prices.iter().enumerate().fold(0, |sum, (i, price)| if (x.get(i)) { sum += price });
    }
    
    // Define a constraint function
    let constraint_fn = |x: BitVec| {
        let weight = weights.iter().enumerate().fold(0, |sum, (i, w)| if (x.get(i)) { sum += w });
        weight <= knap.capacity;
    }
    
    genetic::simulate(pop_size, fitness_fn, constraint_fn);
}