mod cards;

use std::{thread, time};
use std::env;
use std::process::Command;


fn main() {

    let args: Vec<_> = env::args().collect();

    if args[1] == "cards" {
        let  num_cards : usize = args[2].parse().unwrap();
        let  sec_per_card : usize = args[3].parse().unwrap();
        cards::cards_memory(num_cards, sec_per_card);
        //println!("{}", num_cards);
    } else {
        println!("a different game");
    }; 
    
    
    
    
}
