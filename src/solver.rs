use parser::Knapsack;
use parser::KnapItem;
use std::u16;

#[derive(Debug, Clone, Copy)]
pub struct KnapSolution {
    knap_id: u16,
    pub bitmask: u16,
    pub price: u16,
    pub weight: u16,
}

impl KnapSolution {
    
    
    
}

fn items_from_bmask(knap: &Knapsack, bitmask: u16) -> Vec<&KnapItem> {
    knap.items.iter().enumerate().filter(|item| {
        bitmask & (1 << item.0) != 0
    }).map(|item| item.1).collect()
}

fn calc_fitness(items: Vec<&KnapItem>) -> (u16, u16) {
    items.iter().fold((0, 0), |acc, ref item| (acc.0 + item.weight, acc.1 + item.price))
}

pub fn validate(solution: &KnapSolution, knap: &Knapsack) -> bool {
    solution.weight < knap.capacity
}

pub fn solve_first(knap: &Knapsack) -> KnapSolution {
    let mut bitmask = u16::pow(2, knap.items.len() as u32) - 1;
    let mut fitness = calc_fitness(items_from_bmask(knap, bitmask));
        
    while fitness.0 > knap.capacity {
        bitmask = bitmask >> 1;
        fitness = calc_fitness(items_from_bmask(knap, bitmask));
    }
    
    KnapSolution {
        knap_id: knap.id,
        bitmask: bitmask,
        weight: fitness.0,
        price: fitness.1,
    }
}

pub fn solve_bruteforce(knap: &Knapsack) -> KnapSolution {
    let mut bitmask = 0;
    let mut fitness = calc_fitness(items_from_bmask(knap, bitmask));
    let mut best = (0, 0);
    
    let max_bitmask = u16::pow(2, knap.items.len() as u32);
        
    while bitmask < max_bitmask {
        bitmask += 1;
        fitness = calc_fitness(items_from_bmask(knap, bitmask));
        
        if (fitness.0 <= knap.capacity && fitness.1 > best.0) {
            best = (fitness.1, bitmask);
        }
    }
    
    fitness = calc_fitness(items_from_bmask(knap, best.1));
    
    KnapSolution {
        knap_id: knap.id,
        bitmask: bitmask,
        weight: fitness.0,
        price: fitness.1,
    }
}

// pub fn solve_random(knap: Knapsack) -> KnapSolution {
//     
// }
// 

// 
// pub fn solve_greedy(knap: Knapsack) -> KnapSolution {
//     
// }
// 
// pub fn solve_heuristic(knap: Knapsack) -> KnapSolution {
//     
// }