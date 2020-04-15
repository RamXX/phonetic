// Phonetic. A program to convert an interactive text input into its components
// using the NATO phonetic alphabet. This is useful on phone conversations with 
// customer service people, if you need to spell something.
//
// usage: phonetic <text>

use std::env;

mod phonetic;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nargs = args.len();

    if nargs > 1 && &args[1] != "-m" {
        for i in 1..nargs {
            println!("\n[Word \"{}\" follows next]", &args[i]);
            for p in phonetic::nato_spell(&args[i]).iter() {
                println!("{}", p);
            }
        }
    } else if nargs > 2 && &args[1] == "-m" {
        for i in 2..nargs {
            println!("\n[Word \"{}\" follows next]", &args[i]);
            for p in phonetic::morse(&args[i]).iter() {
                println!("{}", p);
            }
        }
    } else {
        println!("USAGE: phonetic [-n] <text>");
    };
}

