#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    println!("Hello, world!");
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut ops = input
        .split(",")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    ops[1] = 12;
    ops[2] = 2;

    let mut i = 0;
    while i < ops.len() {
        match ops[i] {
            1 => {
                let i1 = ops[i + 1] as usize;
                let i2 = ops[i + 2] as usize;
                let o = ops[i + 3] as usize;
                ops[o] = ops[i1] + ops[i2];
            }
            2 => {
                let i1 = ops[i + 1] as usize;
                let i2 = ops[i + 2] as usize;
                let o = ops[i + 3] as usize;
                ops[o] = ops[i1] * ops[i2];
            }
            99 => break,
            _ => panic!("ahhh ops[{}]={}", i, ops[i]),
        }
        i += 4;
    }
    Ok(ops[0])
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut ops = input
        .split(",")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut done = false;
    let mut solution = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut ops = ops.clone();
            ops[1] = noun;
            ops[2] = verb;
            let mut i = 0;
            while i < ops.len() {
                match ops[i] {
                    1 => {
                        let i1 = ops[i + 1] as usize;
                        let i2 = ops[i + 2] as usize;
                        let o = ops[i + 3] as usize;
                        ops[o] = ops[i1] + ops[i2];
                    }
                    2 => {
                        let i1 = ops[i + 1] as usize;
                        let i2 = ops[i + 2] as usize;
                        let o = ops[i + 3] as usize;
                        ops[o] = ops[i1] * ops[i2];
                    }
                    99 => break,
                    _ => panic!("ahhh ops[{}]={}", i, ops[i]),
                }
                i += 4;
            }
            if ops[0] == 19690720 {
                solution = 100 * noun + verb;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
