use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn new(packet: &str) -> Self {
        let pv: Vec<char> = packet.chars().collect();
        let mut i = 0;
        let mut depth = 0;
        let mut res: Vec<Packet> = Vec::new();
        let mut integer_string = String::new();
        while let Some(value) = pv.get(i) {
            match value {
                '[' => {
                    res.push(Packet::new(&packet[i + 1..]));
                    while let Some(value) = pv.get(i) {
                        match value {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        if depth == 0 {
                            break;
                        }
                        i += 1;
                    }
                }
                ']' => {
                    if !integer_string.is_empty() {
                        res.push(Packet::Integer(integer_string.parse::<i32>().unwrap()));
                    }
                    return Packet::List(res);
                }
                ',' => {
                    if !integer_string.is_empty() {
                        res.push(Packet::Integer(integer_string.parse::<i32>().unwrap()));
                        integer_string = String::new();
                    }
                }
                _ => {
                    integer_string.push(*value);
                }
            }
            i += 1;
        }
        Packet::List(res)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Integer(b)) => a.cmp(&vec![Packet::Integer(*b)]),
            (Packet::Integer(a), Packet::List(b)) => vec![Packet::Integer(*a)].cmp(b),
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::List(a), Self::Integer(b)) => a == &vec![Packet::Integer(*b)],
            (Self::Integer(a), Self::List(b)) => &vec![Packet::Integer(*a)] == b,
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut res = 0;
    input.split("\n\n").enumerate().for_each(|(i, pair)| {
        let (left, right) = pair.split_once('\n').unwrap();
        let left = Packet::new(left);
        let right = Packet::new(right);
        if left < right {
            res += i + 1;
        }
    });
    res
}

pub fn part_2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(Packet::new)
        .collect();
    let mut decoder_key1 = 0;
    let mut decoder_key2 = 0;
    packets.push(Packet::new("[[2]]"));
    packets.push(Packet::new("[[6]]"));
    packets.sort();
    for (i, packet) in packets.iter().enumerate() {
        if *packet == Packet::new("[[2]]") {
            decoder_key1 = i + 1;
        }
        if *packet == Packet::new("[[6]]") {
            decoder_key2 = i + 1;
        }
    }
    decoder_key1 * decoder_key2
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
        assert_eq!(part_1(&fs::read_to_string("./example.txt").unwrap()), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&fs::read_to_string("./input.txt").unwrap()), 6369);
    }

    #[test]
    fn part_2_exampe() {
        assert_eq!(part_2(&fs::read_to_string("./example.txt").unwrap()), 140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&fs::read_to_string("./input.txt").unwrap()), 25800)
    }
}
