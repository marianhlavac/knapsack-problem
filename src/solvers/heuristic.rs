use parser::{sum_of_prices, Knapsack, KnapItem};

pub fn solve(knap: Knapsack) -> Knapsack {
    let mut items: Vec<KnapItem> = knap.items.clone();
    items.sort_by(|a, b| (a.price / a.weight).cmp(&(b.price / b.weight)));
    
    let mut result_items: Vec<KnapItem> = vec![];
    let mut total_weight = 0;
    for item in items {
        if item.weight + total_weight <= knap.capacity {
            result_items.push(item);
            total_weight += item.weight;
        } else {
            break;
        }
    }
    
    let mut result_knap = knap.clone();
    result_knap.price = sum_of_prices(&result_items);
    result_knap.items = result_items;
    
    result_knap
}