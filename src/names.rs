use super::api_call;
use super::input_wrapper;
use std::{thread};
use terminal::{Clear, Action};
//use regex //1.4.5

// function for getting a random name
fn random_name() -> String{
    let mut api_name_vec: Vec<String> = vec![]; // vector of names extracted from api function
    let api_name: String = api_call::api_get_request("https://random-data-api.com/api/name/random_name");
    let api_delimit = regex::Regex::new(r",|:").unwrap(); // delimit characters for easier iteration
    let api_delimit_vec: Vec<&str> = api_delimit.split(&api_name).collect(); // vector of all elements in api
    let mut name = "";
    let mut counter = 0;

    // trying to convert elements of api_delimit_vec into String
    for element in &api_delimit_vec{
        if element == &"\"first_name\""
        {
            name = &api_delimit_vec[counter+1];  // using let to bind
        }
        counter = counter +1 ;
    } 
    // println!("{:?}",api_name_vec); // test
    return name.to_string().replace("\"","");
}

pub fn memory_names(length: usize, time: usize){
    let terminal = terminal::stdout();
    let mut random_names: &str = "";
    let mut names_vec: Vec<String> = vec![];  // vector of clean random names
    let mut random_names_vec: Vec<String> = vec![];

    // loop to get {number_of_names} random names
    for number in 0..length
    {
        random_names_vec.push(random_name());
        // println!("{:?}", random_names_vec); // test
    }

    // wait for {time} seconds
    let mem_time_u64 = (length*time).try_into().unwrap();
    let mem_dur = std::time::Duration::from_secs(mem_time_u64);
    
    println!("Memorise this:\n{:?}\nYou have {} seconds.", random_names_vec, length*time);
    
    thread::sleep(mem_dur);
    
    terminal.act(Action::ClearTerminal(Clear::All)).map_err(|err| println!("{:?}", err)).ok();
    
        

    // user prompt
    print!("Type in what you memorised!\nTo exit, enter: exit\n");

    // comparing input and random_names_vec
    let mut counter_check_ans = 0;

    for _item in &random_names_vec {
        let user_input = input_wrapper::get_input();
        if user_input.to_uppercase() == random_names_vec[counter_check_ans].to_uppercase(){
            println!("Bravo! You got {} right!\n", random_names_vec[counter_check_ans]);
        } else if user_input == "exit" {
            break;
        } else {
            println!("Nay :( The right answer is {}.\n", random_names_vec[counter_check_ans]);
        };

    counter_check_ans = counter_check_ans +1;

    }

    println!("end");
 
}



    


