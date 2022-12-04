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
        let rucksacks = input.lines();
        let duplicate_items = rucksacks.map(|line| {
            let size = line.chars().count();
            let left = &line[..size / 2];
            let right = &line[size / 2..];
            for c in left.chars() {
                if right.contains(c) {
                    return c;
                }
            }
            unreachable!();
        });
        let answer = duplicate_items
            .map(|c| {
                let ascii = c as i32;
                let priority = match ascii {
                    65..=90 => ascii - 65 + 27,
                    97..=122 => ascii - 97 + 1,
                    _ => unreachable!(),
                };
                priority
            })
            .sum::<i32>();
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let rucksacks: Vec<&str> = input.lines().collect();
        let badge_items = rucksacks.chunks(3).map(|lines| {
            lines
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|a, b| {
                    a.intersection(&b)
                        .into_iter()
                        .map(|c| c.clone())
                        .collect::<HashSet<_>>()
                })
                .unwrap()
        });

        let answer = badge_items
            .map(|c| {
                let ascii = c.into_iter().next().unwrap() as i32;
                let priority = match ascii {
                    65..=90 => ascii - 65 + 27,
                    97..=122 => ascii - 97 + 1,
                    _ => unreachable!(),
                };
                priority
            })
            .sum::<i32>();
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
