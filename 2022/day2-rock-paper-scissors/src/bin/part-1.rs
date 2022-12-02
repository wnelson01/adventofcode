use day2_rock_paper_scissors::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
}