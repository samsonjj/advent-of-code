#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

struct Cave {
    risk: Vec<Vec<i32>>,
    unvisited: HashSet<Point>,
    visited: HashSet<Point>,
    dist: Vec<Vec<i32>>,
}

impl Cave {
    fn is_visited(&self, point: Point) -> bool {
        self.visited.contains(&point)
    }
    fn get_risk(&self, point: Point) -> i32 {
        self.risk[point.y][point.x]
    }
    fn get_dist(&self, point: Point) -> i32 {
        self.dist[point.y][point.x]
    }
    fn set_dist(&mut self, point: Point, dist: i32) {
        self.dist[point.y][point.x] = dist;
    }
    fn width(&self) -> usize {
        self.risk[0].len()
    }
    fn height(&self) -> usize {
        self.risk.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

fn dijkstra(mut cave: Cave) -> i32 {
    cave.set_dist(Point::zero(), 0);
    cave.unvisited = HashSet::new();
    cave.unvisited.insert(Point { x: 0, y: 0 });

    let end_point = Point { x: cave.width()-1, y: cave.height() -1 };
    let mut max_len = 0;
    while !cave.unvisited.is_empty() {
        let mut min_dist = i32::MAX;
        let mut curr = Point::zero();
        if cave.unvisited.len() > max_len {
            max_len = cave.unvisited.len();
        }
        for point in cave.unvisited.iter() {
            if cave.get_dist(*point) < min_dist {
                curr = *point;
                min_dist = cave.get_dist(*point);
            }
        }
        if curr == end_point {
            println!("max_len={}", max_len);
            return cave.get_dist(end_point);
        }
        for direction in DIRECTIONS {
            let point = curr.delta(direction);
            if point.x >= cave.width() || point.y >= cave.height() || cave.is_visited(point) {
                continue;
            }
            let dist = cave.get_dist(curr) + cave.get_risk(point);
            if dist < cave.get_dist(point) {
                cave.set_dist(point, dist);
            }
            cave.unvisited.insert(point);
        }
        cave.unvisited.remove(&curr);
        cave.visited.insert(curr);
    }
    cave.get_dist(Point { x: cave.width()-1, y: cave.height()-1 })
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn parse_input(input: &str) -> Cave {
    let mut cave = Cave { risk: vec![], unvisited: HashSet::new(), visited: HashSet::new(), dist: vec![] };
    for (i, line) in input.lines().enumerate() {
        let mut risk_row = vec![];
        let mut dist_row = vec![];
        for (j, c) in line.chars().enumerate() {
            cave.unvisited.insert(Point { x: j, y: i });
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
    Ok(dijkstra(cave))
}

fn add_wrap(a: i32, times: i32) -> i32 {
    (a + times - 1) % 9 + 1
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut cave = parse_input(input);

    // extend vertically
    let width = cave.width();
    let height = cave.height();
    for k in 1..5 {
        for i in 0..height {
            let mut risk_row = vec![];
            let mut dist_row = vec![];
            for j in 0..width {
                cave.unvisited.insert(Point { x: j, y: i + k * height });
                risk_row.push(add_wrap(cave.risk[i][j], k as i32));
                dist_row.push(i32::MAX);
            }
            cave.risk.push(risk_row);
            cave.dist.push(dist_row);
        }
    }

    let height = cave.height();
    let width = cave.width();

    // extend horizontally
    for i in 0..height {
        for k in 1..5 {
            for j in 0..width {
                let x = j + k * width;
                cave.unvisited.insert(Point { x, y: i });
                let last_val = cave.risk[i][x - width];
                cave.risk[i].push(add_wrap(last_val, 1));
                cave.dist[i].push(i32::MAX);
            }
        }
    }

    Ok(dijkstra(cave))
}
