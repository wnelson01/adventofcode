use day8_treetop_tree_house::part_2;
use std::fs;

fn main() {
    println!("{}", part_2(&fs::read_to_string("./input.txt").unwrap()));
}
