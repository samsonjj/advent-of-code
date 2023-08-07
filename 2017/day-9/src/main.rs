#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::{Chars, FromStr};
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

fn next(chars: &mut Chars<'_>) -> Option<char> {
    let mut c = chars.next()?;
    while c == '!' {
        chars.next()?;
        c = chars.next()?;
    }
    Some(c)
}

fn parse(chars: &mut Chars<'_>, x: i32) -> (i32, i32) {
    let mut sum = 0;
    let mut garbage = 0;
    loop {
        let c = chars.next();
        if let None = c {
            return (sum, garbage);
        }
        let c = c.unwrap();
        match c {
            '{' => {
                let result = parse(chars, x + 1);
                sum += result.0;
                garbage += result.1;
            }
            '}' => {
                return (sum + x, garbage);
            }
            '<' => {
                while next(chars) != Some('>') {
                    garbage += 1;
                }
                (0, 0);
            }
            ',' => {}
            _ => unreachable!(),
        }
    }
}

impl AocSolver for Temp {
    // 28:37
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut chars = input.chars();
        let answer = parse(&mut chars, 0);
        Ok(format!("{:?}", answer))
    }

    // 32:05
    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut chars = input.chars();
        let answer = parse(&mut chars, 0);
        Ok(format!("{:?}", answer))
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
