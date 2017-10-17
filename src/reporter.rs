use parser::{Knapsack, KnapItem};
use solver::KnapSolution;

pub fn report_display(knap: &Knapsack, solution: &KnapSolution) {
    println!("KNAP_ID: {}\tSIZE: {}\t{:?}\tPRICE: {}\tWEIGHT: {}/{}\tELAPSED: {}ms", 
        knap.id,
        knap.items.len(),
        solution.soltype,
        solution.price,
        solution.weight,
        knap.capacity,
        solution.elapsed,
    );
}

pub fn report_csv_head() {
    println!("knap_id,item_count,capacity,sol_type,price,weight,bitmask,elapsed_ms");
}

pub fn report_csv(knap: &Knapsack, solution: &KnapSolution) {
    println!("{},{},{},{:?},{},{},{},{}", 
        knap.id,
        knap.items.len(),
        knap.capacity,
        solution.soltype,
        solution.price,
        solution.weight,
        solution.bitmask,
        solution.elapsed,
    );
}
