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

struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn new(height: i32) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut trees: Vec<Vec<Tree>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Tree::new(char::to_digit(c, 10).unwrap() as i32))
                    .collect()
            })
            .collect();

        // from left
        for row in trees.iter_mut() {
            let mut max = -1;
            for tree in row.iter_mut() {
                if tree.height > max {
                    tree.visible = true;
                }
                max = std::cmp::max(max, tree.height);
            }
        }

        // from right
        for row in trees.iter_mut() {
            let mut max = -1;
            for tree in row.iter_mut().rev() {
                if tree.height > max {
                    tree.visible = true;
                }
                max = std::cmp::max(max, tree.height);
            }
        }

        // from top
        for column in 0..trees[0].len() {
            let mut max = -1;
            for row in 0..trees.len() {
                let tree = &mut trees[row as usize][column as usize];
                if tree.height > max {
                    tree.visible = true;
                }
                max = std::cmp::max(max, tree.height);
            }
        }

        // from bottom
        for column in 0..trees[0].len() {
            let mut max = -1;
            for row in (0..trees.len()).rev() {
                let tree = &mut trees[row as usize][column as usize];
                if tree.height > max {
                    tree.visible = true;
                }
                max = std::cmp::max(max, tree.height);
            }
        }

        print_visibility(&trees);

        let answer = trees.iter().flatten().filter(|tree| tree.visible).count();
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let trees: Vec<Vec<Tree>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Tree::new(char::to_digit(c, 10).unwrap() as i32))
                    .collect()
            })
            .collect();

        let mut max_score = 0;
        let deltas: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for row in 0..trees.len() {
            for col in 0..trees[0 as usize].len() {
                let tree = &trees[row as usize][col as usize];
                let mut scenic_score = 1;
                for delta in deltas.iter() {
                    let mut directional_score = 0;
                    let mut cursor = (row as i32, col as i32);
                    loop {
                        cursor = (cursor.0 + delta.0, cursor.1 + delta.1);
                        if cursor.0 < 0
                            || cursor.0 >= trees.len() as i32
                            || cursor.1 < 0
                            || cursor.1 >= trees[0].len() as i32
                        {
                            break;
                        }
                        directional_score += 1;
                        let viewed_tree = &trees[cursor.0 as usize][cursor.1 as usize];
                        if viewed_tree.height >= tree.height {
                            break;
                        }
                    }
                    scenic_score *= directional_score;
                }
                max_score = std::cmp::max(max_score, scenic_score);
            }
        }
        Ok(format!("{}", max_score))
    }
}

fn print_visibility(trees: &Vec<Vec<Tree>>) {
    for row in trees.iter() {
        for tree in row.iter() {
            print!("{}", if tree.visible { 'O' } else { 'X' });
        }
        println!();
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
