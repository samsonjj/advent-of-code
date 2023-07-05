#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

use std::str::Split;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

fn get_next(parts: &mut Split<char>) -> (i32, i32) {
    let mut part_1 = parts
        .next()
        .unwrap()
        .split('-')
        .map(|x| x.parse::<i32>().unwrap());
    (part_1.next().unwrap(), part_1.next().unwrap())
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut count = 0;
        for line in input.lines() {
            let mut parts = line.split(',');
            let part_1 = get_next(&mut parts);
            let part_2 = get_next(&mut parts);
            if (part_1.0 <= part_2.0 && part_1.1 >= part_2.1)
                || (part_2.0 <= part_1.0 && part_2.1 >= part_1.1)
            {
                count += 1;
            }
        }
        Ok(format!("{}", count))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut count = 0;
        for line in input.lines() {
            let mut parts = line.split(',');
            let part_1 = get_next(&mut parts);
            let part_2 = get_next(&mut parts);
            if (part_1.0 <= part_2.1 && part_2.0 <= part_1.1) {
                count += 1;
            }
        }
        Ok(format!("{}", count))
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
