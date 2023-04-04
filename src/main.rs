use std::io::{stdin, BufRead, BufReader};
use std::fs::File;
use std::str::Split;

fn main() {
    // Initialize the word library
    let dict = read_ref();

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

// Need testing
fn osa_dist(s1: &str, s2: &str) -> usize { 
    let mut dp = [vec![0; s1.len()+1], vec![0; s2.len()+1]];
    let b1: &[u8] = s1.as_bytes();
    let b2: &[u8] = s2.as_bytes();

    for i in 0..s1.len()+1 {
        dp[i][0] = i;
    }
    for j in 0..s2.len()+1 {
        dp[0][j] = j;
    }

    for i in 1..s1.len()+1 {
        for j in 1..s2.len()+1 {
            if &b1[i-1] == &b2[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + [dp[i-1][j], dp[i][j-1], dp[i-1][j-1]].iter().min().unwrap();
            }
        }
    }
    return dp[s1.len()][s2.len()]
}
