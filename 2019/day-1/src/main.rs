#![feature(box_syntax)]
#![allow(unused_imports, unused_variables, dead_code)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    println!("Hello, world!");
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let num = line.parse::<i32>().unwrap();
        sum += (num / 3) - 2;
    }
    Ok(sum)
}

fn fuel(x: i32) -> i32 {
    let num = x / 3 - 2;
    if num <= 0 {
        return 0;
    }
    num + fuel(num)
}

fn part_2(input: &str) -> AocResult<i32> {
    let sum = input
        .lines()
        .map(|line| fuel(line.parse::<i32>().unwrap()))
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
