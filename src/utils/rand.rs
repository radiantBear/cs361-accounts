use rand::{distr::Alphanumeric, Rng};


pub fn generate_alphanumeric(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}