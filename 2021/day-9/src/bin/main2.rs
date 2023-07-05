#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("../input.txt");
static EXAMPLE: &str = include_str!("../example.txt");

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| {
                    return c.to_digit(10).unwrap() as i32;
                })
                .collect()
        })
        .collect()
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let data = parse_input(INPUT);
    let mut sum = 0;

    let mut low_points = Vec::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let mut low_point = true;
            for d in DIRECTIONS {
                let a = i as i32 + d.0;
                let b = j as i32 + d.1;
                if !(a < 0 || a >= data.len() as i32 || b < 0 || b >= data[i].len() as i32) {
                    if data[i][j] >= data[a as usize][b as usize] {
                        low_point = false;
                    }
                }
            }
            if low_point {
                low_points.push((i as i32, j as i32));
                sum += data[i][j] + 1;
            }
        }
    }
    println!("{}", sum);

    // part 2
    let mut visited = Vec::new();
    for i in 0..data.len() {
        visited.push(Vec::new());
        for j in 0..data[i].len() {
            visited[i].push(false);
        }
    }

    let mut basins = Vec::new();
    for lp in low_points {
        println!("searching ({}, {})", lp.0, lp.1);
        let basin = search(lp.0, lp.1, -1, &data, &mut visited);
        basins.push(basin);
        println!("found basin: {}", basin);
    }
    basins.sort();
    basins.reverse();
    let mut it = basins.iter();
    let mut result = 1;
    println!("{:?}", basins);
    result *= it.next().unwrap();
    result *= it.next().unwrap();
    result *= it.next().unwrap();

    println!("{}", result);
}

fn search(i: i32, j: i32, last: i32, data: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    if i < 0 || i >= data.len() as i32 || j < 0 || j >= data[i as usize].len() as i32 {
        return 0;
    }
    if visited[i as usize][j as usize] {
        return 0;
    }
    if data[i as usize][j as usize] <= last {
        return 0;
    }
    if data[i as usize][j as usize] == 9 {
        return 0;
    }

    visited[i as usize][j as usize] = true;

    let mut result = 1;

    for d in DIRECTIONS {
        result += search(
            i + d.0,
            j + d.1,
            data[i as usize][j as usize],
            data,
            visited,
        );
    }

    return result;
}
