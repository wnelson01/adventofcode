use day9_rope_bridge::part_2;
use std::fs;

fn main() {
    println!("{}", part_2(&fs::read_to_string("./input.txt").unwrap()));
}
