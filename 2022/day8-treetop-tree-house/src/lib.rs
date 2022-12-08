use std::cmp;
use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let trees: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let mut height = 0;
    let mut max_height = 0;
    // // look right
    trees.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, height)| {
            if j == 0 || *height > max_height {
                visible.insert((i, j));
                max_height = *height;
            }
        });
    });
    // look left
    trees.iter().enumerate().rev().for_each(|(i, row)| {
        row.iter().enumerate().rev().for_each(|(j, height)| {
            if j == row.len() - 1 || *height > max_height {
                visible.insert((i, j));
                max_height = *height;
            }
        })
    });
    // // look right
    // (0..trees.len()).into_iter().for_each(|i| {
    //     (0..trees[0].len()).into_iter().for_each(|j| {
    //         height = trees[i][j];
    //         if j == 0 || height > max_height {
    //             visible.insert((i, j));
    //             max_height = height;
    //         }
    //     })
    // });
    // // look left
    // (0..trees.len()).into_iter().for_each(|i| {
    //     (0..trees[0].len()).into_iter().rev().for_each(|j| {
    //         height = trees[i][j];
    //         if j == trees[0].len() - 1 || height > max_height {
    //             visible.insert((i, j));
    //             max_height = height;
    //         }
    //     });
    // });
    // look down
    (0..trees[0].len()).into_iter().for_each(|j| {
        (0..trees.len()).into_iter().for_each(|i| {
            height = trees[i][j];
            if i == 0 || height > max_height {
                visible.insert((i, j));
                max_height = height;
            }
        })
    });
    // look up
    (0..trees[0].len()).into_iter().for_each(|j| {
        (0..trees.len()).into_iter().rev().for_each(|i| {
            height = trees[i][j];
            if i == trees.len() - 1 || height > max_height {
                visible.insert((i, j));
                max_height = height;
            }
        });
    });
    visible.len()
}

fn look_left(i: usize, j: usize, trees: &Vec<Vec<usize>>) -> usize {
    if j == 0 {
        return 0;
    }
    let height = trees[i][j];
    let mut distance = 1;
    let mut col = j;
    while col - 1 > 0 && trees[i][col - 1] < height {
        distance += 1;
        col -= 1;
    }
    distance
}

fn look_right(i: usize, j: usize, trees: &Vec<Vec<usize>>) -> usize {
    if j == trees[0].len() - 1 {
        return 0;
    }
    let height = trees[i][j];
    let mut distance = 1;
    let mut col = j;
    while col + 1 < trees[0].len() - 1 && trees[i][col + 1] < height {
        distance += 1;
        col += 1;
    }
    distance
}

fn look_up(i: usize, j: usize, trees: &Vec<Vec<usize>>) -> usize {
    if i == 0 {
        return 0;
    }
    let height = trees[i][j];
    let mut distance = 1;
    let mut row = i;
    while row - 1 > 0 && trees[row - 1][j] < height {
        distance += 1;
        row -= 1;
    }
    distance
}

fn look_down(i: usize, j: usize, trees: &Vec<Vec<usize>>) -> usize {
    if i == trees.len() - 1 {
        return 0;
    }
    let height = trees[i][j];
    let mut distance = 1;
    let mut row = i;
    while row + 1 < trees.len() - 1 && trees[row + 1][j] < height {
        distance += 1;
        row += 1;
    }
    distance
}

pub fn part_2(input: &str) -> usize {
    let mut max_scenic_score = 0;
    let trees: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    (0..trees.len()).into_iter().for_each(|i| {
        (0..trees[0].len()).into_iter().for_each(|j| {
            let left = look_left(i, j, &trees);
            let right = look_right(i, j, &trees);
            let up = look_up(i, j, &trees);
            let down = look_down(i, j, &trees);
            let scenic_score = left * right * up * down;
            max_scenic_score = cmp::max(max_scenic_score, scenic_score);
        })
    });
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        assert_eq!(part_1("30373\n25512\n65332\n33549\n35390"), 21)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2("30373\n25512\n65332\n33549\n35390"), 8)
    }
}
