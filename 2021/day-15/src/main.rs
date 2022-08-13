#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::str::FromStr;
use std::cmp::Ordering;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

struct Cave {
    risk: Vec<Vec<i32>>,
    open: BinaryHeap<State>,
    closed: HashSet<Point>,
    dist: Vec<Vec<i32>>,
}

impl Cave {
    fn is_visited(&self, point: Point) -> bool {
        self.closed.contains(&point)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Point,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn zero() -> Self {
        Self { x: 0, y: 0}
    }
}

impl Point {
    fn delta(&self, d: (i32, i32)) -> Self {
        let x = if d.0.is_negative() {
            self.x.wrapping_sub(d.0.wrapping_abs() as u32 as usize)
        } else {
            self.x.wrapping_add(d.0 as usize)
        };
        let y = if d.1.is_negative() {
            self.y.wrapping_sub(d.1.wrapping_abs() as u32 as usize)
        } else {
            self.y.wrapping_add(d.1 as usize)
        };
        Self { x, y }
    }
}

static DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
];

fn dijkstra(mut cave: Cave, goal: Point) -> i32 {
    cave.dist[0][0] = 0;
    cave.open.push(State{ position: Point { x: 0, y: 0 }, cost: 0 });
    while let Some(State { cost, position }) = cave.open.pop() {
        if position == goal {
            return cave.dist[position.y][position.x];
        }
        if cost > cave.dist[position.y][position.x] {
            continue;
        }
        for direction in DIRECTIONS {
            let next_pos= position.delta(direction);
            if next_pos.x >= cave.risk[0].len() || next_pos.y >= cave.risk.len() || cave.is_visited(next_pos) {
                continue;
            }
            let cost = cave.dist[position.y][position.x] + cave.risk[next_pos.y][next_pos.x];
            let state = State { position: next_pos, cost };
            if state.cost < cave.dist[state.position.y][state.position.x] {
                cave.dist[state.position.y][state.position.x] = state.cost;
                cave.open.push(state);
            }
        }
    }
    cave.dist[cave.dist.len()-1][cave.dist[0].len()-1]
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn parse_input(input: &str) -> Cave {
    let mut cave = Cave { risk: vec![], open: BinaryHeap::new(), closed: HashSet::new(), dist: vec![] };
    for (i, line) in input.lines().enumerate() {
        let mut risk_row = vec![];
        let mut dist_row = vec![];
        for (j, c) in line.chars().enumerate() {
            risk_row.push(c.to_digit(10).unwrap() as i32);
            dist_row.push(i32::MAX);
        }
        cave.risk.push(risk_row);
        cave.dist.push(dist_row);
    }
    cave
}

fn part_1(input: &str) -> AocResult<i32> {
    let cave = parse_input(input);
    let goal = Point { x: cave.risk[0].len()-1, y: cave.risk.len()-1 };
    Ok(dijkstra(cave, goal))
}

fn add_wrap(a: i32, times: i32) -> i32 {
    (a + times - 1) % 9 + 1
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut cave = parse_input(input);

    // extend vertically
    let width = cave.risk[0].len();
    let height = cave.risk.len();
    for k in 1..5 {
        for i in 0..height {
            let mut risk_row = vec![];
            let mut dist_row = vec![];
            for j in 0..width {
                risk_row.push(add_wrap(cave.risk[i][j], k as i32));
                dist_row.push(i32::MAX);
            }
            cave.risk.push(risk_row);
            cave.dist.push(dist_row);
        }
    }

    let height = cave.risk.len();
    let width = cave.risk[0].len();

    // extend horizontally
    for i in 0..height {
        for k in 1..5 {
            for j in 0..width {
                let x = j + k * width;
                let last_val = cave.risk[i][x - width];
                cave.risk[i].push(add_wrap(last_val, 1));
                cave.dist[i].push(i32::MAX);
            }
        }
    }

    let goal = Point { x: cave.risk[0].len()-1, y: cave.risk.len()-1 };
    Ok(dijkstra(cave, goal))
}
