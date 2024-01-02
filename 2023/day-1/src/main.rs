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

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const VALUES: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut answer = 0;
        for line in input.lines() {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            answer += 10 * digits.first().unwrap() + digits.last().unwrap();
        }
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut answer = 0;
        for line in input.lines() {
            let mut first: Option<u32> = None;
            let mut last = 0;
            let char_array: Vec<char> = line.chars().collect();
            for i in 0..line.len() {
                for num_index in 0..NUMBERS.len() {
                    let num = NUMBERS[num_index];
                    let value = VALUES[num_index];
                    if char_array[i].is_digit(10) {
                        let x = char_array[i].to_digit(10).unwrap();
                        first = first.or(Some(x));
                        last = x;
                    }
                    if line.len() >= i + num.len() && &line[i..i + num.len()] == num {
                        first = first.or(Some(value));
                        last = value;
                    }
                }
            }
            answer += 10 * first.unwrap() + last;
        }
        Ok(format!("{answer}"))
    }
}
