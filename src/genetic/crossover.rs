use rand::{thread_rng, Rng};

pub fn single_point(p1: Vec<bool>, p2: Vec<bool>) -> Vec<bool> {
    let size: usize = p1.len();
    let mut child: Vec<bool> = vec![false; size];
    let midpoint: usize = thread_rng().gen_range(0, size);
    
    for i in 0..size {
        child[i] = if i <= midpoint { p1[i] } else { p2[i] }; 
    }
    
    child
}