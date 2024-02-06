use std::fs;
use regex::Regex;

fn main(){
    let content = fs::read_to_string("02/input.txt").expect("expected a file.");

    let re = Regex::new(r"(?<min>[0-9]+)-(?<max>[0-9]+) (?<letter>[a-zA-Z]): (?<word>[a-zA-Z]+)").unwrap();
    let mut correct_passwords = 0;
    for line in content.split('\n') {
        let Some(caps) = re.captures(line) else {
            println!("no match!");
            continue;
        };

        let mut count = 0;
        let word = &caps["word"];
        let min: i32 = caps["min"].to_string().parse().unwrap();
        let max: i32 = caps["max"].to_string().parse().unwrap();
        for l in word.chars() {
            if l == caps["letter"].chars().nth(0).unwrap() {
                count += 1;
            }
        }
        if count >= min && count <= max {
            correct_passwords += 1;
        }
    }

    println!("{}", correct_passwords);

}