use parser::{Knapsack, SolutionType};
use solver_recursive;
use solver_branchandbound;
use solver_dynamic;
use solver_fptas;
use time::PreciseTime;

/// Solves an instance of knapsack problem, returning solved knapsack.
pub fn solve(knap: &Knapsack, soltype: SolutionType) -> Knapsack {
    let start = PreciseTime::now();
    
    // Start solving
    let mut solved = match soltype {
        SolutionType::Recursive => solver_recursive::solve(knap.clone()),
        SolutionType::BranchAndBound => solver_branchandbound::solve(knap.clone()),
        SolutionType::Dynamic => solver_dynamic::solve(knap.clone()),
        SolutionType::FPTAS25 => solver_fptas::solve(knap.clone(), 0.25),
        SolutionType::FPTAS50 => solver_fptas::solve(knap.clone(), 0.5),
        SolutionType::FPTAS75 => solver_fptas::solve(knap.clone(), 0.75),
        SolutionType::None => knap.clone(),
    };
    
    let elapsed_t = start.to(PreciseTime::now());
    solved.elapsed = if elapsed_t.num_milliseconds() <= 1 { elapsed_t.num_nanoseconds().unwrap() as f32 / 1000000.0 } else { elapsed_t.num_milliseconds() as f32 };
    
    solved
}