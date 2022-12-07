use std::collections::HashMap;

pub fn check_window(window: &[(usize, char)]) -> bool {
    let mut character_counts = HashMap::new();
    for (_, c) in window.iter() {
        *character_counts.entry(c).or_insert(0) += 1;
        if character_counts[&c] > 1 {
            return false;
        }
    }
    true
}

pub fn process_windows(input: &str, size: usize) -> usize {
    for window in input
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(size)
    {
        if check_window(window) {
            return window.last().unwrap().0 + 1;
        }
    }
    0
}

pub fn process_part1(input: &str) -> usize {
    process_windows(input, 4)
}

pub fn process_part2(input: &str) -> usize {
    process_windows(input, 14)
}

#[cfg(test)]
mod tests {
    use crate::{process_part1, process_part2};

    #[test]
    fn part_1_example_1() {
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn part_1_example_2() {
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn part_1_example_3() {
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn part_1_example_4() {
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn part_2_example_3() {
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn part_2_example_4() {
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn part_2_example_5() {
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
