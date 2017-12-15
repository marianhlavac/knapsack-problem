use parser::{sum_of_prices, Knapsack};
use solvers::recursive;

fn solve_recurr(knap: Knapsack, best: &mut u16, reward_left: u16) -> Knapsack {
    // Finish when depth is maximum
    if knap.items.len() <= 0 { return knap; }
    
    // Split knapsacks
    let (knapsack_with, knapsack_wout) = recursive::split_knapsacks(&knap);
    
    // Solve recursively, conditioned by b&b
    let mut top = knap.price + reward_left;
    let last_price = knap.items.last().unwrap().price;
    if best < &mut top { 
        let knapsack_with_sol = solve_recurr(knapsack_with, best, reward_left - last_price);
        let knapsack_wout_sol = solve_recurr(knapsack_wout, best, reward_left - last_price);

        // Return the better solution
        if knapsack_with_sol.price > knapsack_wout_sol.price {
            *best = knapsack_with_sol.price.clone();
            knapsack_with_sol
        } else {
            *best = knapsack_wout_sol.price.clone();
            knapsack_wout_sol
        }
    } else {
        knap
    }
}

pub fn solve(knap: Knapsack) -> Knapsack {    
    let mut best = 0;
    let total_reward = sum_of_prices(&knap.items);
    solve_recurr(knap, &mut best, total_reward)
}