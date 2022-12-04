use day3_rucksack_reorganization::process_part2;
use std::fs;

fn main() {
    let rucksacks = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&rucksacks));
}