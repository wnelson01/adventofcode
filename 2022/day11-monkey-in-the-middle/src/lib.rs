use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize, usize) -> usize,
    operand: String,
    divisor: usize,
    true_clause: usize,
    false_clause: usize,
    inspections: usize,
}

pub fn parse_items(items: &str) -> Vec<usize> {
    items
        .trim()
        .split(", ")
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn parse_operation(operation: &str) -> (fn(usize, usize) -> usize, String) {
    let mut operation = operation.split_whitespace();
    operation.next();
    operation.next();
    operation.next();
    let operator = match operation.next().unwrap() {
        "+" => |x, y| x + y,
        "-" => |x, y| x - y,
        "/" => |x, y| x / y,
        "*" => |x, y| x * y,
        &_ => |x, y| x + y,
    };
    let operand = operation.next().unwrap();
    (operator, operand.to_string())
}

pub fn parse_test(test: &str) -> (usize, usize, usize) {
    let (_, divisor) = test.split_once("divisible by ").unwrap();
    let (divisor, true_clause) = test.split_once("If true: ").unwrap();
    let (true_clause, false_clause) = true_clause.split_once("If false: ").unwrap();
    let (_, divisor) = divisor.split_once("by ").unwrap();
    let (_, true_clause) = true_clause.split_once("monkey ").unwrap();
    let (_, false_clause) = false_clause.split_once("monkey ").unwrap();
    let divisor = divisor.trim();
    let true_clause = true_clause.trim();
    let false_clause = false_clause.trim();
    (
        divisor.parse::<usize>().unwrap(),
        true_clause.parse::<usize>().unwrap(),
        false_clause.parse::<usize>().unwrap(),
    )
}

pub fn part_1(input: &str) -> usize {
    let monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let (_, items) = monkey.split_once("items: ").unwrap();
            let (items, operation) = items.split_once("Operation: ").unwrap();
            let (operation, test) = operation.split_once("Test: ").unwrap();
            let operation_parse = parse_operation(operation);
            let test_parse = parse_test(test);
            Rc::new(RefCell::new(Monkey {
                items: parse_items(items),
                operation: operation_parse.0,
                operand: operation_parse.1,
                divisor: test_parse.0,
                true_clause: test_parse.1,
                false_clause: test_parse.2,
                inspections: 0,
            }))
        })
        .collect::<Vec<Rc<RefCell<Monkey>>>>();
    (0..20).for_each(|_| {
        monkeys.iter().for_each(|monkey| {
            let true_monkey = Rc::clone(&monkeys[monkey.borrow().true_clause]);
            let false_monkey = Rc::clone(&monkeys[monkey.borrow().false_clause]);
            let operation = monkey.borrow().operation;
            let operand = monkey.borrow().operand.clone();
            let divisor = monkey.borrow().divisor;
            let inspections = monkey.borrow().items.len();
            monkey.borrow_mut().inspections += inspections;
            monkey.borrow_mut().items.iter_mut().for_each(|item| {
                if operand == "old" {
                    *item = operation(*item, *item) / 3;
                } else {
                    *item = operation(*item, operand.parse::<usize>().unwrap()) / 3;
                }
                if *item % divisor == 0 {
                    true_monkey.borrow_mut().items.push(*item);
                } else {
                    false_monkey.borrow_mut().items.push(*item);
                }
            });
            monkey.borrow_mut().items.drain(0..);
        });
    });
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspections)
        .collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    println!("{:?}", inspections);
    inspections[0] * inspections[1]
}

pub fn part_2(input: &str) -> usize {
    let monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let (_, items) = monkey.split_once("items: ").unwrap();
            let (items, operation) = items.split_once("Operation: ").unwrap();
            let (operation, test) = operation.split_once("Test: ").unwrap();
            let operation_parse = parse_operation(operation);
            let test_parse = parse_test(test);
            Rc::new(RefCell::new(Monkey {
                items: parse_items(items),
                operation: operation_parse.0,
                operand: operation_parse.1,
                divisor: test_parse.0,
                true_clause: test_parse.1,
                false_clause: test_parse.2,
                inspections: 0,
            }))
        })
        .collect::<Vec<Rc<RefCell<Monkey>>>>();
    let bigmod = monkeys
        .iter()
        .map(|monkey| monkey.borrow().divisor)
        .fold(1, |acc, divisor| acc * divisor);
    println!("{}", bigmod);
    (0..10000).for_each(|_| {
        monkeys.iter().for_each(|monkey| {
            let true_monkey = Rc::clone(&monkeys[monkey.borrow().true_clause]);
            let false_monkey = Rc::clone(&monkeys[monkey.borrow().false_clause]);
            let operation = monkey.borrow().operation;
            let operand = monkey.borrow().operand.clone();
            let divisor = monkey.borrow().divisor;
            let inspections = monkey.borrow().items.len();
            monkey.borrow_mut().inspections += inspections;
            monkey.borrow_mut().items.iter_mut().for_each(|item| {
                if operand == "old" {
                    *item = operation(*item, *item) % bigmod;
                } else {
                    *item = operation(*item, operand.parse::<usize>().unwrap()) % bigmod;
                }
                if *item % divisor == 0 {
                    true_monkey.borrow_mut().items.push(*item);
                } else {
                    false_monkey.borrow_mut().items.push(*item);
                }
            });
            monkey.borrow_mut().items.drain(0..);
        });
    });
    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspections)
        .collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    println!("{:?}", inspections);
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./example.txt").unwrap()), 10605)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(
            part_2(&fs::read_to_string("./example.txt").unwrap()),
            2713310158
        )
    }
}
