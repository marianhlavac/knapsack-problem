extern crate time;

mod parser;
mod solver;
mod reporter;
mod solver_recursive;
mod solver_branchandbound;
mod solver_dynamic;
mod solver_fptas;

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
    
    let delimiter = ",";
    reporter::header_csv(delimiter);
    
    // Load each instance set file
    for filename in input_files {
        let file = read_file(&filename);
        let knapsacks: Vec<Knapsack> = file.lines()
            .map(|line| parser::parse_knapsack(&line.unwrap()))
            .collect();
            
        // And solve whole set with results reports
        for knapsack in knapsacks {
            let recursive = solver::solve(&knapsack, SolutionType::Recursive);
            reporter::report_csv(&recursive, SolutionType::Recursive, delimiter);
            let brandbound = solver::solve(&knapsack, SolutionType::BranchAndBound);
            reporter::report_csv(&brandbound, SolutionType::BranchAndBound, delimiter);
            let dynamic = solver::solve(&knapsack, SolutionType::Dynamic);
            reporter::report_csv(&dynamic, SolutionType::Dynamic, delimiter);
            let fptas25 = solver::solve(&knapsack, SolutionType::FPTAS25);
            reporter::report_csv(&fptas25, SolutionType::FPTAS25, delimiter);
            let fptas50 = solver::solve(&knapsack, SolutionType::FPTAS50);
            reporter::report_csv(&fptas50, SolutionType::FPTAS50, delimiter);
            let fptas75 = solver::solve(&knapsack, SolutionType::FPTAS75);
            reporter::report_csv(&fptas75, SolutionType::FPTAS75, delimiter);
        }
    }
}
