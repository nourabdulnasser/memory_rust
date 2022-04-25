use std::env;
use rand::Rng;
use std::time::Duration;
use std::thread::sleep;

pub fn cards_memory(length: usize, time: usize){

   // random suits and ranks

    let ranks = vec!["A","K","Q","J","2","3","4","5","6","7","8","9","10"]; 
    let suits = vec!["♣","♦","♥","♠"];

    let random_card = || {

        let mut rng = rand::thread_rng();

        let rand_index_rank = rng.gen_range(0..ranks.len()-1);
        let rand_index_suit = rng.gen_range(0..suits.len()-1);
    
        let rank = ranks[rand_index_rank];
        let suit = suits[rand_index_suit];

        //println!("{}", rank.to_owned()+suit);

        return rank.to_owned()+suit;

        // looks
        // like
        // this

    };

    // generating cards
    
    let mut cards_line: String = "".to_string();

    for  card in 0..length{
        //random_card();
        cards_line = cards_line + &random_card();
        if card < length-1 {
            cards_line = cards_line + ",  ";
        }

        // looks,  like,  this 

    }

    println!("{}", cards_line);

    // wait for {time} seconds

    
   

   


    // user prompt




   
                          
    // compare and print results
                          
                          
                          


    
    //let suit = "CDHS";
    //println!("{}", generate(length, suit));

}

