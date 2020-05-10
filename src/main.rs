use std::env;

mod phonetic;

/// Phonetic. A program to convert an interactive text input into its components
/// using the NATO phonetic alphabet, or morse code when used with -m. 
///
/// usage: phonetic [-m] <text>
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
        println!("USAGE: phonetic [-m] <text>");
    };
}

