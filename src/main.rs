extern crate time;

mod parser;
mod solver;
mod reporter;
mod solver_recursive;
mod solver_branchandbound;
mod solver_dynamic;
mod solver_fptas;
mod solver_heuristic;

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
    let mut methods = Vec::new();
    reporter::header_csv(delimiter);
    
    // Check CLI arguments for settings string
    if input_files[0].starts_with("opt:") {
        let options = input_files[0].split_off(4);
        
        options.chars().for_each(|option| match option {
            'r' => methods.push(SolutionType::Recursive),
            'b' => methods.push(SolutionType::BranchAndBound),
            'd' => methods.push(SolutionType::Dynamic),
            'f' => methods.push(SolutionType::FPTAS50),
            'h' => methods.push(SolutionType::Heuristic),
            _ => (),
        });
        
        input_files.remove(0);
    } else {
        methods = vec!(
            SolutionType::Recursive, 
            SolutionType::BranchAndBound, 
            SolutionType::Dynamic,
        );
    }
    
    // Load each instance set file
    for filename in input_files {
        let file = read_file(&filename);
        let knapsacks: Vec<Knapsack> = file.lines()
            .map(|line| parser::parse_knapsack(&line.unwrap()))
            .collect();
            
        // And solve whole set with results reports
        for knapsack in knapsacks {
            for method in &methods {
                let results = solver::solve(&knapsack, *method);
                reporter::report_csv(&results, *method, delimiter);
            }
        }
    }
}
