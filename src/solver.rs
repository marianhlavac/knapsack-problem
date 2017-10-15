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

fn items_from_bmask(knap: &Knapsack, bitmask: u32) -> Vec<&KnapItem> {
    knap.items.iter().enumerate().filter(|item| {
        bitmask & (1 << item.0) != 0
    }).map(|item| item.1).collect()
}

fn bitmask_from_items(items: Vec<&KnapItem>) -> u16 {
    return 0
}

fn calc_fitness(items: Vec<&KnapItem>) -> (u16, u16) {
    items.iter().fold((0, 0), |acc, ref item| (acc.0 + item.weight, acc.1 + item.price))
    // let mut fit : (u16, u16) = (0, 0);
    // for item in items {
    //     fit.0 += item.weight;
    //     fit.1 += item.price;
    // }
    // 
    // fit
}

pub fn validate(solution: &KnapSolution, knap: &Knapsack) -> bool {
    solution.weight <= knap.capacity
}

pub fn solve_bruteforce(knap: &Knapsack) -> KnapSolution {
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

pub fn solve_heuristic(knap: Knapsack) -> KnapSolution {
    KnapSolution {
        knap_id: 0,
        bitmask: 0,
        weight: 0,
        price: 0,
        elapsed: 0,
    }
}