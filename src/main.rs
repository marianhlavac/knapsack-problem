extern crate time;

mod parser;
mod solver;
mod reporter;
use parser::Knapsack;
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
    
    reporter::report_csv_head();
    
    // Load each instance set file
    for filename in input_files {
        let file = read_file(&filename);
        let knapsacks: Vec<Knapsack> = file.lines()
            .map(|line| parser::parse_knapsack(&line.unwrap()))
            .collect();
            
        // And solve whole set with results reports
        for knapsack in knapsacks {
            let solution_bf = solver::solve(&knapsack, solver::SolutionType::Bruteforce);
            reporter::report_csv(&knapsack, &solution_bf);
            let solution_heu = solver::solve(&knapsack, solver::SolutionType::Heuristic);
            reporter::report_csv(&knapsack, &solution_heu);
        }
    }
    

    
}
