use std::collections::HashSet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn find_common_item(compartment_1: &str, compartment_2: &str) -> char {
    let a: HashSet<char> = compartment_1.chars().collect();
    let b: HashSet<char> = compartment_2.chars().collect();
    let intersection = a.intersection(&b);
    intersection.last().unwrap().clone()
}

pub fn find_priority(item: char) -> u32 {
    let priority = item as u32;
    match priority {
        65..=90 => priority - 38,
        97..=122 => priority - 96,
        _ => 0
    }
}

pub fn process_part1(rucksacks: &str) -> u32 {
    rucksacks.lines().fold(0, |acc, rucksack| {
        let compartment_1 = &rucksack[0..rucksack.len() / 2];
        let compartment_2 = &rucksack[rucksack.len() / 2..rucksack.len()];
        let common_item = find_common_item(compartment_1, compartment_2);
        let priority = find_priority(common_item);
        acc + priority
    })
}

pub fn process_part2(rucksacks: &str) -> u32 {
    rucksacks.lines().collect::<Vec<&str>>().chunks(3).fold(0,|acc, group| {
        let a: HashSet<char> = group[0].chars().collect();
        let b: HashSet<char> = group[1].chars().collect();
        let c: HashSet<char> = group[2].chars().collect();
        let a_b_intersection = String::from_iter(a.intersection(&b).collect::<Vec<&char>>()).chars().collect::<HashSet<char>>();
        let a_b_c_intersection = a_b_intersection.intersection(&c).last().unwrap().clone();
        let priority = find_priority(a_b_c_intersection);
        acc + priority
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
