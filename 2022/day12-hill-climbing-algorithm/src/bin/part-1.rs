use day12_hill_climbing_algorithm::part_1;
use std::fs;

fn main() {
    println!("{}", part_1(&fs::read_to_string("./input.txt").unwrap()));
}
