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
pub fn header_csv(delimiter: &str) {
    println!("knap_id{}method{}item_count{}price{}elapsed_ms",
        delimiter, delimiter, delimiter, delimiter);
}

#[allow(dead_code)]
pub fn report_csv(knap: &Knapsack, solution: SolutionType, delimiter: &str) {
    let mut stype = String::from(format!("{:?}", solution));
    //stype.truncate(7);
    
    println!("{}{}{}{}{}{}{}{}{}",
        knap.id, delimiter,
        stype, delimiter,
        knap.config.1, delimiter,
        knap.price, delimiter,
        knap.elapsed,
    );
}