use std::collections::HashSet;

static _EX_INP:  &str  = "abc

a
b
c

ab
ac

a
a
a
a

b";

fn main() {
    let puzzle_input = include_str!("../input.txt");
    part1(puzzle_input);
    part2(puzzle_input);
}
fn part1(input_str: &str) {
    println!("Part 1");
    let inputs: Vec<&str> = input_str.split("\r\n\r\n").collect();
    let mut sum = 0;
    for group in inputs {
        let mut votes: HashSet<char> = HashSet::new();
        for char in group.chars() {
            if char >= 'a' && char <= 'z' {
                votes.insert(char);
            }
        }
        println!("* {:?} {}", votes, votes.len());
        sum += votes.len();
    }
    println!("Sum = {}", sum);
}
fn part2(input_str: &str) {
    println!("Part 2");
    let inputs: Vec<&str> = input_str.split("\r\n\r\n").collect();
    let mut sum = 0;
    for group in inputs {
        let mut votes: HashSet<char> = HashSet::new();
        let mut first_line = true;
        for line in group.lines() {
            // println!("{}",line);
            let mut individual_votes: HashSet<char> = HashSet::new();
            for char in line.chars() {
                individual_votes.insert(char);                
            }
            if first_line {
                first_line = false;
                votes.extend(&individual_votes);
            } else {
                votes = votes.intersection(&individual_votes).copied().collect();
            }
            // println!("votes:{:?} {}", votes, votes.len());
        }
        println!("* {:?} {}", votes, votes.len());
        sum += votes.len();
    }
    println!("Sum = {}", sum);
}
