#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

const LIST_SIZE: i32 = 256;

struct Ring {
    skip_size: i32,
    current_position: i32,
    values: Vec<i32>,
}

impl Ring {
    fn new() -> Self {
        Self {
            skip_size: 0,
            current_position: 0,
            values: (0..LIST_SIZE).collect(),
        }
    }

    fn reverse(&mut self, length: i32) {
        for i in 0..length / 2 {
            let index = (self.current_position + i) % LIST_SIZE;
            let other_index = (self.current_position + length - i - 1) % LIST_SIZE;
            self.values.swap(index as usize, other_index as usize);
        }
        self.current_position += length + self.skip_size;
        self.skip_size += 1;
    }
}

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
    fn part_1(&self, input: &str) -> AocResult<String> {
        let grid: usize = (0..128)
            .map(|row| {
                let key = format!("{}-{}", input, row);
                let hash = knot_hash(key.as_str());
                return hash.chars().filter(|c| c == &'1').count();
            })
            .sum();
        Ok(format!("{:?}", grid))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 5;
        Ok(format!("{}", answer))
    }
}

fn knot_hash(input: &str) -> String {
    let mut ring = Ring::new();
    let mut lengths = input.trim().chars().map(|c| c as i32).collect::<Vec<i32>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    for round in 0..64 {
        for length in lengths.iter() {
            ring.reverse(*length);
        }
    }

    let hexes = ring
        .values
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
        .map(|result| format!("{result:08b}"))
        .collect::<Vec<_>>();

    hexes.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
