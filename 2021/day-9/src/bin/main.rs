#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("../input.txt");
static EXAMPLE: &str = include_str!("../example.txt");

const diffs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i64> {
    let heights: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    const diffs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let height = heights.len() as i32;
    let width = heights[0].len() as i32;

    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if diffs.iter().all(|diff| {
                let ii = i as i32 + diff.0;
                let jj = j as i32 + diff.1;
                if ii < 0 || ii >= height {
                    true
                } else if jj < 0 || jj >= width {
                    true
                } else {
                    heights[ii as usize][jj as usize] > heights[i as usize][j as usize]
                }
            }) {
                sum += heights[i as usize][j as usize] + 1;
            }
        }
    }
    Ok(sum as i64)
}

fn basin_size(lp: (i32, i32), heights: &Vec<Vec<i32>>, hs: &mut HashSet<(i32, i32)>) -> i32 {
    hs.insert((lp.0, lp.1));
    for diff in diffs {
        let ii = lp.0 as i32 + diff.0;
        let jj = lp.1 as i32 + diff.1;
        // println!("{:?}, {:?}", &ii, &jj);
        if !hs.contains(&(ii, jj))
            && ii >= 0
            && ii < heights.len() as i32
            && jj >= 0
            && jj < heights[0].len() as i32
            && heights[ii as usize][jj as usize] > heights[lp.0 as usize][lp.1 as usize]
            && heights[ii as usize][jj as usize] != 9
        {
            hs.insert((ii, jj));
            basin_size((ii, jj), heights, hs);
        }
    }
    hs.len() as i32
}

fn part_2(input: &str) -> AocResult<i64> {
    let heights: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect();

    let height = heights.len() as i32;
    let width = heights[0].len() as i32;

    let mut hs = HashSet::new();
    let mut basins = vec![];
    for i in 0..height {
        for j in 0..width {
            if diffs.iter().all(|diff| {
                let ii = i as i32 + diff.0;
                let jj = j as i32 + diff.1;
                if ii < 0 || ii >= height {
                    true
                } else if jj < 0 || jj >= width {
                    true
                } else {
                    heights[ii as usize][jj as usize] > heights[i as usize][j as usize]
                }
            }) {
                basins.push(basin_size((i, j), &heights, &mut hs));
                hs.clear();
            }
        }
    }
    basins.sort();
    // println!("basins={:?}", basins);
    let l = basins.len();
    // println!("1={:?}", basins[l - 1]);
    // println!("2={:?}", basins[l - 3]);
    // println!("3={:?}", basins[l - 2]);
    Ok(basins[l - 1] as i64 * basins[l - 2] as i64 * basins[l - 3] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
