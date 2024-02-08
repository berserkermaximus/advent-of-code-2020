use std::fs;

fn calculate_trees(array: &Vec<Vec<char>>, dx: usize, dy: usize) -> i64{
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;
    while y < array.len()-1 {
        x += dx;
        y += dy;
        if array[y][x % array[0].len()] == '#' {
            trees += 1;
        }
    }

    return trees;
}

fn main() {
    let content = fs::read_to_string("03/input.txt").expect("expected a file");
    let array: Vec<Vec<char>> = content.split('\n').map(|line| line.chars().collect()).collect();
    
    let dx: [usize; 5] = [1, 3, 5, 7, 1];
    let dy: [usize; 5] = [1, 1, 1, 1, 2];

    let mut multi_tree = 1;

    for i in 0..5 {
        multi_tree *= calculate_trees(&array, dx[i], dy[i]);
    }

    println!("{}", multi_tree);
}