use day3_rucksack_reorganization::process_part1;
use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&rucksacks));
}