pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn process_part1(assignments: &str) -> i32 {
    assignments.lines().fold(0, |acc, pairs| {
        println!("overlaps: {}", acc);
        let assignment = pairs.split_once(",").unwrap();
        println!("assignment: {:?}", assignment);
        let elf_1 = assignment.0.split_once("-").unwrap();
        println!("elf 1: {:?}", elf_1);
        let elf_2 = assignment.1.split_once("-").unwrap();
        println!("elf 2: {:?}", elf_2);
        let elf_1_start: i32 = elf_1.0.parse().unwrap();
        let elf_1_stop: i32 = elf_1.1.parse().unwrap();
        let elf_2_start: i32 = elf_2.0.parse().unwrap();
        let elf_2_stop: i32 = elf_2.1.parse().unwrap();
        if elf_1_start >= elf_2_start && elf_1_stop <= elf_2_stop {
            println!("overlap, {} >= {} && {} <= {}", elf_1.0, elf_2.0, elf_1.1, elf_2.1);
            acc + 1
        } else if elf_2_start >= elf_1_start && elf_2_stop <= elf_1_stop {
            println!("overlap, {} >= {} && {} <= {}", elf_2.0, elf_1.0, elf_2.1, elf_1.1);
            acc + 1
        } else {
            acc
        }
    })
}

pub fn process_part2(assignments: &str) -> i32 {
    assignments.lines().fold(0, |acc, pairs| {
        println!("overlaps: {}", acc);
        let assignment = pairs.split_once(",").unwrap();
        println!("assignment: {:?}", assignment);
        let elf_1 = assignment.0.split_once("-").unwrap();
        println!("elf 1: {:?}", elf_1);
        let elf_2 = assignment.1.split_once("-").unwrap();
        println!("elf 2: {:?}", elf_2);
        let elf_1_start: i32 = elf_1.0.parse().unwrap();
        let elf_1_stop: i32 = elf_1.1.parse().unwrap();
        let elf_2_start: i32 = elf_2.0.parse().unwrap();
        let elf_2_stop: i32 = elf_2.1.parse().unwrap();
        if elf_1_start >= elf_2_start && elf_1_start <= elf_2_stop {
            println!("overlap, {} >= {} && {} <= {}", elf_1.0, elf_2.0, elf_1.1, elf_2.1);
            acc + 1
        } else if elf_2_start >= elf_1_start && elf_2_start <= elf_1_stop {
            println!("overlap, {} >= {} && {} <= {}", elf_2.0, elf_1.0, elf_2.1, elf_1.1);
            acc + 1
        } else {
            acc
        }
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
