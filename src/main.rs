use std::env;
use std::process;

fn reverse_str(original_str: String) -> String {

    let original_characters: Vec<char> = original_str.chars().collect();
    let mut reversed_str = String::from("");

    for i in (0..original_characters.len()).rev() {
        reversed_str.push(original_characters[i]);
    }

    reversed_str
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please enter exactly one argument");
        process::exit(-1); 
    }

    let original_str = args[1].clone();
    let reversed_str = reverse_str(original_str);

    println!("{}", reversed_str);
}

