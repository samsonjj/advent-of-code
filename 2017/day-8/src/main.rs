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
    // 9:38
    fn part_1(&self, input: &str) -> AocResult<String> {
        let answer = 3;
        let mut data = HashMap::<&str, i32>::new();
        let mut max = 0;
        for line in input.lines() {
            let mut tokens = line.split_whitespace();

            let target = tokens.next().unwrap();
            let inc_dec = tokens.next().unwrap();
            let amount = tokens.next().unwrap();
            let _if = tokens.next().unwrap();
            let target_cond = tokens.next().unwrap();
            let operand = tokens.next().unwrap();
            let amount_cond = tokens.next().unwrap();

            dbg!(operand);
            let f: fn(i32, i32) -> bool = match operand {
                ">" => |x, y| x > y,
                "<" => |x, y| x < y,
                ">=" => |x, y| x >= y,
                "<=" => |x, y| x <= y,
                "==" => |x, y| x == y,
                "!=" => |x, y| x != y,
                _ => unreachable!(),
            };

            if f(
                *data.entry(target_cond).or_insert(0),
                amount_cond.parse::<i32>().unwrap(),
            ) {
                let entry = data.entry(target).or_insert(0);
                match inc_dec {
                    "inc" => *entry += amount.parse::<i32>().unwrap(),
                    "dec" => *entry -= amount.parse::<i32>().unwrap(),
                    _ => unreachable!(),
                }
            }
        }

        let answer = data.values().max().unwrap();
        Ok(format!("{}", answer))
    }

    // 12:23
    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 3;
        let mut data = HashMap::<&str, i32>::new();
        let mut max = 0;
        for line in input.lines() {
            let mut tokens = line.split_whitespace();

            let target = tokens.next().unwrap();
            let inc_dec = tokens.next().unwrap();
            let amount = tokens.next().unwrap();
            let _if = tokens.next().unwrap();
            let target_cond = tokens.next().unwrap();
            let operand = tokens.next().unwrap();
            let amount_cond = tokens.next().unwrap();

            dbg!(operand);
            let f: fn(i32, i32) -> bool = match operand {
                ">" => |x, y| x > y,
                "<" => |x, y| x < y,
                ">=" => |x, y| x >= y,
                "<=" => |x, y| x <= y,
                "==" => |x, y| x == y,
                "!=" => |x, y| x != y,
                _ => unreachable!(),
            };

            if f(
                *data.entry(target_cond).or_insert(0),
                amount_cond.parse::<i32>().unwrap(),
            ) {
                let entry = data.entry(target).or_insert(0);
                match inc_dec {
                    "inc" => *entry += amount.parse::<i32>().unwrap(),
                    "dec" => *entry -= amount.parse::<i32>().unwrap(),
                    _ => unreachable!(),
                }
            }
            max = std::cmp::max(max, *data.values().max().unwrap_or(&0));
        }

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
