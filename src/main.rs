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
use greetings::{english, french, spanish};

use crate::how_to_hold_data_for_operations::{
    //derived::user_defined::user_def,
    primitives::{
        compound::{comp, main2, main6, main7, miracle, multiplier /*analyze_slice*/},
        scalar::{main3 /*,main9 */, scaler, scaler2, scaler3, scaler4},
    },
};

fn main() {
    println!("Hello, world!");
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", english::default_greeting());
    println!("{}", english::default_greeting2());

    //println!("{}", user_def());
    println!("{}", scaler());
    println!("{}", comp());
    println!("{}", scaler2());
    println!("{}", scaler3());

    scaler4();

    miracle();
    main2();
    main3();
    main7();
    //analyze_slice(15);
    main6();
    // main9();
    println!("{}", multiplier(&[7.0, 8.6, 9.98, 12.22, 34.00]));
    how_to_hold_data_for_operations::derived::user_defined::run();
}
