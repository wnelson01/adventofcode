use std::collections::HashSet;

#[derive(Clone, Copy, Default, Debug, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Debug)]
struct Knot {
    position: Position,
    history: Vec<Position>,
}

impl Default for Knot {
    fn default() -> Knot {
        Knot {
            position: Position { x: 0, y: 0 },
            history: vec![Position { x: 0, y: 0 }],
        }
    }
}

impl Knot {
    fn update_knot(&mut self, head: Position) {
        let x_check = (head.x - self.position.x).abs() == 2;
        let y_check = (head.y - self.position.y).abs() == 2;
        if self.position.x != head.x && self.position.y != head.y && (x_check || y_check) {
            self.position.x += (head.x - self.position.x).signum();
            self.position.y += (head.y - self.position.y).signum();
            self.history.push(self.position);
            return;
        }
        if self.position.x != head.x && x_check {
            self.position.x += (head.x - self.position.x).signum();
            self.history.push(self.position);
            return;
        }
        if self.position.y != head.y && y_check {
            self.position.y += (head.y - self.position.y).signum();
            self.history.push(self.position);
            return;
        }
    }
}

#[derive(Default, Debug)]
struct Rope {
    head: Knot,
    tail: Vec<Knot>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            head: Knot::default(),
            tail: vec![Knot::default(); length],
        }
    }

    fn process_motion(&mut self, motion: Motion) {
        (0..motion.steps).into_iter().for_each(|_| {
            match motion.direction {
                Direction::Up => {
                    self.head.position.y += 1;
                }
                Direction::Down => {
                    self.head.position.y -= 1;
                }
                Direction::Left => {
                    self.head.position.x -= 1;
                }
                Direction::Right => {
                    self.head.position.x += 1;
                }
            };
            let mut head_position = self.head.position;
            for tail in &mut self.tail {
                tail.update_knot(head_position);
                head_position = tail.position;
            }
            self.head.history.push(self.head.position);
        });
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: i32,
}

impl Motion {
    fn new(motion: (&str, &str)) -> Option<Self> {
        match motion.0 {
            "U" => Some(Self {
                direction: Direction::Up,
                steps: motion.1.parse::<i32>().unwrap(),
            }),
            "D" => Some(Self {
                direction: Direction::Down,
                steps: motion.1.parse::<i32>().unwrap(),
            }),
            "L" => Some(Self {
                direction: Direction::Left,
                steps: motion.1.parse::<i32>().unwrap(),
            }),
            "R" => Some(Self {
                direction: Direction::Right,
                steps: motion.1.parse::<i32>().unwrap(),
            }),
            &_ => None,
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let motions = input
        .lines()
        .map(|line| Motion::new(line.split_once(" ").unwrap()).unwrap())
        .collect::<Vec<Motion>>();
    let mut rope = Rope::new(1);
    println!("{:?}", rope);
    for motion in motions {
        rope.process_motion(motion);
    }
    let tail = rope.tail.last().unwrap();
    let tail_history = tail.history.clone();
    let visited: HashSet<Position> = tail_history.into_iter().collect();
    visited.len()
}

pub fn part_2(input: &str) -> usize {
    let motions = input
        .lines()
        .map(|line| Motion::new(line.split_once(" ").unwrap()).unwrap())
        .collect::<Vec<Motion>>();
    let mut rope = Rope::new(9);
    for motion in motions {
        rope.process_motion(motion);
    }
    let tail = rope.tail.last().unwrap();
    let tail_history = tail.history.clone();
    let visited: HashSet<Position> = tail_history.into_iter().collect();
    visited.len()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./input.txt").unwrap()), 6037)
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(part_2("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2"), 1);
    }

    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"), 36)
    }
}
