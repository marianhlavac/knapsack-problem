use genetic::utils::{odds_are};

pub fn random_inverse(individual: &mut Vec<bool>, probability: f32) {
    for i in 0..individual.len() {
        if odds_are(probability) {
            individual[i] = !individual[i];
        }
    }
}