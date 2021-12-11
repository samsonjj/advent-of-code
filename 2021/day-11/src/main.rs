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

fn print_octopi(o: &Vec<Vec<i32>>) {
    for row in o {
        for n in row {
            print!("{}", n);
        }
        println!();
    }
}

fn part_1(input: &str) -> AocResult<i32> {
    let DISPLACEMENTS = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let mut octopi: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        octopi.push(row);
    }
    let mut answer = None;
    let mut sum = 0;
    for k in 0..100 {
        for i in 0..octopi.len() {
            for j in 0..octopi[i].len() {
                octopi[i][j] += 1;
            }
        }

        // if !octopi.iter().any(|row| row.iter().any(|num| num > &9)) {
        //     break;
        // }

        while true {
            let mut done = true;
            for i in 0..octopi.len() {
                for j in 0..octopi[i].len() {
                    if octopi[i][j] > 9 {
                        done = false;
                        sum += 1;
                        octopi[i][j] = -1;
                        for d in DISPLACEMENTS.iter() {
                            let x = (i as i32 + d.0);
                            let y = (j as i32 + d.1);
                            if x < 0
                                || x >= octopi.len() as i32
                                || y < 0
                                || y >= octopi[i].len() as i32
                            {
                                continue;
                            }
                            if octopi[x as usize][y as usize] != -1 {
                                octopi[x as usize][y as usize] += 1;
                            }
                        }
                    }
                }
            }
            if done {
                break;
            }
        }

        let mut all = true;
        for i in 0..octopi.len() {
            for j in 0..octopi[i].len() {
                if octopi[i][j] == -1 {
                    octopi[i][j] = 0;
                } else {
                    all = false
                }
            }
        }
        if all && matches!(answer, None) {
            answer = Some(k);
        }
    }
    Ok(sum)
}

fn part_2(input: &str) -> AocResult<i32> {
    let DISPLACEMENTS = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];
    let mut octopi: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        octopi.push(row);
    }
    let mut answer = None;
    let mut sum = 0;
    for k in 0..10000 {
        println!("k={}", k);
        print_octopi(&octopi);
        for i in 0..octopi.len() {
            for j in 0..octopi[i].len() {
                octopi[i][j] += 1;
            }
        }
        println!();

        // if !octopi.iter().any(|row| row.iter().any(|num| num > &9)) {
        //     break;
        // }

        while true {
            let mut done = true;
            for i in 0..octopi.len() {
                for j in 0..octopi[i].len() {
                    if octopi[i][j] > 9 {
                        done = false;
                        sum += 1;
                        octopi[i][j] = -1;
                        for d in DISPLACEMENTS.iter() {
                            let x = (i as i32 + d.0);
                            let y = (j as i32 + d.1);
                            if x < 0
                                || x >= octopi.len() as i32
                                || y < 0
                                || y >= octopi[i].len() as i32
                            {
                                continue;
                            }
                            if octopi[x as usize][y as usize] != -1 {
                                if x == 0 && y == 0 {
                                    println!("hello");
                                }
                                octopi[x as usize][y as usize] += 1;
                            }
                        }
                    }
                }
            }
            if done {
                break;
            }
        }

        for i in 0..octopi.len() {
            for j in 0..octopi[i].len() {
                if octopi[i][j] == -1 {
                    octopi[i][j] = 0;
                }
            }
        }

        println!("end");
        print_octopi(&octopi);

        if octopi.iter().all(|row| row.iter().all(|num| num == &0)) {
            answer = Some(k + 1);
            println!("haha!");
            break;
        }
    }
    println!("answer={:?}", answer);
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
