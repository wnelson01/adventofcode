use day11_monkey_in_the_middle::part_1;
use std::fs;

fn main() {
    println!("{}", part_1(&fs::read_to_string("./input.txt").unwrap()));
}
