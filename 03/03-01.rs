use std::fs;

fn main() {
    let content = fs::read_to_string("03/input.txt").expect("expected a file");
    let array: Vec<Vec<char>> = content.split('\n').map(|line| line.chars().collect()).collect();
    
    let mut x = 0;
    let dx = 3;

    let mut trees = 0;
    for i in 1..array.len() {
        x += dx;
        if array[i][x % array[0].len()] == '#' {
            trees += 1;
        }
    }

    println!("{}", trees);
}