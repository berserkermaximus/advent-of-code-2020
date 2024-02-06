use std::fs;

fn main() {
    let contents = fs::read_to_string("01/input.txt")
        .expect("Should have been able to read the file");

    let numbers: Vec<i32> = contents.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut hashmap: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in &numbers {
        hashmap.insert(*i, 1);
    }

    for i in &numbers {
        let t = 2020 - *i;
        if hashmap.get(&t).is_some() {
            println!("{}", t * *i);
            break;
        }
    }
}