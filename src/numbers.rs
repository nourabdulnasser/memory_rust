use random_string::generate;
use std::{thread, time,io};
use terminal::{Clear, Action};


pub fn memory_numbers(length: usize,sleep_per_digit: usize) {


    let terminal = terminal::stdout();



    let charset = "1234567890";
    let numbers_to_mem = generate(length, charset);
    let sleep_time = sleep_per_digit*length;
    let sleep_time_u64: u64 = sleep_time as u64;


    println!("{}", numbers_to_mem);
    println!("you have {} amount of second(s) to memorise these digits",sleep_time);
    thread::sleep(time::Duration::from_secs(sleep_time_u64));


     terminal.act(Action::ClearTerminal(Clear::All));
     println!("okay print what you memorised");


    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read from stdin");

    let guess_trim = guess.trim();



     if guess_trim == numbers_to_mem {

         println!("well done you got it correct, the number was {}",numbers_to_mem);

     }
     else {
         println!("wow you're trash, the number was {}",numbers_to_mem);


     }



}
