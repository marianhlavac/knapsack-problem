use parser::{sum_of_prices, Knapsack, KnapItem};

pub fn split_knapsacks(knap: &Knapsack) -> (Knapsack, Knapsack) {
    let mut new_items = knap.items.clone();
    let item = new_items.pop().unwrap();
    
    let mut knapsack_with = knap.clone();
    knapsack_with.items = new_items.clone();
    let mut knapsack_wout = knapsack_with.clone();
    
    if item.weight <= knapsack_with.capacity {
        knapsack_wout.price += item.price;
        knapsack_wout.capacity -= item.weight;
    }
    
    (knapsack_with, knapsack_wout)
}

pub fn solve(knap: Knapsack) -> Knapsack {
    // Finish when depth is maximum
    if knap.items.len() <= 0 { return knap; }
    
    // Split knapsacks
    let (knapsack_with, knapsack_wout) = split_knapsacks(&knap);
    
    // Solve recursively
    let knapsack_with_sol = solve(knapsack_with);
    let knapsack_wout_sol = solve(knapsack_wout);
    
    // Return the better solution
    if knapsack_with_sol.price > knapsack_wout_sol.price {
        knapsack_with_sol
    } else {
        knapsack_wout_sol
    }
}