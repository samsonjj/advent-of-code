#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut checksum = 0;
        for line in input.lines() {
            let max = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .max()
                .unwrap();
            let min = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .min()
                .unwrap();
            dbg!(&max, &min);
            checksum += max - min;
        }
        Ok(format!("{}", checksum))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 5;
        let checksum: i32 = input
            .lines()
            .map(|line| {
                let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                for (i, x) in nums.clone().enumerate() {
                    for (j, y) in nums.clone().enumerate() {
                        if x % y == 0 && i != j {
                            return x / y;
                        }
                    }
                }
                return 0;
            })
            .sum();

        Ok(format!("{}", checksum))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
