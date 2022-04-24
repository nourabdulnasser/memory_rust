mod numbers;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];


    if mode == "-numbers" {
    let amount_of_digits = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_digits = args[3].parse::<usize>().unwrap();
    numbers::memory_numbers(amount_of_digits,amount_of_seconds_per_digits);
    }
}
