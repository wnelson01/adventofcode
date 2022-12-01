use std::fs;

fn main() {
    let binding = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut calories = binding
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|food| food.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    calories.sort();
    calories.reverse();
    println!("Part 1: {:?}", calories[0]);
    println!("Part 2: {:?}", calories[0] + calories[1] + calories[2]);
}
