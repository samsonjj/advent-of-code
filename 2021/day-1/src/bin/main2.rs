#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};

static INPUT: &str = include_str!("../input.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut iter = input.lines();
    let mut sum = 0;
    let mut last: i32 = iter.next().unwrap().parse()?;
    for line in INPUT.lines() {
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
    let mut window = [0i32; 3];
    let mut idx = 0;
    let mut iter = input.lines().map(|line| line.parse::<i32>().unwrap());

    window[idx] = iter.next().unwrap();
    idx = (idx + 1) % 3;
    window[idx] = iter.next().unwrap();
    idx = (idx + 1) % 3;
    window[idx] = iter.next().unwrap();
    idx = (idx + 1) % 3;

    let mut prev: i32 = window.iter().sum();
    for num in iter {
        let curr = prev - window[idx] + num;
        if curr > prev {
            sum += 1;
        }
        prev = curr;
        window[idx] = num;
        idx = (idx + 1) % 3;
    }

    Ok(sum)
}
