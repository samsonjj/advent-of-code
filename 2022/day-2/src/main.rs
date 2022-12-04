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
        let rounds = input.lines().map(|line| (&line[0..1], &line[2..]));
        let score = rounds
            .map(|round| {
                let (left, right) = round;
                let mut score = match round.1 {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => unreachable!(),
                };
                score += match round {
                    ("A", "X") => 3,
                    ("A", "Y") => 6,
                    ("A", "Z") => 0,
                    ("B", "X") => 0,
                    ("B", "Y") => 3,
                    ("B", "Z") => 6,
                    ("C", "X") => 6,
                    ("C", "Y") => 0,
                    ("C", "Z") => 3,
                    _ => 0,
                };
                score
            })
            .sum::<i32>();
        Ok(format!("{}", score))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let rounds = input.lines().map(|line| (&line[0..1], &line[2..]));
        let score = rounds
            .map(|mut round| {
                round.1 = match round {
                    ("A", "X") => "Z",
                    ("A", "Y") => "X",
                    ("A", "Z") => "Y",
                    ("B", "X") => "X",
                    ("B", "Y") => "Y",
                    ("B", "Z") => "Z",
                    ("C", "X") => "Y",
                    ("C", "Y") => "Z",
                    ("C", "Z") => "X",
                    _ => unreachable!(),
                };

                let mut score = match round.1 {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => unreachable!(),
                };
                score += match round {
                    ("A", "X") => 3,
                    ("A", "Y") => 6,
                    ("A", "Z") => 0,
                    ("B", "X") => 0,
                    ("B", "Y") => 3,
                    ("B", "Z") => 6,
                    ("C", "X") => 6,
                    ("C", "Y") => 0,
                    ("C", "Z") => 3,
                    _ => 0,
                };
                score
            })
            .sum::<i32>();
        Ok(format!("{}", score))
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
