use parser::{sum_of_prices, Knapsack};
use solvers::recursive;

fn solve_recurr(knap: &Knapsack, mut state: &mut Vec<Vec<u16>>, total_price: u16, total_weight: u16, item_id: usize) {
    // End the recursion
    if item_id >= knap.items.len() { return; }
    
    // Alter the state table
    state[item_id][total_price as usize] = total_weight;
    
    // If there is another item available to insertion into knapsack...
    if item_id + 1 < knap.items.len() {
        let next_item = knap.items[item_id+1];
        let next_price = total_price + next_item.price;
        let next_weight = total_weight + next_item.weight;

        // ... and it doesn't exceed the knapsack capacity...
        if next_weight <= knap.capacity {
            // Recursively solve with item inserted
            solve_recurr(&knap, &mut state, next_price, next_weight, item_id + 1);
        }
        // Also recursively solve with item not inserted
        solve_recurr(&knap, &mut state, total_price, total_weight, item_id + 1);
    }
}

pub fn solve(knap: Knapsack) -> Knapsack {
    // Compute state table dimensions
    let reward_columns = sum_of_prices(&knap.items) as usize + 1;
    let item_rows = knap.items.len();
    
    // Create state memory table
    let mut state = vec![vec![0u16; reward_columns]; item_rows];
    
    // Recursion bounce-off
    let item = knap.items[0];
    solve_recurr(&knap, &mut state, item.price, item.weight, 0);
    solve_recurr(&knap, &mut state, 0, 0, 0);
    
    // Pick the last row from state table
    let mut last_row = state.last().unwrap().clone();
    while *last_row.last().unwrap() == 0 { last_row.pop(); }
    
    // Most right non-null field is the result
    let result = last_row.len() as u16 - 1;
    
    let mut solution = knap.clone();
    solution.price = result;
    
    solution
}