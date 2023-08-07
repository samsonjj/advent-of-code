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

fn sort_token(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    return chars.into_iter().collect();
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut token_set = HashSet::new();
    return passphrase
        .split_whitespace()
        .find(|token| {
            if token_set.contains(&token.to_string()) {
                return true;
            }
            token_set.insert(token.to_string());
            return false;
        })
        .is_none();
}

fn is_valid_passphrase_2(passphrase: &str) -> bool {
    let mut token_set = HashSet::new();
    return passphrase
        .split_whitespace()
        .find(|token| {
            let token = sort_token(token);
            if token_set.contains(&token.to_string()) {
                return true;
            }
            token_set.insert(token.to_string());
            return false;
        })
        .is_none();
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let answer = input
            .lines()
            .filter(|line| is_valid_passphrase(line))
            .count();

        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = input
            .lines()
            .filter(|line| is_valid_passphrase_2(line))
            .count();

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
