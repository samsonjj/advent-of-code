#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("../input.txt");
static EXAMPLE: &str = include_str!("../example.txt");

#[derive(Clone, Debug)]
struct Valve {
    label: String,
    index: usize,
    flow_rate: i32,
    neighbors: Vec<usize>,
}

fn parse(input: &str) -> Vec<Valve> {
    let mut lookup_table: HashMap<&str, usize> = HashMap::new();
    let regex_pattern = r"Valve (.*) has flow rate=(\d*); tunnels? leads? to valves? (.*)";
    let re = regex::Regex::new(regex_pattern).unwrap();

    let collected = input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str(),
            )
        })
        .collect::<Vec<_>>();

    for (i, item) in collected.iter().enumerate() {
        lookup_table.insert(item.0, i);
    }

    let valves = collected
        .into_iter()
        .map(|(label, flow_rate, neighbors)| Valve {
            label: label.to_string(),
            index: *lookup_table.get(label).unwrap(),
            flow_rate,
            neighbors: neighbors
                .split(", ")
                .map(|label| *lookup_table.get(label).unwrap())
                .collect(),
        })
        .collect();

    valves
}

fn floyd_warshall(valves: &Vec<Valve>) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![i32::MAX / 2; valves.len()]; valves.len()];
    for valve in valves.iter() {
        dist[valve.index][valve.index] = 0;
        for neighbor in valve.neighbors.iter() {
            dist[valve.index][*neighbor] = 1;
        }
    }

    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    dbg!(valves);
    dist
}

fn main() {
    let temp = Temp {};
    temp.execute(EXAMPLE);
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let answer = 2;
        let valves = parse(input);
        let dist = floyd_warshall(&valves);

        dbg!(dist);

        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 2;
        Ok(format!("{}", answer))
    }
}
