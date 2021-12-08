#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Hello, world!");
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let crabs = input
        .trim()
        .split(",")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let min: i32 = *crabs.iter().min().unwrap();
    let max: i32 = *crabs.iter().max().unwrap();

    let mut min_sum = i32::MAX;
    for i in min..max {
        let mut sum = 0;
        for crab in crabs.iter() {
            sum += (crab - i).abs();
        }
        if sum < min_sum {
            min_sum = sum;
        }
    }

    Ok(min_sum)
}

fn part_2(input: &str) -> AocResult<i32> {
    let crabs = input
        .trim()
        .split(",")
        .map(|token| token.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let min: i32 = *crabs.iter().min().unwrap();
    let max: i32 = *crabs.iter().max().unwrap();

    let mut min_sum = i32::MAX;
    for i in min..max {
        let mut sum = 0;
        for crab in crabs.iter() {
            let n = (crab - i).abs();
            sum += n * (n + 1) / 2;
        }
        println!("sum={}", sum);
        if sum < min_sum {
            min_sum = sum;
        }
    }

    Ok(min_sum)
}
