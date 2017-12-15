use parser::{Knapsack, KnapItem};
use solvers::dynamic;
use std::f32;

pub fn solve(knap: Knapsack, accuracy: f32) -> Knapsack {
    // Find the largest price in knapsack
    let max_price = knap.items.iter().fold(0, |acc, &x| if x.price > acc { x.price } else { acc });
    let ratio = (1.0 - accuracy) * max_price as f32 / knap.items.len() as f32;
    
    // Modify prices in knapsack
    let mut mod_knap = knap.clone();
    mod_knap.items = knap.items.iter().map(|item| { KnapItem { 
        id: item.id, weight: item.weight, price: f32::floor(item.price as f32 / ratio) as u16
    } }).collect();
    
    // Solve with dynamic solver
    let mut solution = dynamic::solve(mod_knap);
    
    // Fix the result price
    solution.price = f32::floor(solution.price as f32 * ratio) as u16;
    
    solution
}