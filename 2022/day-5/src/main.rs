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
        let first_line_size = input.lines().next().unwrap().len();
        let size = (first_line_size + 1) / 4;
        let mut stacks = (0..size).map(|_| Vec::<char>::new()).collect::<Vec<_>>();

        let mut lines = input.lines();

        loop {
            let line = lines.next().unwrap();
            if &line[1..2] == "1" {
                break;
            }
            let mut chars = line.chars();
            for i in 0..size {
                let c = if i == 0 {
                    (&mut chars).skip(1).next().unwrap()
                } else {
                    (&mut chars).skip(3).next().unwrap()
                };
                if c != ' ' {
                    (&mut stacks)[i as usize].push(c);
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        for line in lines.skip(1) {
            let mut iter = line
                .split(' ')
                .filter(|x| x.chars().all(|c| c.is_numeric()));
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let from = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let to = iter.next().unwrap().parse::<usize>().unwrap() - 1;

            for i in 0..count {
                let val = (&mut stacks)[from].pop().unwrap();
                (&mut stacks)[to].push(val);
            }
        }

        let answer = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();

        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let first_line_size = input.lines().next().unwrap().len();
        let size = (first_line_size + 1) / 4;
        let mut stacks = (0..size).map(|_| Vec::<char>::new()).collect::<Vec<_>>();

        let mut lines = input.lines();

        loop {
            let line = lines.next().unwrap();
            if &line[1..2] == "1" {
                break;
            }
            let mut chars = line.chars();
            for i in 0..size {
                let c = if i == 0 {
                    (&mut chars).skip(1).next().unwrap()
                } else {
                    (&mut chars).skip(3).next().unwrap()
                };
                if c != ' ' {
                    (&mut stacks)[i as usize].push(c);
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        for line in lines.skip(1) {
            let mut iter = line
                .split(' ')
                .filter(|x| x.chars().all(|c| c.is_numeric()));
            let count = iter.next().unwrap().parse::<usize>().unwrap();
            let from = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let to = iter.next().unwrap().parse::<usize>().unwrap() - 1;

            let len = stacks[from].len();
            let mut val: Vec<char> = (&mut stacks)[from].drain((len - count)..).collect();
            (&mut stacks)[to].append(&mut val);
        }

        let answer = stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect::<String>();

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
