use parser::{Knapsack, SolutionType};

#[allow(dead_code)]
pub fn report_display(knap: &Knapsack, solution: SolutionType) {
    println!("#{}\tcfg {:?}\t${}\t{} ms\t{:?}", 
        knap.id,
        knap.config,
        knap.price,
        knap.elapsed,
        solution,
    );
}

#[allow(dead_code)]
pub fn header_csv() {
    println!("id,solution_type,item_count,price,elapsed_ms");
}

#[allow(dead_code)]
pub fn report_csv(knap: &Knapsack, solution: SolutionType) {
    println!("{},{:?},{},{},{}",
        knap.id,
        solution,
        knap.config.1,
        knap.price,
        knap.elapsed,
    );
}