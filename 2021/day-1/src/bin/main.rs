#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};
use std::num::ParseIntError;

const INPUT: &str = include_str!("../input.txt");

fn parse(s: &str) -> Result<i32, ParseIntError> {
    Ok(s.parse::<i32>()?)
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut iter = input.lines();
    let mut last = parse(iter.next().unwrap())?;
    let mut sum = 0;
    for line in iter {
        let num = parse(line)?;
        if num > last {
            sum += 1;
        }
        last = num;
    }

    Ok(sum)
}

fn part_2(input: &str) -> AocResult<i32> {
    let nums = input
        .lines()
        .map(|line| parse(line).unwrap())
        .collect::<Vec<i32>>();
    let mut window = nums[0] + nums[1] + nums[2];
    let mut sum = 0;
    for i in 3..nums.len() {
        let num = window - nums[i - 3] + nums[i];
        if num > window {
            sum += 1;
        }
        window = num;
    }

    Ok(sum)
}
