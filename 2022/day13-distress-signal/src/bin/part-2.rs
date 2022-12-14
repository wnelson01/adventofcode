use day13_distress_signal::part_2;
use std::fs;

fn main() {
    println!("{}", part_2(&fs::read_to_string("./input.txt").unwrap()));
}
