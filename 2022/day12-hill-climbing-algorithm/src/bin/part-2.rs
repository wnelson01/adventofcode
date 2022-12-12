use day12_hill_climbing_algorithm::part_2;
use std::fs;

fn main() {
    println!("{}", part_2(&fs::read_to_string("./input.txt").unwrap()));
}
