// Slope stuff

use std::fs;



fn count_trees(slope: &Vec<Vec<bool>>, dx: usize, dy: usize) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut n_trees = 0;
    let width = slope[0].len();
    let height = slope.len();
    while y < height {
        if slope[y][x] {
            n_trees += 1;
        }
        x += dx;
        x %= width;
        y += dy;
    }
    return n_trees;
}

fn part_one(slope: &Vec<Vec<bool>>) {
    let trees = count_trees(&slope, 3, 1);
    println!("Encountered {} trees", trees);
}

fn part_two(slope: &Vec<Vec<bool>>) {
    let mut cases: Vec<u32> = Vec::<u32>::new();
    cases.push(count_trees(&slope, 1, 1));
    cases.push(count_trees(&slope, 3, 1));
    cases.push(count_trees(&slope, 5, 1));
    cases.push(count_trees(&slope, 7, 1));
    cases.push(count_trees(&slope, 1, 2));
    let product:u32 = cases.iter().product();
    println!("Part 2, key is: {}", product);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("cant open file");

    let mut slope = Vec::<Vec<bool>>::new();
    for line in input.lines() {
        let row: Vec<bool> = 
            line.chars().map(|c| c == '#').collect();
        slope.push(row);
    }
    part_one(&slope);
    part_two(&slope);
}
