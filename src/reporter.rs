use solver::KnapSolution;

pub fn report_display(solution: &KnapSolution) {
    println!("TYPE: {:?}\tKNAP_ID: {}\tPRICE: {}\tWEIGHT: {}\tELAPSED: {}ms", 
        solution.soltype,
        solution.knap_id,
        solution.price,
        solution.weight,
        solution.elapsed,
    );
}

pub fn report_csv(solution: &KnapSolution) {
    println!("not implemented");
}
