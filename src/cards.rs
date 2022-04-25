use std::env;
use rand::Rng;
use std::{thread, time};
use std::io::{stdin,stdout,Write};


fn get_input() -> String {

    let mut input =String::new();


    let _=stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n')=input.chars().next_back() {
        input.pop();
    }
    if let Some('\r')=input.chars().next_back() {
        input.pop();
    }

    return input;

}


pub fn cards_memory(length: usize, time: usize){


    let ranks = vec!["A","K","Q","J","2","3","4","5","6","7","8","9","10"];
    let suits = vec!["c","d","h","s"]; 
    //let suits = vec!["♣","♦","♥","♠"];
    
    // random suits and ranks

    let random_card = || {

        let mut rng = rand::thread_rng();

        let rand_index_rank = rng.gen_range(0..ranks.len());
        let rand_index_suit = rng.gen_range(0..suits.len());
    
        let rank = ranks[rand_index_rank];
        let suit = suits[rand_index_suit];

        //println!("{}", rank.to_owned()+suit);

        return rank.to_owned()+suit;

    };

    // generating cards
    
    let mut cards_line: String = "".to_string();
    // let mut cards_vec = vec![];

    for  card in 0..length{
        // random_card();
        // let random_card_str = &random_card(); // to push random strings into vector
        cards_line = cards_line + &random_card();
        // cards_vec.push(random_card_str);
        if card < length-1 {
            cards_line = cards_line + ",  ";
        }

        // "looks,  like,  this" 

    }

    // println!("{:?}", cards_vec); // test
    


    // wait for {time} seconds

    let mem_time_u64 = (length*time).try_into().unwrap(); 
    let mem_dur = std::time::Duration::from_secs(mem_time_u64);

    println!("Memorise this:\n{{ {} }}\nYou have {} seconds.", cards_line, length*time);

    let now = time::Instant::now();

    thread::sleep(mem_dur);

    print!("\x1B[2J\x1B[1;1H");  // not as clean as I'd like it to be but will do for now.



    // user prompt

    print!("Type in what you memorised!\nTo exit, enter: exit\n");



    // comparing input and cards_line_vec

    let cards_line_split = cards_line.split(",  ");
    let cards_line_vec: Vec<&str> = cards_line_split.collect();

    // println!("{:?}", cards_line_vec); // test

    let mut counter = 0;

    for item in &cards_line_vec {
        let user_input = get_input();
        if user_input.to_uppercase() == cards_line_vec[counter].to_uppercase(){
            println!("Bravo! You got {} right!\n", cards_line_vec[counter]);
        } else if user_input == "exit" {
           break;
        } else {
            println!("Nay :( The right answer is {}.\n", cards_line_vec[counter]);
        };

        counter = counter +1;

    }

    println!("end");
    
}

