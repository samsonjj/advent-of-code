#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
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
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut programs: VecDeque<char> = ('a'..='p').collect();
        apply(&mut programs, input)?;
        Ok(format!("{}", programs.iter().collect::<String>()))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut programs: VecDeque<char> = ('a'..='p').collect();
        let initial = programs.clone();

        let mut count = 0;
        while {
            apply(&mut programs, input)?;
            count += 1;
            programs != initial
        } {}

        for _ in 0..(1_000_000 % count) {
            apply(&mut programs, input)?;
        }

        Ok(format!("{}", programs.iter().collect::<String>()))
    }
}

fn apply(programs: &mut VecDeque<char>, instructions: &str) -> AocResult<()> {
    for instruction in instructions.split(",") {
        match &instruction[0..1] {
            "s" => {
                let num_programs: usize = instruction[1..].parse()?;
                for i in 0..num_programs {
                    let tmp = programs.pop_back().unwrap();
                    programs.push_front(tmp);
                }
            }
            "x" => {
                let mut parts = instruction[1..].split("/");
                let a: usize = parts.next().unwrap().parse().unwrap();
                let b: usize = parts.next().unwrap().parse().unwrap();
                programs.swap(a, b);
            }
            "p" => {
                let a = &instruction[1..2].chars().next().unwrap();
                let b = &instruction[3..4].chars().next().unwrap();
                programs.iter_mut().for_each(|x| {
                    if x == a {
                        *x = *b;
                    } else if x == b {
                        *x = *a;
                    }
                });
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
