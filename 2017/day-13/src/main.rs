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

fn caught(depth: i32, range: i32) -> bool {
    if range == 1 {
        return true;
    }
    // time at which we enter the layer
    let time = depth;
    // Ex: range = 5
    // -> [0][1][2][3][4]
    // <- [8][7][6][5][4]
    let position = depth % (range * 2 - 2);

    // caught if position is zero when we enter
    position == 0
}

struct Layer {
    depth: i32,
    range: i32,
}

struct Firewall {
    layers: Vec<Layer>,
}

impl Firewall {
    fn new(input: &str) -> Self {
        let layers = input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let depth = parts.next().unwrap().parse::<i32>().unwrap();
                let range = parts.next().unwrap().parse::<i32>().unwrap();
                Layer { depth, range }
            })
            .collect::<Vec<Layer>>();
        Self { layers }
    }
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let firewall = Firewall::new(input);
        let mut severity = 0;
        for layer in firewall.layers.iter() {
            if caught(layer.depth, layer.range) {
                severity += layer.depth * layer.range;
            }
        }
        Ok(format!("{}", severity))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 5;
        let firewall = Firewall::new(input);
        let mut offset = 0;
        loop {
            if !firewall
                .layers
                .iter()
                .any(|layer| caught(layer.depth + offset, layer.range))
            {
                return Ok(format!("{}", offset));
            }
            offset += 1;
        }
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
