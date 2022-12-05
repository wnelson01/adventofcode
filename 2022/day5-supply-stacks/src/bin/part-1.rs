use day5_supply_stacks::process_part1;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&input));
}
