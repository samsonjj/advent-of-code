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

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut offsets = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut pos = 0i32;
        let mut steps = 0;
        loop {
            steps += 1;
            let jump = offsets[pos as usize];
            offsets[pos as usize] += 1;
            pos += jump;
            if pos < 0 || pos >= offsets.len() as i32 {
                break;
            }
        }
        Ok(format!("{}", steps))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut offsets = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut pos = 0i32;
        let mut steps = 0;
        loop {
            steps += 1;
            let jump = offsets[pos as usize];
            if offsets[pos as usize] >= 3 {
                offsets[pos as usize] -= 1;
            } else {
                offsets[pos as usize] += 1;
            }
            pos += jump;
            if pos < 0 || pos >= offsets.len() as i32 {
                break;
            }
        }
        Ok(format!("{}", steps))
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
