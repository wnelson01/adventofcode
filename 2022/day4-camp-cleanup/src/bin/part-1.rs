use day4_camp_cleanup::process_part1;
use std::fs;

fn main() {
    let assignments = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&assignments));
}