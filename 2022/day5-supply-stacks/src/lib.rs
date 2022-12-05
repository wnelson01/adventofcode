#[derive(Debug)]
struct Stacks {
    crates: Vec<Vec<char>>,
}

impl Stacks {
    fn new(starting: &str) -> Self {
        let mut stacks_of_crates: Vec<Vec<char>> = Vec::new();
        starting.lines().rev().enumerate().for_each(|(idx, level)| {
            if idx == 0 {
                level
                    .split_whitespace()
                    .for_each(|_| stacks_of_crates.push(Vec::new()))
            } else {
                level.chars().enumerate().for_each(|(idx, c)| {
                    if c.is_alphabetic() {
                        stacks_of_crates[idx / 4].push(c);
                    }
                });
            }
        });
        Self {
            crates: stacks_of_crates,
        }
    }

    fn rearrange(&mut self, procedure: &str) {
        let procedure = procedure.split_whitespace().collect::<Vec<&str>>();
        let quantity = procedure[1].parse::<usize>().unwrap();
        let from = procedure[3].parse::<usize>().unwrap() - 1;
        let to = procedure[5].parse::<usize>().unwrap() - 1;
        (0..quantity).for_each(|_| {
            let popped = self.crates[from].pop();
            if let Some(value) = popped {
                self.crates[to].push(value);
            }
        });
    }

    fn over_nine_thousand(&mut self, procedure: &str) {
        let procedure = procedure.split_whitespace().collect::<Vec<&str>>();
        let quantity = procedure[1].parse::<usize>().unwrap();
        let from = procedure[3].parse::<usize>().unwrap() - 1;
        let to = procedure[5].parse::<usize>().unwrap() - 1;
        let mut staging: Vec<char> = Vec::new();
        (0..quantity).for_each(|_| {
            let popped = self.crates[from].pop();
            if let Some(value) = popped {
                staging.push(value);
            }
        });
        staging.iter().rev().for_each(|c| {
            self.crates[to].push(*c);
        })
    }

    fn top_of_each_stack(&mut self) -> String {
        self.crates
            .iter()
            .fold("".to_string(), |mut acc: String, stack| {
                if let Some(last) = stack.last() {
                    acc += &last.to_string();
                }
                acc
            })
    }
}

pub fn process_part1(input: &str) -> String {
    let (starting, rearrangement_procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::new(starting);
    rearrangement_procedure.lines().for_each(|procedure| {
        stacks.rearrange(procedure);
    });
    stacks.top_of_each_stack()
}

pub fn process_part2(input: &str) -> String {
    let (starting, rearrangement_procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::new(starting);
    rearrangement_procedure.lines().for_each(|procedure| {
        stacks.over_nine_thousand(procedure);
    });
    stacks.top_of_each_stack()
}
