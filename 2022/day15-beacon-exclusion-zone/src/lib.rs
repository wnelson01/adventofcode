use regex::Regex;
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
        non_beacons.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?}", non_beacons);
        if non_beacons.len() > 1 {
            let mut merged_non_beacons = Vec::new();
            let mut start = non_beacons[0].0;
            let mut stop = non_beacons[0].1;
            non_beacons.iter().skip(1).for_each(|beacon| {
                if beacon.0 > stop {
                    merged_non_beacons.push((start, stop));
                    start = beacon.0;
                    stop = beacon.1;
                }
                stop = i32::max(stop, beacon.1);
            });
            merged_non_beacons.push((start, stop));
            non_beacons = merged_non_beacons;
            println!("merged: {:?}", non_beacons);
        }
    });
    println!("{:?}", non_beacons);
    println!("{:?}", beacons);
    (non_beacons[0].0 - non_beacons[0].1).abs()
}

pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let mut start = intervals[0].0;
    let mut stop = intervals[0].1;
    intervals.iter().skip(1).for_each(|interval| {
        if interval.0 > stop {
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
    let mut xs = Vec::new();
    let mut ys = Vec::new();
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
        xs.push((sx - md, sx + md));
        ys.push((i32::max(0, sy - md), i32::min(sy + md, 4000000)));
    });
    xs = merge_intervals(xs);
    ys = merge_intervals(ys);
    println!("{:?}", xs);
    println!("{:?}", ys);
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
