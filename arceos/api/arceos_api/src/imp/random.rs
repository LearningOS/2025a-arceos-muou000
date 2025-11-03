pub use axhal::misc::*;

pub fn get_random_number() -> u128 {
    axhal::misc::random()
}