pub mod parser;
use parser::Knapsack;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn read_file(file_path: &str) -> BufReader<File> {
    return BufReader::new(match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("File {} can't be opened: {}", file_path, err),
    });
}

fn main() {
    let file = read_file("data/knap_4.inst.dat");
    
    let knapsacks: Vec<Knapsack> = file.lines().map(|line| {
        parser::parse_knapsack(&line.unwrap())
    }).collect();
    
    println!("{:?}", knapsacks)  
}
