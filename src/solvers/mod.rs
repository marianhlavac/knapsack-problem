use parser::{bit_substructures, Knapsack, SolutionType};
use solver_recursive;
use solver_branchandbound;
use solver_heuristic;
use solver_dynamic;
use solver_fptas;
use genetic;
use time::PreciseTime;

/// Solves an instance of knapsack problem, returning solved knapsack.
pub fn solve(knap: &Knapsack, soltype: SolutionType) -> Knapsack {
    let start = PreciseTime::now();
    
    // Start solving
    let mut solved = match soltype {
        SolutionType::Recursive => solver_recursive::solve(knap.clone()),
        SolutionType::BranchAndBound => solver_branchandbound::solve(knap.clone()),
        SolutionType::Heuristic => solver_heuristic::solve(knap.clone()),
        SolutionType::Dynamic => solver_dynamic::solve(knap.clone()),
        SolutionType::FPTAS25 => solver_fptas::solve(knap.clone(), 0.25),
        SolutionType::FPTAS50 => solver_fptas::solve(knap.clone(), 0.5),
        SolutionType::FPTAS75 => solver_fptas::solve(knap.clone(), 0.75),
        SolutionType::Evolution => solve_by_evolution(knap.clone(), 8),
        SolutionType::None => knap.clone(),
    };
    
    let elapsed_t = start.to(PreciseTime::now());
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