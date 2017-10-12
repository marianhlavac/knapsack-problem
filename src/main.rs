mod parser;
mod solver; 
use parser::Knapsack;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_file(file_path: &str) -> BufReader<File> {
    return BufReader::new(match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", file_path, err),
    });
}

fn main() {
    let file = read_file("data/knap_15.inst.dat");
    
    let knapsacks: Vec<Knapsack> = file.lines().map(|line| {
        parser::parse_knapsack(&line.unwrap())
    }).collect();
    
    for knapsack in knapsacks {
        println!("--- {:?}", knapsack);
        
        let solutionsf = solver::solve_first(&knapsack);
        println!("Solve First Method: {:?} {}", solutionsf, if solver::validate(&solutionsf, &knapsack) {"is valid"} else {"is not valid"});
        
        let solutionbf = solver::solve_bruteforce(&knapsack);
        println!("Bruteforce Method: {:?} {}", solutionbf, if solver::validate(&solutionbf, &knapsack) {"is valid"} else {"is not valid"});
    }
    
    
}
