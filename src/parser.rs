#[derive(Debug)]
pub struct Knapsack {
    id: u16,
    capacity: u16,
    items: Vec<(u16, u16)>,
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
    
    let items: Vec<(u16, u16)> = values.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    
    return Knapsack { 
        id: props[0], 
        capacity: props[2], 
        items: items,
    };
}