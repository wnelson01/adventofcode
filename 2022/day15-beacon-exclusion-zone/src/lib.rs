use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let mut non_beacons = Vec::new();
    let mut beacons = HashSet::new();
    let re = Regex::new(r"(-*[0-9]+)").unwrap();
    let row = 2000000;
    input.lines().for_each(|line| {
        let coords = re
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let sx = coords[0];
        let sy = coords[1];
        let bx = coords[2];
        let by = coords[3];
        let md = (sx - bx).abs() + (sy - by).abs();
        if (sy - md..=sy + md).contains(&row) {
            if by == row {
                beacons.insert((bx, by));
            }
            let mdx = md - (row - sy).abs();
            non_beacons.push((sx - mdx, sx + mdx));
        }
        if non_beacons.len() > 1 {
            non_beacons = merge_intervals(&mut non_beacons);
        }
    });
    println!("non_beacons on y = 2000000: {:?}", non_beacons);
    println!("beacons on y = 2000000: {:?}", beacons);
    (non_beacons[0].0 - non_beacons[0].1).abs()
}

pub fn merge_intervals(intervals: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut res = Vec::new();
    let mut start = intervals[0].0;
    let mut stop = intervals[0].1;
    intervals.iter().skip(1).for_each(|interval| {
        if interval.0 - 1 >= stop + 1 {
            res.push((start, stop));
            start = interval.0;
            stop = interval.1;
        }
        stop = i32::max(stop, interval.1);
    });
    res.push((start, stop));
    res
}

pub fn part_2(input: &str) -> i32 {
    let re = Regex::new(r"(-*[0-9]+)").unwrap();
    let mut rows: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    input.lines().for_each(|line| {
        let coords = re
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let sx = coords[0];
        let sy = coords[1];
        let bx = coords[2];
        let by = coords[3];
        let md = (sx - bx).abs() + (sy - by).abs();
        (sy - md..=sy + md).for_each(|y| {
            if 0 <= y && y <= 4000000 {
                let x = (md - (sy - y).abs()).abs();
                if 0 <= x && x <= 4000000 {
                    let mdxy = (sx - x, sx + x);
                    if rows.contains_key(&y) {
                        let row = rows.get_mut(&y).unwrap();
                        row.push(mdxy);
                        *row = merge_intervals(row);
                    } else {
                        rows.insert(y, vec![mdxy]);
                    }
                }
            }
        });
    });
    for key in rows.keys() {
        let xs = rows.get(key).unwrap();
        if xs.len() > 1 {
            println!("{:?} {:?}", key, xs);
        }
    }
    0
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
