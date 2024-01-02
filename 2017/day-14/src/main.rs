#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

const LIST_SIZE: i32 = 256;

struct KnotHash {
    skip_size: i32,
    current_position: i32,
    values: Vec<i32>,
}

impl KnotHash {
    fn new() -> Self {
        Self {
            skip_size: 0,
            current_position: 0,
            values: (0..LIST_SIZE).collect(),
        }
    }

    fn reverse(&mut self, length: i32) {
        for i in 0..length / 2 {
            let index = (self.current_position + i) % LIST_SIZE;
            let other_index = (self.current_position + length - i - 1) % LIST_SIZE;
            self.values.swap(index as usize, other_index as usize);
        }
        self.current_position += length + self.skip_size;
        self.skip_size += 1;
    }

    fn knot_hash(input: &str) -> String {
        let mut ring = Self::new();
        let mut lengths = input.trim().chars().map(|c| c as i32).collect::<Vec<i32>>();
        lengths.append(&mut vec![17, 31, 73, 47, 23]);

        for round in 0..64 {
            for length in lengths.iter() {
                ring.reverse(*length);
            }
        }

        let hexes = ring
            .values
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
            .map(|result| format!("{result:08b}"))
            .collect::<Vec<_>>();

        hexes.join("")
    }
}

fn main() {
    let temp = Temp {};
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--example".to_string()) {
        temp.execute(EXAMPLE);
    } else {
        temp.execute(INPUT);
    }
}

struct Temp {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Free,
    Used,
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let grid: usize = (0..128)
            .map(|row| {
                let key = format!("{}-{}", input, row);
                let hash = KnotHash::knot_hash(key.as_str());
                return hash.chars().filter(|c| c == &'1').count();
            })
            .sum();
        Ok(format!("{:?}", grid))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let grid: Vec<Vec<Square>> = (0..128)
            .map(|row| {
                let key = format!("{}-{}", input, row);
                let hash = KnotHash::knot_hash(key.as_str());
                return hash
                    .chars()
                    .map(|c| match c {
                        '0' => Square::Free,
                        '1' => Square::Used,
                        _ => unreachable!(),
                    })
                    .collect();
            })
            .collect();

        let mut visited = vec![vec![false; 128]; 128];
        let mut region_count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == Square::Free {
                    continue;
                }
                if !visited[i][j] {
                    region_count += 1;
                    dfs(&grid, (i, j), &mut visited);
                }
            }
        }

        Ok(format!("{}", region_count))
    }
}

fn dfs(grid: &Vec<Vec<Square>>, pos: (usize, usize), visited: &mut Vec<Vec<bool>>) {
    if pos.0 >= 128 || pos.1 >= 128 {
        // handles wrapping from subtracting past zero
        return;
    }
    if grid[pos.0][pos.1] == Square::Free {
        return;
    }

    if visited[pos.0][pos.1] {
        return;
    }
    visited[pos.0][pos.1] = true;

    dfs(grid, (pos.0.wrapping_sub(1), pos.1), visited);
    dfs(grid, (pos.0, pos.1.wrapping_sub(1)), visited);
    dfs(grid, (pos.0 + 1, pos.1), visited);
    dfs(grid, (pos.0, pos.1 + 1), visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
