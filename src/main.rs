extern crate time;

mod parser;
mod solver; 
mod reporter;
use parser::Knapsack;
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Reads a file, returning BufReader in order to parse it after.
fn read_file(file_path: &str) -> BufReader<File> {
    return BufReader::new(match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", file_path, err),
    });
}

/// Solves knapsack problem instances and measures elapsed time on solving.
fn main() {
    // Load a instance set file
    let file = read_file("data/knap_15.inst.dat");
    let knapsacks: Vec<Knapsack> = file.lines().map(|line| {
        parser::parse_knapsack(&line.unwrap())
    }).collect();
    
    // Solve whole set and report results
    for knapsack in knapsacks { 
        let solution = solver::solve(&knapsack, Bruteforce);
        reporter::report_display(&solution);
    }
}
