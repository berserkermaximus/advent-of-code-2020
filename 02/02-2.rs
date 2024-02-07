use std::fs;
use regex::Regex;

fn main(){
    let content = fs::read_to_string("03/input.txt").expect("expected a file.");

    let re = Regex::new(r"(?<min>[0-9]+)-(?<max>[0-9]+) (?<letter>[a-zA-Z]): (?<word>[a-zA-Z]+)").unwrap();
    let mut correct_passwords = 0;
    for line in content.split('\n') {
        let Some(caps) = re.captures(line) else {
            println!("no match!");
            continue;
        };
        
        let word = &caps["word"];
        let min: usize = caps["min"].parse::<i32>().unwrap() as usize;
        let max: usize = caps["max"].parse::<i32>().unwrap() as usize;
        let letter = caps["letter"].chars().nth(0).unwrap();
        
        if (word.chars().nth(min - 1).unwrap() == letter) ^ (word.chars().nth(max - 1).unwrap() == letter) {
            correct_passwords += 1;
        }
    }

    println!("{}", correct_passwords);

}