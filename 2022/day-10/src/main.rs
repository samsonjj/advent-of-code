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
        let mut cycle = 0;
        let mut register = 1;
        let mut add: Option<i32> = None;

        let mut score = 0;

        let mut lines = input.lines();
        loop {
            cycle += 1;

            if (cycle - 20) % 40 == 0 {
                score += cycle * register;
                if cycle == 220 {
                    break;
                }
            }

            if let Some(x) = add {
                register += x;
                add = None;
                continue;
            } else {
                let line = lines.next().unwrap();
                let command = &line[..4];
                match command {
                    "addx" => {
                        let value = &line[5..].parse::<i32>().unwrap();
                        add = Some(*value);
                        continue;
                    }
                    "noop" => {
                        continue;
                    }
                    c => panic!("invalid command {c}"),
                }
            }
        }
        let answer = score;
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut cycle: i32 = 0;
        let mut register: i32 = 1;
        let mut add: Option<i32> = None;

        let mut pixels: Vec<char> = vec![];
        pixels.push('\n');

        let mut lines = input.lines();
        loop {
            cycle += 1;

            let pixel = (cycle - 1) % 40;
            if (register - pixel).abs() <= 1 {
                pixels.push('#');
            } else {
                pixels.push('.');
            }
            if pixel == 39 {
                pixels.push('\n');
            }

            if cycle == 240 {
                break;
            }

            if let Some(x) = add {
                register += x;
                add = None;
                continue;
            } else {
                let line = lines.next().unwrap();
                let command = &line[..4];
                match command {
                    "addx" => {
                        let value = &line[5..].parse::<i32>().unwrap();
                        add = Some(*value);
                        continue;
                    }
                    "noop" => {
                        continue;
                    }
                    c => panic!("invalid command {c}"),
                }
            }
        }
        let answer = pixels.iter().collect::<String>();
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
