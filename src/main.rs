// fn main() {
//     println!("I dont know why but i feel a bit RUSTY");
// }

// mod greetings;
// use greetings::*;
// fn main() {
//     println!("Hello, world!");
//     println!("{}", default_greeting());
//     println!("{}", default_greeting2());
// }

///associate greetings module with this crate
mod greetings;
mod how_to_hold_data_for_operations;      

extern crate my_project_name_lib;

///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greetings::{french, spanish, english};

fn main() {
    println!("Hello, world!");
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", english::default_greeting());
    println!("{}", english::default_greeting2());
}
