#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("../src/input.txt");
static EXAMPLE: &str = include_str!("../src/example.txt");

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

fn usize_repr(s: &str) -> usize {
    match s {
        "n" => 0,
        "ne" => 1,
        "se" => 2,
        "s" => 3,
        "sw" => 4,
        "nw" => 5,
        _ => panic!("Unknown direction: {}", s),
    }
}

fn fall(hm: &mut HashMap<i32, i32>, x: i32, y: i32) {}

impl AocSolver for Temp {
    // 11:46
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut steps: Vec<usize> = vec![0; 6];

        for token in input.split(",") {
            steps[usize_repr(token)] += 1;
        }

        for x in (0..3).into_iter() {
            let opposite = (x + 3) % 6;
            let amount = std::cmp::min(steps[x], steps[opposite]);
            steps[x] -= amount;
            steps[opposite] -= amount;
        }

        for x in (0..3).into_iter() {
            let diag = (x + 2) % 6;
            let amount = std::cmp::min(steps[x], steps[diag]);
            steps[x] -= amount;
            steps[diag] -= amount;
            steps[(x + 1) % 6] += amount;
        }

        Ok(format!("{}", steps.into_iter().sum::<usize>()))
    }

    // 14:22
    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut steps: Vec<usize> = vec![0; 6];

        let max: usize = input
            .split(",")
            .map(|token| {
                let x = usize_repr(token);
                steps[x] += 1;

                let opposite = (x + 3) % 6;
                let amount = std::cmp::min(steps[x], steps[opposite]);
                steps[x] -= amount;
                steps[opposite] -= amount;

                for x in [(x - 2) % 6, x].into_iter() {
                    let diag = (x + 2) % 6;
                    let amount = std::cmp::min(steps[x], steps[diag]);
                    steps[x] -= amount;
                    steps[diag] -= amount;
                    steps[(x + 1) % 6] += amount;
                }

                steps.iter().sum()
            })
            .max()
            .unwrap();

        Ok(format!("{}", max))
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
