use day2_rock_paper_scissors::process_part2;
use std::fs;

fn main() {
    let strategy_guide = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part2(&strategy_guide));
}