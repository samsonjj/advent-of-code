#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut sum = 0;
    let mut last = 0;
    for line in input.lines() {
        let num = line.parse::<i32>()?;
        if num > last {
            sum += 1;
        }
        last = num;
    }

    Ok(sum)
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut sum = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut iter = INPUT.lines().map(|line| line.parse::<i32>().unwrap());
    window.push_back(iter.next().unwrap());
    window.push_back(iter.next().unwrap());
    window.push_back(iter.next().unwrap());
    for item in iter {
        let prev_sum: i32 = window.iter().sum();
        window.pop_front();
        window.push_back(item);
        let curr_sum: i32 = window.iter().sum();
        if curr_sum > prev_sum {
            sum += 1;
        }
    }

    Ok(sum)
}
