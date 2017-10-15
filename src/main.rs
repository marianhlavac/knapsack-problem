extern crate time;

mod parser;
mod solver; 
use parser::Knapsack;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::prelude::*;                                                           
use std::io; 

fn read_file(file_path: &str) -> BufReader<File> {
    return BufReader::new(match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", file_path, err),
    });
}

fn main() {
    let file = read_file("data/knap_20.inst.dat");
    
    let knapsacks: Vec<Knapsack> = file.lines().map(|line| {
        parser::parse_knapsack(&line.unwrap())
    }).collect();
    
    for knapsack in knapsacks { 
        print!("METHOD_BRUTEFORCE\t\tKSID: {}\t\t", knapsack.id);   
        io::stdout().flush().ok(); 
        let sol = solver::solve_bruteforce(&knapsack);
        println!("P: {}\tW: {}\t\t{}VALID\tEL: {}ms", 
            sol.price,
            sol.weight, 
            if solver::validate(&sol, &knapsack) {""} else {"IN"},
            sol.elapsed,
        );
    }
    
    
}
