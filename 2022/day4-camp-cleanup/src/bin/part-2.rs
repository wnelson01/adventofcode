use day4_camp_cleanup::process_part2;
use std::fs;

fn main() {
    let assignments = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&assignments));
}