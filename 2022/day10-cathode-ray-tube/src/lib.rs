use std::io::{self, Write};

pub fn get_cycles(input: &str) -> Vec<i32> {
    let mut cycles = vec![1];
    input.lines().for_each(|line| {
        let line = line.split_once(" ");
        match line {
            None => {
                cycles.push(*cycles.last().unwrap());
            }
            Some((_, value)) => {
                cycles.push(*cycles.last().unwrap());
                cycles.push(*cycles.last().unwrap() + value.parse::<i32>().unwrap());
            }
        }
    });
    cycles
}

pub fn part_1(input: &str) -> i32 {
    let cycles = get_cycles(input);
    (20 * cycles[19])
        + (60 * cycles[59])
        + (100 * cycles[99])
        + (140 * cycles[139])
        + (180 * cycles[179])
        + (220 * cycles[219])
}

pub fn part_2(input: &str) {
    let cycles = get_cycles(input);
    (0..240).into_iter().for_each(|cycle| {
        let x_register = cycles[cycle];
        let sprite_position = x_register - 1..=x_register + 1;
        let pixel = (cycle % 40) as i32;
        // println!("{:?}", sprite_position);
        // println!("{}", pixel);
        if (sprite_position).contains(&pixel) {
            print!("#");
        } else {
            print!(".");
        }
        if pixel == 39 {
            print!("\n");
            io::stdout().flush().unwrap();
        }
    })
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
        assert_eq!(
            part_1(&fs::read_to_string("./part_1_example.txt").unwrap()),
            13140
        )
    }

    #[test]
    fn part_2_example() {
        assert_eq!(
            part_2(&fs::read_to_string("./part_1_example.txt").unwrap()),
            ()
        )
    }
}
