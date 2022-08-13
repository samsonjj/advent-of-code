#![feature(box_syntax)]
#![allow(
    dead_code,
    unused_imports,
    unused_variables
)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp: Box<dyn AocSolver> = Box::new(Temp{});
    temp.execute(INPUT);
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("{}", 3))
    }

    fn part_2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("{}", 5))
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
