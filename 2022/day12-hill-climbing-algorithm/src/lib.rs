use std::collections::HashSet;
use std::collections::VecDeque;

pub fn shortest_path(
    heightmap: &mut Vec<Vec<char>>,
    starting: (usize, usize),
    ending: (usize, usize),
) -> i32 {
    let mut deque: VecDeque<(usize, usize)> = VecDeque::from([starting]);
    let mut level: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::from([starting]);
    let mut steps = -1;
    while !deque.is_empty() {
        deque.drain(0..).for_each(|step| {
            level.push_back(step);
        });
        steps += 1;
        while let Some(step) = level.pop_front() {
            let (y, x) = step;
            if (y, x) == ending {
                return steps;
            }
            if y > 0
                && heightmap[y - 1][x] as u32 <= heightmap[y][x] as u32 + 1
                && !visited.contains(&(y - 1, x))
            {
                visited.insert((y - 1, x));
                deque.push_back((y - 1, x));
            }
            if y < heightmap.len() - 1
                && heightmap[y + 1][x] as u32 <= heightmap[y][x] as u32 + 1
                && !visited.contains(&(y + 1, x))
            {
                visited.insert((y + 1, x));
                deque.push_back((y + 1, x));
            }
            if x > 0
                && heightmap[y][x - 1] as u32 <= heightmap[y][x] as u32 + 1
                && !visited.contains(&(y, x - 1))
            {
                visited.insert((y, x - 1));
                deque.push_back((y, x - 1));
            }
            if x < heightmap[0].len() - 1
                && heightmap[y][x + 1] as u32 <= heightmap[y][x] as u32 + 1
                && !visited.contains(&(y, x + 1))
            {
                visited.insert((y, x + 1));
                deque.push_back((y, x + 1));
            }
        }
    }
    i32::MAX
}

pub fn part_1(input: &str) -> i32 {
    let mut heightmap: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starting = (0, 0);
    let mut ending = (0, 0);
    (0..heightmap.len()).for_each(|y| {
        (0..heightmap[0].len()).for_each(|x| {
            if heightmap[y][x] == 'S' {
                starting = (y, x);
            }
            if heightmap[y][x] == 'E' {
                ending = (y, x);
            }
        });
    });
    heightmap[starting.0][starting.1] = 'a';
    heightmap[ending.0][ending.1] = 'z';
    shortest_path(&mut heightmap, starting, ending)
}

pub fn part_2(input: &str) -> i32 {
    let mut heightmap: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut starting = Vec::new();
    let mut ending = (0, 0);
    (0..heightmap.len()).for_each(|y| {
        (0..heightmap[0].len()).for_each(|x| {
            if heightmap[y][x] == 'a' {
                starting.push((y, x));
            }
            if heightmap[y][x] == 'E' {
                ending = (y, x);
            }
        });
    });
    heightmap[ending.0][ending.1] = 'z';
    let mut lowest = starting
        .iter()
        .map(|start| shortest_path(&mut heightmap, *start, ending))
        .collect::<Vec<i32>>();
    lowest.sort();
    lowest[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(&fs::read_to_string("./example.txt").unwrap()), 31);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./input.txt").unwrap()), 449);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(&fs::read_to_string("./example.txt").unwrap()), 29);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&fs::read_to_string("./input.txt").unwrap()), 443);
    }
}
