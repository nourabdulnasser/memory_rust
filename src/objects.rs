use std::{thread, time};
use terminal::{Clear, Action};
extern crate fakedata_generator;
use fakedata_generator::*;
use super::input_wrapper;

fn get_random_object() -> String{
    // function  to generate a random object
    let objects = "tiger ,bottle of honey ,water bottle ,bottle of ink ,book ,empty tin can ,toilet paper tube ,toilet ,rolling pin ,hair clip ,crowbar ,can of chili ,food ,thimble ,roll of toilet paper ,chalk ,sticky note ,glow stick ,handheld game system ,flowers ,table ,flyswatter ,sword ,speakers ,purse ,craft book ,purse/bag ,milk ,slipper ,pair of socks ,toy boat ,ipod ,rat ,fork ,bangle bracelet ,bottle of soda ,eraser ,microphone ,paintbrush ,pepper shaker ,keyboard ,flag ,box of markers ,mp3 player ,panda ,keychain ,jar of pickles ,basketball ,mirror ,hair ribbon ,cell phone ,plush pony ,zebra ,trucks ,feather duster ,jigsaw puzzle ,bouquet of flowers ,bottle of syrup ,spool of wire ,street lights ,miniature portrait ,small pouch ,cat ,extension cord ,fishing hook ,book of matches ,turtle ,apple ,snowglobe ,water ,box ,pencil holder ,box of tissues ,key chain ,pool stick ,mop ,cookie jar ,ladle ,dolphin ,ipod charger ,marble ,lace ,camera ,hair pin ,credit card ,sponge ,clothes pin ,egg beater ,plate ,box of baking soda ,magazine ,trash bag ,tweezers ,catalogue ,belt ,puddle ,hanger ,twister ,box of crayons ,pillow ,plastic fork ,money ,tube of lipstick ,umbrella ,hair tie ,mouse pad ,locket ,dog ,tire swing ,candy wrapper ,lamp ,wristwatch ,soccer ball ,squirt gun ,spatula ,bowl ,bottle of oil ,stick of incense ,key ,pair of binoculars ,frying pan ,zipper ,pair of scissors ,cork ,shawl ,multitool ,clock ,nail ,bottle of nail polish ,letter opener ,notepad ,tv ,plush frog ,perfume ,tea cup ,toy robot ,bag of rubber bands ,bottle of lotion ,cement stone ,squirrel ,toy car ,shampoo ,sticker book ,spool of string ,game cartridge ,butter knife ,crow ,tea pot ,comic book ,whale ,quartz crystal ,egg timer ,can of beans ,domino set ,bottle of sunscreen ,television ,chicken ,pocketwatch ,piece of gum ,vase ,bottle of paint ,tennis ball ,socks ,desk ,clay pot ,plush rabbit ,cup ,word search ,bag ,lotion ,salt shaker ,container of pudding ,face wash ,remote ,scotch tape ,beef ,baseball hat ,shovel ,snail shell ,nail clippers ,thermometer ,pop can ,pants ,straw ,wrench ,steak knife ,bag of popcorn ,wallet ,bell ,phone ,card ,pocketknife ,bottle of glue ,laser pointer ,sand paper ,lemon ,toy plane ,plush octopus ,model car ,orange ,sandal ,lamp shade ,lip gloss ,rock ,box of Q-tips ,carrots ,wine glass ,toe ring ,ball of yarn ,picture frame ,deodorant ,light ,sofa ,rusty nail ,music CD ,drill press ,video games ,blouse ,wooden spoon ,can of whipped cream ,canvas ,sheep ,empty jar ,monitor ,carton of ice cream ,floor ,rubber duck ,fake flowers ,screw ,pen ,box of chalk ,drawer ,tissue box ,bottle of perfume ,couch ,canteen ,CD ,boom box ,pearl necklace ,lion ,chapter book ,paperclip ,bread ,martini glass ,book of jokes ,giraffe ,box of chocolates ,stick ,shirt ,spectacles ,bookmark ,bonesaw ,pair of earrings ,light bulb ,tomato ,broccoli ,rubber band ,seat belt ,shirt button ,candlestick ,garden spade ,pair of dice ,pair of sunglasses ,white out ,dictionary ,bottle ,plush bear ,plush cat ,comb ,spool of ribbon ,lime ,bottle of pills ,toy soldier ,photo album ,baseball ,sailboat ,empty bottle ,candle ,chocolate ,carrot ,chenille stick ,few batteries ,pinecone ,tube of lip balm ,door ,leg warmers ,sun glasses ,radio ,cucumber ,scarf ,helmet ,button ,toy top ,hammer ,window ,acorn ,roll of gauze ,watch ,children's book ";

    let random_object = gen_enum(objects.to_string());
    random_object
}

pub fn memory_objects(length: usize,sleep_per_object: usize) {

    let terminal = terminal::stdout();
    let mut object_set: Vec<String> = Vec::with_capacity(length);
    let sleep_time = sleep_per_object*length;
    let sleep_time_u64: u64 = sleep_time as u64;

    // for every object the user wants, generate a random object and push onto the object_set vector
    for _i in 0..length {
        let a_new_object = get_random_object();
        object_set.push(a_new_object+"- ");
    }


    let random_object_string: String = object_set.into_iter().collect();
    // turn random object vector into a string

    println!("{}",random_object_string);
    println!("you have {} amount of second(s) to memorise these objects",sleep_time);
    thread::sleep(time::Duration::from_secs(sleep_time_u64));


     terminal.act(Action::ClearTerminal(Clear::All)).map_err(|err| println!("{:?}", err)).ok();

     println!("okay print what you memorised");


     let mut guess_trim: String = input_wrapper::get_input().to_owned();

     guess_trim.push_str(" - ");


     if guess_trim == random_object_string {

         println!("well done you got it correct, the objects were {}",random_object_string);

     }
     else {
         println!("wow you're trash, the objects were {}",random_object_string);
     }



}

