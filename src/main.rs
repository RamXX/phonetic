// Phonetic. A program to convert an interactive text input into its components
// using the NATO phonetic alphabet. This is useful on phone conversations with 
// customer service people, if you need to spell something.
//
// usage: phonetic <text>

use std::env;

mod textutil;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nargs = args.len();

    if nargs > 1 {
        for i in 1..nargs {
            println!("---");
            for p in textutil::nato_spell(&args[i]).iter() {
                println!("{}", p);
            }
        }
    } else {
        println!("USAGE: phonetic <text>");
    };
}

