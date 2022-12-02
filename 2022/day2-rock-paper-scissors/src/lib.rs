pub fn process_part1(strategy_guide: &str) -> i32 {
    strategy_guide.lines().fold(0, |acc, round| {
        let shapes = round.split_once(" ").unwrap();
        let outcome = match shapes.0 {
            "A" => match shapes.1 {
                "X" => 1 + 3,
                "Y" => 2 + 6,
                "Z" => 3 + 0,
                _ => 0
            },
            "B" => match shapes.1 {
                "X" => 1 + 0,
                "Y" => 2 + 3,
                "Z" => 3 + 6,
                _ => 0
            },
            "C" => match shapes.1 {
                "X" => 1 + 6,
                "Y" => 2 + 0,
                "Z" => 3 + 3,
                _ => 0
            },
            _ => 0,
        };
        acc + outcome
    })
}

pub fn process_part2(strategy_guide: &str) -> i32 {
    strategy_guide.lines().fold(0, |acc, round| {
        let shapes = round.split_once(" ").unwrap();
        let outcome = match shapes.0 {
            "A" => match shapes.1 {
                "X" => 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => 0
            },
            "B" => match shapes.1 {
                "X" => 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => 0
            },
            "C" => match shapes.1 {
                "X" => 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => 0
            },
            _ => 0,
        };
        acc + outcome
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
