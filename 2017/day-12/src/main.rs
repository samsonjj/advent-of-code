#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
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

struct Graph {
    data: Vec<Vec<usize>>,
    visited: HashSet<usize>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.split(" <-> ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        Self {
            data,
            visited: HashSet::new(),
        }
    }
    fn dfs(&mut self, node: usize) {
        if self.visited.contains(&node) {
            return;
        }
        self.visited.insert(node);
        for neighbor in self.data.get(node).unwrap().clone() {
            self.dfs(neighbor);
        }
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut graph = Graph::new(input);
        graph.dfs(0);
        Ok(format!("{}", graph.visited.len()))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut graph = Graph::new(input);
        let mut count = 0;
        for node in 0..graph.data.len() {
            if !graph.visited.contains(&node) {
                graph.dfs(node);
                count += 1;
            }
        }
        Ok(format!("{}", count))
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
