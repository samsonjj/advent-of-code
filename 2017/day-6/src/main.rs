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

fn parse_input(s: &str) -> Vec<i32> {
    s.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut banks = parse_input(input);
        let mut seen_banks: Vec<Vec<i32>> = vec![];
        let mut steps = 0;

        while !seen_banks.contains(&banks) {
            steps += 1;
            seen_banks.push(banks.clone());
            let mut max_bank_index = 0;
            for (i, bank) in banks.iter().enumerate() {
                if bank > &banks[max_bank_index] {
                    max_bank_index = i;
                }
            }

            let mut blocks = banks[max_bank_index];
            banks[max_bank_index] = 0;
            let mut index = max_bank_index;
            while blocks > 0 {
                index = (index + 1) % banks.len();
                banks[index] += 1;
                blocks -= 1;
            }
        }
        Ok(format!("{}", steps))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut banks = parse_input(input);
        let mut seen_banks: HashMap<Vec<i32>, i32> = HashMap::new();
        let mut steps = 0;

        while !seen_banks.contains_key(&banks) {
            steps += 1;
            seen_banks.insert(banks.clone(), steps);
            let mut max_bank_index = 0;
            for (i, bank) in banks.iter().enumerate() {
                if bank > &banks[max_bank_index] {
                    max_bank_index = i;
                }
            }

            let mut blocks = banks[max_bank_index];
            banks[max_bank_index] = 0;
            let mut index = max_bank_index;
            while blocks > 0 {
                index = (index + 1) % banks.len();
                banks[index] += 1;
                blocks -= 1;
            }
        }
        Ok(format!("{}", steps - seen_banks.get(&banks).unwrap() + 1))
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
