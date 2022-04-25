mod numbers;
mod cards;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];


    if mode == "-numbers" {
    let amount_of_digits = args[2].parse::<usize>().unwrap();
    let amount_of_seconds_per_digits = args[3].parse::<usize>().unwrap();
    numbers::memory_numbers(amount_of_digits,amount_of_seconds_per_digits);
    }
    
    /* let args: Vec<_> = env::args().collect();

    if args[1] == "cards" {
        let  num_cards : usize = args[2].parse().unwrap();
        let  sec_per_card : usize = args[3].parse().unwrap();
        cards::cards_memory(num_cards, sec_per_card);
        //println!("{}", num_cards);
    } else {
        println!("a different game");
    }; */ 

}
