#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

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

use regex::Regex;

fn parse_line(line: &str) -> AocResult<i64> {
    let regex_pattern = Regex::new(r"Generator \w starts with (\d+)")?;

    Ok(regex_pattern
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap())
}

fn gen_next(x: i64, factor: i64) -> i64 {
    x * factor % 2147483647
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut lines = input.lines();
        let mut a = parse_line(lines.next().unwrap())?;
        let mut b = parse_line(lines.next().unwrap())?;

        let a_factor = 16807;
        let b_factor = 48271;
        let mask = 0xffff;

        let mut judge_count = 0;

        for i in 0..40_000_000 {
            a = gen_next(a, a_factor);
            b = gen_next(b, b_factor);
            if a & mask == b & mask {
                judge_count += 1;
            }
        }

        Ok(format!("{}", judge_count))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut lines = input.lines();
        let mut a = parse_line(lines.next().unwrap())?;
        let mut b = parse_line(lines.next().unwrap())?;

        let a_factor = 16807;
        let b_factor = 48271;
        let mask = 0xffff;

        let mut judge_count = 0;

        for i in 0..5_000_000 {
            while {
                a = gen_next(a, a_factor);
                a % 4 != 0
            } {}
            while {
                b = gen_next(b, b_factor);
                b % 8 != 0
            } {}
            if a & mask == b & mask {
                judge_count += 1;
            }
        }

        Ok(format!("{}", judge_count))
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
