use day7_no_space_left_on_device::part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", part_1(&input));
}
