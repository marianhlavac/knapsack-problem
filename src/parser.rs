#[derive(Debug, Clone, Copy)]
pub struct KnapItem {
    pub id: u16,
    pub weight: u16,
    pub price: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum SolutionType {
    Recursive,
    BranchAndBound,
}

#[derive(Debug, Clone)]
pub struct Knapsack {
    pub id: u16,
    pub capacity: u16,
    pub items: Vec<KnapItem>,
    pub config: (u16, u16),
    pub price: u16,
    pub elapsed: f32,
}

fn parse_num(val: &str) -> u16 {
    return val.parse::<u16>().unwrap();
}

pub fn parse_knapsack(string: &str) -> Knapsack {
    let mut values: Vec<u16> = string.split(char::is_whitespace).map(parse_num).collect();
    let props: Vec<u16> = values.drain(0..3).collect();
    
    if values.len() != 2 * props[1] as usize {
        panic!("There should be {} items in the string.", props[1]);
    }
    
    let items: Vec<KnapItem> = values.chunks(2).enumerate().map(|chunk| KnapItem { 
        id: chunk.0 as u16, weight: chunk.1[0], price: chunk.1[1] 
    }).collect();
    let items_count = items.len() as u16;
    
    return Knapsack { 
        id: props[0], 
        capacity: props[2], 
        items: items,
        config: (props[2], items_count),
        price: 0,
        elapsed: 0.0,
    };
}