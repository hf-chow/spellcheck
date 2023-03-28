use std::io::{stdin, BufRead, BufReader};
use std::fs::File;
use std::str::Split;

fn main() {
    // Initialize the word library
    read_ref();

    let mut s = String::new();
    println!("Please type or paste what you want to check");

    stdin().read_line(&mut s).expect("Please check if the input is a string");
    println!("You have entered {s}.");
    
    let sub_s: Split<&str> = s.split(" ");
    println!("Your input contains the following words:");
    for i in sub_s {
        println!("{i}");
    }
}

fn read_ref() -> Vec<String> {
    let file = File::open("ref.txt")
        .expect("File not found, please check if the file \"ref.txt\" exists.");
    let reader = BufReader::new(file);

    let sub_s: Vec<String> = reader
        .lines()
        .map(|s| s.unwrap())
        .collect();

    return sub_s
}
