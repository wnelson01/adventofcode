use std::collections::HashSet;

pub fn highest_y_coordinate(at_rest: &HashSet<(i32, i32)>) -> i32 {
    let mut y_coordinates: Vec<i32> = at_rest.iter().map(|coordinate| coordinate.1).collect();
    y_coordinates.sort();
    *y_coordinates.last().unwrap()
}

pub fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut at_rest: HashSet<(i32, i32)> = HashSet::new();
    input.lines().for_each(|line| {
        line.split(" -> ")
            .collect::<Vec<&str>>()
            .windows(2)
            .for_each(|window| {
                let rock1 = window[0].split_once(',').unwrap();
                let rock2 = window[1].split_once(',').unwrap();
                let x1 = i32::min(
                    rock1.0.parse::<i32>().unwrap(),
                    rock2.0.parse::<i32>().unwrap(),
                );
                let y1 = i32::min(
                    rock1.1.parse::<i32>().unwrap(),
                    rock2.1.parse::<i32>().unwrap(),
                );
                let x2 = i32::max(
                    rock1.0.parse::<i32>().unwrap(),
                    rock2.0.parse::<i32>().unwrap(),
                );
                let y2 = i32::max(
                    rock1.1.parse::<i32>().unwrap(),
                    rock2.1.parse::<i32>().unwrap(),
                );
                (x1..=x2).for_each(|x| {
                    at_rest.insert((x, y1));
                });
                (y1..=y2).for_each(|y| {
                    at_rest.insert((x1, y));
                });
            });
    });
    at_rest
}

pub fn drop_sand_part_1(at_rest: &HashSet<(i32, i32)>) -> Result<(i32, i32), &str> {
    let mut sand = (500, 0);
    let y = highest_y_coordinate(at_rest);
    while !at_rest.contains(&(sand.0, sand.1 + 1))
        || !at_rest.contains(&(sand.0 - 1, sand.1 + 1))
        || !at_rest.contains(&(sand.0 + 1, sand.1 + 1))
    {
        if !at_rest.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !at_rest.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1)
        } else if !at_rest.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1)
        }
        if sand.1 > y {
            return Err("at rest");
        }
    }
    Ok(sand)
}

pub fn part_1(input: &str) -> i32 {
    let mut at_rest = parse(input);
    let mut count = 0;
    while let Ok(sand) = drop_sand_part_1(&at_rest) {
        at_rest.insert(sand);
        count += 1;
    }
    count
}

pub fn drop_sand_part_2(at_rest: &HashSet<(i32, i32)>, y: i32) -> Result<(i32, i32), &str> {
    let mut sand = (500, 0);
    while !at_rest.contains(&(sand.0, sand.1 + 1))
        || !at_rest.contains(&(sand.0 - 1, sand.1 + 1))
        || !at_rest.contains(&(sand.0 + 1, sand.1 + 1))
    {
        if sand.1 == y - 1 {
            return Ok(sand);
        }
        if !at_rest.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !at_rest.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1)
        } else if !at_rest.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1)
        }
    }
    Ok(sand)
}

pub fn part_2(input: &str) -> i32 {
    let mut at_rest = parse(input);
    let y = highest_y_coordinate(&at_rest) + 2;
    let mut count = 0;
    while let Ok(sand) = drop_sand_part_2(&at_rest, y) {
        count += 1;
        if sand == (500, 0) {
            return count;
        }
        at_rest.insert(sand);
    }
    count
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, fs};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_parse() {
        let result = parse(&fs::read_to_string("./example.txt").unwrap());
        assert_eq!(
            result,
            HashSet::from([
                (494, 9),
                (495, 9),
                (496, 9),
                (497, 9),
                (498, 9),
                (499, 9),
                (500, 9),
                (501, 9),
                (502, 9),
                (502, 8),
                (502, 7),
                (502, 6),
                (502, 5),
                (502, 4),
                (503, 4),
                (496, 6),
                (497, 6),
                (498, 6),
                (498, 5),
                (498, 4)
            ])
        );
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./example.txt").unwrap()), 24);
    }

    #[test]
    fn input_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./input.txt").unwrap()), 897);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&fs::read_to_string("./example.txt").unwrap()), 93);
    }
}
