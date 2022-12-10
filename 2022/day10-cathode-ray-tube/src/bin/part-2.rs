use day10_cathode_ray_tube::part_2;
use std::fs;

fn main() {
    part_2(&fs::read_to_string("./input.txt").unwrap());
}
