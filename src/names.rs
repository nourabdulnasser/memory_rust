use std::{thread, time};
use terminal::{Clear, Action};
extern crate fakedata_generator;
use fakedata_generator::*;
use super::input_wrapper;

fn get_random_word() -> String{
    // function  to generate a random word
    let contents = "squealing ,wide ,secret ,ball ,wholesale ,gaudy ,distance ,notice ,delicate ,nondescript ,abashed ,mice ,momentous ,charge ,vast ,power ,smell ,wander ,vague ,peck ,bad ,sharp ,company ,channel ,sack ,sisters ,grate ,silly ,judicious ,radiate ,marked ,murky ,clear ,admit ,cannon ,handsome ,arm ,walk ,mourn ,work ,nimble ,dress ,juvenile ,cent ,foamy ,tense ,wistful ,hypnotic ,children ,thread ,cowardly ,hungry ,highfalutin ,wakeful ,fairies ,boast ,unarmed ,beginner ,breathe ,celery ,spiffy ,worthless ,jail ,wonder ,serious ,ice ,women ,ignore ,wax ,homely ,disagree ,cactus ,unwritten ,voracious ,avoid ,broken ,suffer ,cellar ,word ,tasteless ,same ,rain ,few ,babies ,entertaining ,complete ,arrive ,hard ,ignore ,talk ,elite ,meaty ,exciting ,undress ,surprise ,tall ,substantial ,mammoth ,rub ,prepare ,saw ,stream ,smell ,crib ,elite ,farm ,snow ,fixed ,famous ,fluttering ,rings ,gorgeous ,alcoholic ,wandering ,exchange ,hook ,cannon ,mass ,close ,gaudy ,late ,sparkle ,dime ,screw ,nonchalant ,careful ,level ,destruction ,earthy ,rush ,obedient ,nostalgic ,statement ,easy ,chicken ,fresh ,top ,wait ,hat ,week ,pack ,remember ,statement ,cagey ,pig ,rhythm ,functional ,periodic ,squirrel ,reminiscent ,quack ,reason ,messy ,brake ,miniature ,late ,juicy ,unused ,quartz ,plan ,damaged ,pest ,punch ,sneaky ,roasted ,inform ,naive ,real ,glove ,morning ,lace ,stupid ,grease ,pricey ,appliance ,ragged ,careful ,color ,vacuous ,invite ,breakable ,roof ,exchange ,month ,loving ,car ,trust ,mark ,poor ,shave ,breathe ,freezing ,cable ,evanescent ,wilderness ,hop ,alleged ,gray ,periodic ,political ,piquant ,used ,cowardly ,search ,playground ,coach ,savory ,scandalous ,valuable ,axiomatic ,burst ,file ,gusty ,uneven ,normal ,pray ,sprout ,trains ,reflective ,imaginary ,delay ,slip ,internal ,please ,leg ,tent ,tickle ,poor ,whispering ,cows ,relieved ,rambunctious ,breath ,undress ,tramp ,flimsy ,guiltless ,glistening ,immense ,greedy ,behavior ,funny ,work ,huge ,children ,electric ,shivering ,smoke ,regular ,hum ,hard-to-find ,boot ,multiply ,chemical ,silent ,quixotic ,pinch ,things ,nondescript ,frighten ,visitor ,early ,violent ,acoustic ,jagged ,impolite ,fuzzy ,wanting ,far ,channel ,adjoining ,cultured ,bore ,bottle ,subsequent ,frogs ,pets ,direction ,expensive ,exciting ,basket ,polish ,sound ,flagrant ,kind ,jelly ,mammoth ,cemetery ,early ,incredible ,employ ,knee ,acrid ,basin ,angle ,efficacious ,regret ,exuberant ,downtown ,brake ,coherent ,pink ,spicy ,gainful ,secretive ,gigantic ,request ,argument ,beneficial ,quiver ,roof ,orange ,basin ,chalk ,great ,nosy ,eight ,narrow ,ludicrous ,sigh ,trucks ,itch ,end ,flippant ,miscreant ,care ,slow ,appreciate ,alike ,mature ,sassy ,dog ,wacky ,forgetful ,volcano ,introduce ,absurd ,tested ,observant ,tested ,sock ,ship ,kind ,man ,plantation ,strange ,glow ,thankful ,interest ,drab ,excellent ,pump ,chance ,hate ,live ,enthusiastic ,cast ,green ,spiffy ,nonchalant ,hobbies ";


    let random_word = gen_enum(contents.to_string());
    random_word
}










pub fn memory_words(length: usize,sleep_per_word: usize) {

    let terminal = terminal::stdout();
    let mut word_set: Vec<String> = Vec::with_capacity(length);
    let sleep_time = sleep_per_word*length;
    let sleep_time_u64: u64 = sleep_time as u64;

    // for every word the user wants, generate a random word and push onto the word_set vector
    for _i in 0..length {
        let a_new_word = get_random_word();
        word_set.push(a_new_word);
    }


    let random_word_string: String = word_set.into_iter().collect();
    // turn random word vector into a string

    println!("{}",random_word_string);
    println!("you have {} amount of second(s) to memorise these words",sleep_time);
    thread::sleep(time::Duration::from_secs(sleep_time_u64));


     terminal.act(Action::ClearTerminal(Clear::All)).map_err(|err| println!("{:?}", err)).ok();

     println!("okay print what you memorised");


     let mut guess_trim: String = input_wrapper::get_input().to_owned();

     guess_trim.push_str(" ");


     if guess_trim == random_word_string {

         println!("well done you got it correct, the words were {}",random_word_string);

     }
     else {
         println!("wow you're trash, the words were {}",random_word_string);


     }



}
