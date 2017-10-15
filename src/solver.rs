use parser::Knapsack;
use parser::KnapItem;
use time::{Duration, PreciseTime};
use std::u32;

#[derive(Debug, Clone, Copy)]
pub struct KnapSolution {
    pub knap_id: u16,
    pub bitmask: u32,
    pub price: u16,
    pub weight: u16,
    pub elapsed: u16,
}

enum SolutionType {
    Bruteforce,
    Heuristic,
}

/// Converts bitmask to vector of items. (bitmask is a presence mask)
fn items_from_bmask(knap: &Knapsack, bitmask: u32) -> Vec<&KnapItem> {
    knap.items.iter().enumerate().filter(|item| {
        bitmask & (1 << item.0) != 0
    }).map(|item| item.1).collect()
}

/// Converts a vector of items to a presence bitmask.
fn bitmask_from_items(items: Vec<&KnapItem>) -> u16 {
    return 0
}

/// Calculate a "fitness" of specified vector of items, returning a tuple
/// consisting of total items weight and total items price.
fn calc_fitness(items: Vec<&KnapItem>) -> (u16, u16) {
    items.iter().fold((0, 0), |acc, ref item| (acc.0 + item.weight, acc.1 + item.price))
}

/// Validates a solution. (items weight can't exceed knapsack capacity)
pub fn validate(solution: &KnapSolution, knap: &Knapsack) -> bool {
    solution.weight <= knap.capacity
}

/// Solves an instance of knapsack problem, returning a solution structure.
/// Types of solutions are Bruteforce and heuristic (sorted by price/weight ratio).
pub fn solve(knap: &Knapsack, type: SolutionType) -> KnapSolution {
    let start = PreciseTime::now();
    let mut bitmask = 0;
    let mut fitness = calc_fitness(items_from_bmask(knap, bitmask));
    let mut best = (fitness.1, 0);
    
    let max_bitmask = u32::pow(2, knap.items.len() as u32);
        
    while bitmask < max_bitmask {
        bitmask += 1;
        fitness = calc_fitness(items_from_bmask(knap, bitmask));
        
        if fitness.0 <= knap.capacity && fitness.1 > best.0 {
            best = (fitness.1, bitmask);
        }
    }
    
    fitness = calc_fitness(items_from_bmask(knap, best.1));
    
    let elapsed = start.to(PreciseTime::now()).num_milliseconds() as u16;
    
    KnapSolution {
        knap_id: knap.id,
        bitmask: bitmask,
        weight: fitness.0,
        price: fitness.1,
        elapsed: elapsed,
    }
}