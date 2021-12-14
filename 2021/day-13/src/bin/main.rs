#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

enum Axis {
    X,
    Y,
}

fn print_paper(paper: &HashSet<(i32, i32)>) {
    let mut max = (0, 0);
    for (x, y) in paper.iter() {
        if x > &max.0 {
            max.0 = *x;
        }
        if y > &max.1 {
            max.1 = *y;
        }
    }
    for j in 0..=max.1 {
        for i in 0..=max.0 {
            if paper.contains(&(i as i32, j as i32)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(paper: &HashSet<(i32, i32)>, index: i32, axis: Axis) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    for (x, y) in paper {
        match axis {
            Axis::X => {
                if x > &index {
                    result.insert((index - (x - index), *y));
                } else {
                    result.insert((*x, *y));
                }
            }
            Axis::Y => {
                if y > &index {
                    result.insert((*x, index - (y - index)));
                } else {
                    result.insert((*x, *y));
                }
            }
        }
    }
    result
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut paper = HashSet::new();
    let mut iter = input.split("\n\n");
    let dots = iter.next().unwrap();
    let folds = iter.next().unwrap();

    for line in dots.lines() {
        let mut nums = line.split(",").map(|token| token.parse::<i32>().unwrap());
        let coord = (nums.next().unwrap(), nums.next().unwrap());
        paper.insert(coord);
    }

    let first_fold = &folds.lines().next().unwrap()[13..];
    let x_fold = first_fold.parse::<i32>().unwrap();

    let result = fold(&paper, x_fold, Axis::X);

    Ok(result.len() as i32)
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut paper = HashSet::new();
    let mut iter = input.split("\n\n");
    let dots = iter.next().unwrap();
    let folds = iter.next().unwrap();

    for line in dots.lines() {
        let mut nums = line.split(",").map(|token| token.parse::<i32>().unwrap());
        let coord = (nums.next().unwrap(), nums.next().unwrap());
        paper.insert(coord);
    }

    for s in folds.lines() {
        paper = fold(
            &paper,
            s[13..].parse::<i32>().unwrap(),
            match &s[11..12] {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("invalid something {}", &s[11..12]),
            },
        );
    }

    println!();
    print_paper(&paper);
    println!();
    Ok(paper.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
