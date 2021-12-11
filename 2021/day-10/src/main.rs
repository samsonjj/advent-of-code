#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn score(c: char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid char {}", c),
    }
}

fn points(c: char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid char {}", c),
    }
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            if matches!(c, '(' | '[' | '{' | '<') {
                stack.push(c);
            } else if matches!(c, ')' | ']' | '}' | '>') {
                let last = stack.pop().unwrap();
                let expected = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => panic!("invalid char {}", c),
                };
                if last != expected {
                    sum += points(c);
                    break;
                }
            }
        }
    }
    Ok(sum as u64)
}

fn part_2(input: &str) -> AocResult<u64> {
    let mut scores = vec![];
    for line in input.lines() {
        let mut stack = vec![];
        let mut corrupted = false;
        for c in line.chars() {
            if matches!(c, '(' | '[' | '{' | '<') {
                stack.push(c);
            } else if matches!(c, ')' | ']' | '}' | '>') {
                let last = stack.pop().unwrap();
                let expected = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => panic!("invalid char {}", c),
                };
                if last != expected {
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            let mut s = 0u64;
            while !stack.is_empty() {
                let c = match stack.pop().unwrap() {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    x => panic!("invalid char {}", x),
                };
                s *= 5;
                s += score(c) as u64;
            }
            scores.push(s);
        }
    }
    scores.sort();
    Ok(scores[scores.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
