use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Default, Debug)]
struct Node {
    name: String,
    size: Rc<RefCell<u32>>,
    // size: u32,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

fn update_size(node: Node) -> u32 {
    if node.children.is_empty() {
        // println!("{}", *node.size.borrow());
        return *node.size.borrow();
    } else {
        let size = node
            .children
            .iter()
            .fold(0, |acc, child| acc + update_size(child.borrow().clone()));
        size
    }
}

impl Node {
    // fn update_size(&self) {
    //     let curr = Rc::new(RefCell::new(self.clone()));
    //     let size = *Rc::clone(&curr).borrow().size.borrow_mut();
    //     if Rc::clone(&curr).borrow().children.is_empty() {
    //         let mut parent = Rc::clone(Rc::clone(&curr).borrow().parent.as_ref().unwrap());
    //         let size = Rc::clone(&Rc::clone(&parent).borrow().size);
    //         *size.borrow_mut() += *(Rc::clone(&curr).borrow().size).borrow();
    //     }
    //     Rc::clone(&curr)
    //         .borrow()
    //         .children
    //         .iter()
    //         .for_each(|child| child.borrow().update_size())
    // }
    // fn update_size(&self) {
    //     let mut curr = Rc::new(RefCell::new(self.clone()));
    //     let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
    //     stack.push(curr);
    //     while !stack.is_empty() {
    //         curr = stack.pop().unwrap();
    //         let mut parent = Rc::clone(Rc::clone(&curr).borrow().parent.as_ref().unwrap());
    //         let size = Rc::clone(&Rc::clone(&parent).borrow().size);
    //         *size.borrow_mut() += *(Rc::clone(&curr).borrow().size).borrow();
    //     }
    // }
    fn get_size(&self) -> u32 {
        let mut curr = Rc::new(RefCell::new(self.clone()));
        if Rc::clone(&curr).borrow().children.is_empty() {
            *(curr.borrow()).size.borrow()
        } else {
            Rc::clone(&curr)
                .borrow()
                .children
                .iter()
                .fold(0, |acc, child| acc + child.borrow().clone().get_size())
        }
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
            output += &format!("{} {}", &curr.borrow().name, &curr.borrow().size.borrow());
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

pub fn part_1(input: &str) -> &str {
    let root = Rc::new(RefCell::new(Node {
        name: String::from("/"),
        size: Rc::new(RefCell::new(0)),
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
                                        size: Rc::new(RefCell::new(0)),
                                        parent: Some(Rc::clone(&curr)),
                                        ..Default::default()
                                    }))));
                            }
                        } else if token == "$" {
                            lines.push_front(line);
                            break;
                        } else {
                            curr.borrow_mut()
                                .children
                                .push(Rc::clone(&Rc::new(RefCell::new(Node {
                                    name: tokens.next().unwrap().to_string(),
                                    size: Rc::new(RefCell::new(token.parse::<u32>().unwrap())),
                                    // size: token.parse::<u32>().unwrap(),
                                    parent: Some(Rc::clone(&curr)),
                                    ..Default::default()
                                }))));
                        }
                    }
                }
            }
        }
    }
    *root.borrow().size.borrow_mut() += update_size(root.borrow().clone());
    println!("{}", root.borrow());
    // println!("{}", root.borrow().size.borrow());
    "part 1"
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
}
