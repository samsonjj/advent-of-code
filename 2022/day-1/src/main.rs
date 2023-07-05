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
        let result = input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .max()
            .unwrap();

        Ok(format!("{}", result))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        use std::collections::BinaryHeap;

        let mut heap: BinaryHeap<i32> = input
            .split("\n\n")
            .map(|elf| {
                elf.lines()
                    .map(|line| line.parse::<i32>().unwrap())
                    .sum::<i32>()
            })
            .collect();

        return Ok((0..3).map(|_| heap.pop().unwrap()).sum::<i32>().to_string());
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

trait AsBool {
    fn as_bool(&self) -> bool;
}

impl dyn AsBool {
    fn is_false(&self) -> bool {
        !self.as_bool()
    }
}

struct MyTrueBool {}

impl AsBool for MyTrueBool {
    fn as_bool(&self) -> bool {
        true
    }
}
