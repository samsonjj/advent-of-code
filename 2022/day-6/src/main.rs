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

struct Buffer<T> {
    data: std::collections::VecDeque<T>,
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut buffer = std::collections::VecDeque::new();
        for (i, c) in input.chars().enumerate() {
            buffer.push_back(c);
            if buffer.len() > 4 {
                buffer.pop_front();
            }
            if buffer.len() == 4 {
                // test uniqueness
                let set: HashSet<char> = buffer.iter().map(|c| *c).collect();
                if set.len() == 4 {
                    return Ok(format!("{}", i + 1));
                }
            }
        }

        let answer = 42;
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut buffer = std::collections::VecDeque::new();
        for (i, c) in input.chars().enumerate() {
            buffer.push_back(c);
            if buffer.len() > 14 {
                buffer.pop_front();
            }
            if buffer.len() == 14 {
                // test uniqueness
                let set: HashSet<char> = buffer.iter().map(|c| *c).collect();
                if set.len() == 14 {
                    return Ok(format!("{}", i + 1));
                }
            }
        }
        let answer = 5;
        Ok(format!("{}", answer))
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
