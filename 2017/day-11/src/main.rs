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
    // 11:46
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut steps: HashMap<&str, i32> = HashMap::new();
        for token in input.split(",") {
            let entry = steps.entry(token).or_insert(0);
            *entry += 1;
        }
        loop {
            if steps.get("n").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                steps.entry("n").and_modify(|x| *x -= 1);
                steps.entry("s").and_modify(|x| *x -= 1);
            } else if steps.get("ne").unwrap() > &0 && steps.get("sw").unwrap() > &0 {
                steps.entry("ne").and_modify(|x| *x -= 1);
                steps.entry("sw").and_modify(|x| *x -= 1);
            } else if steps.get("se").unwrap() > &0 && steps.get("nw").unwrap() > &0 {
                steps.entry("nw").and_modify(|x| *x -= 1);
                steps.entry("se").and_modify(|x| *x -= 1);
            } else if steps.get("ne").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                steps.entry("ne").and_modify(|x| *x -= 1);
                steps.entry("s").and_modify(|x| *x -= 1);
                steps.entry("se").and_modify(|x| *x += 1);
            } else if steps.get("se").unwrap() > &0 && steps.get("n").unwrap() > &0 {
                steps.entry("se").and_modify(|x| *x -= 1);
                steps.entry("n").and_modify(|x| *x -= 1);
                steps.entry("ne").and_modify(|x| *x += 1);
            } else if steps.get("nw").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                steps.entry("nw").and_modify(|x| *x -= 1);
                steps.entry("s").and_modify(|x| *x -= 1);
                steps.entry("sw").and_modify(|x| *x += 1);
            } else if steps.get("sw").unwrap() > &0 && steps.get("n").unwrap() > &0 {
                steps.entry("sw").and_modify(|x| *x -= 1);
                steps.entry("n").and_modify(|x| *x -= 1);
                steps.entry("nw").and_modify(|x| *x += 1);
            } else if steps.get("nw").unwrap() > &0 && steps.get("ne").unwrap() > &0 {
                steps.entry("ne").and_modify(|x| *x -= 1);
                steps.entry("nw").and_modify(|x| *x -= 1);
                steps.entry("n").and_modify(|x| *x += 1);
            } else if steps.get("sw").unwrap() > &0 && steps.get("se").unwrap() > &0 {
                steps.entry("se").and_modify(|x| *x -= 1);
                steps.entry("sw").and_modify(|x| *x -= 1);
                steps.entry("s").and_modify(|x| *x += 1);
            } else {
                break;
            }
        }

        Ok(format!("{}", steps.values().sum::<i32>()))
    }

    // 14:22
    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut steps: HashMap<&str, i32> = HashMap::new();
        steps.insert("n", 0);
        steps.insert("s", 0);
        steps.insert("ne", 0);
        steps.insert("nw", 0);
        steps.insert("se", 0);
        steps.insert("sw", 0);
        let mut max_distance = 0;
        for token in input.split(",") {
            let entry = steps.entry(token).or_insert(0);
            *entry += 1;
            loop {
                if steps.get("n").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                    steps.entry("n").and_modify(|x| *x -= 1);
                    steps.entry("s").and_modify(|x| *x -= 1);
                } else if steps.get("ne").unwrap() > &0 && steps.get("sw").unwrap() > &0 {
                    steps.entry("ne").and_modify(|x| *x -= 1);
                    steps.entry("sw").and_modify(|x| *x -= 1);
                } else if steps.get("se").unwrap() > &0 && steps.get("nw").unwrap() > &0 {
                    steps.entry("nw").and_modify(|x| *x -= 1);
                    steps.entry("se").and_modify(|x| *x -= 1);
                } else if steps.get("ne").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                    steps.entry("ne").and_modify(|x| *x -= 1);
                    steps.entry("s").and_modify(|x| *x -= 1);
                    steps.entry("se").and_modify(|x| *x += 1);
                } else if steps.get("se").unwrap() > &0 && steps.get("n").unwrap() > &0 {
                    steps.entry("se").and_modify(|x| *x -= 1);
                    steps.entry("n").and_modify(|x| *x -= 1);
                    steps.entry("ne").and_modify(|x| *x += 1);
                } else if steps.get("nw").unwrap() > &0 && steps.get("s").unwrap() > &0 {
                    steps.entry("nw").and_modify(|x| *x -= 1);
                    steps.entry("s").and_modify(|x| *x -= 1);
                    steps.entry("sw").and_modify(|x| *x += 1);
                } else if steps.get("sw").unwrap() > &0 && steps.get("n").unwrap() > &0 {
                    steps.entry("sw").and_modify(|x| *x -= 1);
                    steps.entry("n").and_modify(|x| *x -= 1);
                    steps.entry("nw").and_modify(|x| *x += 1);
                } else if steps.get("nw").unwrap() > &0 && steps.get("ne").unwrap() > &0 {
                    steps.entry("ne").and_modify(|x| *x -= 1);
                    steps.entry("nw").and_modify(|x| *x -= 1);
                    steps.entry("n").and_modify(|x| *x += 1);
                } else if steps.get("sw").unwrap() > &0 && steps.get("se").unwrap() > &0 {
                    steps.entry("se").and_modify(|x| *x -= 1);
                    steps.entry("sw").and_modify(|x| *x -= 1);
                    steps.entry("s").and_modify(|x| *x += 1);
                } else {
                    break;
                }
            }
            max_distance = std::cmp::max(max_distance, steps.values().sum::<i32>());
        }
        Ok(format!("{}", max_distance))
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
