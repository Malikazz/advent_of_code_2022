
use itertools::Itertools;
use std::fs::read_to_string;

fn parse_p8() -> Vec<Vec<u32>> {
    let input = read_to_string("src/assets/problem_08").unwrap();

    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn directions_p8(grid: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    let row = grid[y].clone();

    // Collect all the items from each row at the x offset
    let column = grid.iter().map(|row| row[x]).collect::<Vec<u32>>();

    // split around our current point
    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    // reverse splits
    let up = up.iter().copied().rev().collect();
    let left = left.iter().copied().rev().collect();
    // normal splits
    let right = right[1..].to_vec();
    let down = down[1..].to_vec();

    [up, down, left, right]
}

pub fn problem_08() -> usize {
    // I kept having slighty off problems in this problem
    // so I went and found a solve to learn from.
    // credit here https://nickymeuleman.netlify.app/garden/aoc2022-day08
    let trees = parse_p8();
    let len = trees.len();

    // dont include the edges
    let answer = (1..len - 1)
        // get pairs to compare
        .cartesian_product(1..len - 1)
        .map(|(y, x)| {
            let height = trees[y][x];
            // split everything
            directions_p8(&trees, x, y)
                .iter()
                .map(|direction| {
                    direction
                        .iter()
                        .position(|h| *h >= height)
                        .map(|p| p + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .product::<usize>()
        })
        .max()
        .unwrap();
    print!("Problem 08: {:?}\n\n", answer);
    answer
}
