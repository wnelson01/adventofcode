use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Default, Debug)]
struct Node {
    name: String,
    size: u32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn percolate_size(&mut self, size: u32) {
        self.size += size;
        if let Some(parent) = &self.parent {
            parent.borrow_mut().percolate_size(size);
        };
    }

    fn part_1(&self) -> u32 {
        let mut total = 0;
        let mut curr = Rc::new(RefCell::new(self.clone()));
        let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
        stack.push(curr);
        while !stack.is_empty() {
            curr = stack.pop().unwrap();
            if curr.borrow().size <= 100000 && !curr.borrow().name.contains(".") && !curr.borrow().children.is_empty() {
                println!("{} {}", curr.borrow().size, curr.borrow().name);
                total += curr.borrow().size;
            }
            curr.borrow().children.iter().for_each(|child| {
                stack.push(Rc::clone(child));
            });
        }
        total
    }

    fn part_2(&self) -> u32 {
        let unused_space = 70000000 - self.size;
        let enough_space: u32 = 30000000;
        let mut deletion_candidates: Vec<u32> = vec![];
        let mut curr = Rc::new(RefCell::new(self.clone()));
        let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
        stack.push(curr);
        while !stack.is_empty() {
            curr = stack.pop().unwrap();
            if curr.borrow().size + unused_space >= enough_space && !curr.borrow().children.is_empty() {
                deletion_candidates.push(curr.borrow().size);
            }
            curr.borrow().children.iter().for_each(|child| {
                stack.push(Rc::clone(child));
            });
        }
        deletion_candidates.sort();
        *deletion_candidates.first().unwrap()
    }

    fn build(input: &str) -> Node {
        let root = Rc::new(RefCell::new(Node {
            name: String::from("/"),
            size: 0,
            ..Default::default()
        }));
        let mut curr = Rc::clone(&root);
        let mut lines: VecDeque<&str> = input.lines().collect();
        while !lines.is_empty() {
            let mut line = lines.pop_front().unwrap();
            let mut tokens = line.split_whitespace();
            tokens.next();
            if let Some(command) = tokens.next() {
                if command == "cd" {
                    if let Some(directory) = tokens.next() {
                        if directory == ".." {
                            curr = Rc::clone(Rc::clone(&curr).borrow().parent.as_ref().unwrap());
                        } else if directory != "/" {
                            curr = Rc::clone(
                                Rc::clone(&curr)
                                    .borrow()
                                    .children
                                    .iter()
                                    .find(|child| Rc::clone(child).borrow().name == *directory)
                                    .as_ref()
                                    .unwrap(),
                            );
                        }
                    }
                }
                if command == "ls" {
                    while !lines.is_empty() {
                        line = lines.pop_front().unwrap();
                        tokens = line.split_whitespace();
                        if let Some(token) = tokens.next() {
                            if token == "dir" {
                                if let Some(directory) = tokens.next() {
                                    curr.borrow_mut()
                                        .children
                                        .push(Rc::clone(&Rc::new(RefCell::new(Node {
                                            name: String::from(directory),
                                            // size: Rc::new(RefCell::new(0)),
                                            size: 0,
                                            parent: Some(Rc::clone(&curr)),
                                            ..Default::default()
                                        }))));
                                }
                            } else if token == "$" {
                                lines.push_front(line);
                                break;
                            } else {
                                let size = token.parse::<u32>().unwrap();
                                curr.borrow_mut()
                                    .children
                                    .push(Rc::clone(&Rc::new(RefCell::new(Node {
                                        name: tokens.next().unwrap().to_string(),
                                        size: size,
                                        parent: Some(Rc::clone(&curr)),
                                        ..Default::default()
                                    }))));
                                curr.borrow_mut().percolate_size(size);
                            }
                        }
                    }
                }
            }
        }
        let x = root.borrow().clone(); x
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut level = 0;
        let mut previous_level;
        let mut curr = Rc::new(RefCell::new(self.clone()));
        let mut stack: Vec<(usize, Rc<RefCell<Node>>)> = vec![];
        stack.push((level, curr));
        while !stack.is_empty() {
            previous_level = level;
            (level, curr) = stack.pop().unwrap();
            Rc::clone(&curr).borrow().children.iter().for_each(|child| {
                stack.push((level + 1, Rc::clone(child)));
            });
            if previous_level == level {
                output += &"| ".repeat(level);
            } else {
                output += &"| ".repeat(level - 1);
                output += "|-";
            }
            output += &format!("{} {}", &curr.borrow().name, &curr.borrow().size);
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

pub fn part_1(input: &str) -> u32 {
    let root = Node::build(input);
    root.part_1()
}

pub fn part_2(input: &str) -> u32 {
    let root = Node::build(input);
    root.part_2()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_part_1() {
        let root = Node::build("$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k"
        );
        assert_eq!(root.part_1(), 95437);
    }

    #[test]
    fn example_part_2() {
        let root = Node::build("$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k"
        );
        assert_eq!(root.part_2(), 24933642);
    }
}
