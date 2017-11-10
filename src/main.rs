extern crate time;

mod parser;
mod solver;
mod reporter;
mod solver_recursive;
mod solver_branchandbound;
mod solver_dynamic;

use parser::{Knapsack, SolutionType};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

/// Reads a file, returning BufReader in order to parse it after.
fn read_file(file_path: &str) -> BufReader<File> {
    return BufReader::new(match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", file_path, err),
    });
}

/// Solves knapsack problem instances and measures elapsed time on solving.
fn main() {
    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();
    let mut input_files = args.clone();
    input_files.remove(0);
    
    // Load each instance set file
    for filename in input_files {
        let file = read_file(&filename);
        let knapsacks: Vec<Knapsack> = file.lines()
            .map(|line| parser::parse_knapsack(&line.unwrap()))
            .collect();
            
        reporter::header_csv();
            
        // And solve whole set with results reports
        for knapsack in knapsacks {
            let solved = solver::solve(&knapsack, SolutionType::Recursive);
            reporter::report_csv(&solved, SolutionType::Recursive);
            let solved2 = solver::solve(&knapsack, SolutionType::BranchAndBound);
            reporter::report_csv(&solved2, SolutionType::BranchAndBound);
            let solved3 = solver::solve(&knapsack, SolutionType::Dynamic);
            reporter::report_csv(&solved3, SolutionType::Dynamic);
        }
    }
}
