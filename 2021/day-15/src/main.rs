#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

struct Cave {
    risk: Vec<Vec<i32>>,
    unvisited: HashSet<Point>,
    tent_dist: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Cave {
    fn is_visited(&self, point: Point) -> bool {
        !self.unvisited.contains(&point)
    }
    fn get_risk(&self, point: Point) -> i32 {
        self.risk[point.y][point.x]
    }
    fn get_tent_dist(&self, point: Point) -> i32 {
        self.tent_dist[point.y][point.x]
    }
    fn set_tent_dist(&mut self, point: Point, dist: i32) {
        self.tent_dist[point.y][point.x] = dist;
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

fn dijkstra(mut cave: Cave) -> i32 {
    cave.set_tent_dist(Point::zero(), 0);

    while !cave.unvisited.is_empty() {
        let mut min_tent_dist = i32::MAX;
        let mut curr = Point::zero();
        for point in cave.unvisited.iter() {
            if cave.get_tent_dist(*point) < min_tent_dist {
                curr = *point;
                min_tent_dist = cave.get_tent_dist(*point);
            }
        }
        for point in [
            Point { x: curr.x.wrapping_sub(1), y: curr.y },
            Point { x: curr.x + 1, y: curr.y },
            Point { x: curr.x, y: curr.y + 1 },
            Point { x: curr.x, y: curr.y.wrapping_sub(1) },
        ] {
            if point.x >= cave.width || point.y >= cave.height || cave.is_visited(point) {
                continue;
            }
            let dist = cave.get_tent_dist(curr) + cave.get_risk(point);
            if dist < cave.get_tent_dist(point) {
                cave.set_tent_dist(point, dist);
            }
        }
        cave.unvisited.remove(&curr);
    }
    cave.get_tent_dist(Point { x: cave.width-1, y: cave.height-1 })
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut cave = Cave { risk: vec![], unvisited: HashSet::new(), tent_dist: vec![], width: 0, height: 0 };
    for (i, line) in input.lines().enumerate() {
        let mut risk_row = vec![];
        let mut tent_dist_row = vec![];
        for (j, c) in line.chars().enumerate() {
            cave.unvisited.insert(Point { x: j, y: i });
            risk_row.push(c.to_digit(10).unwrap() as i32);
            tent_dist_row.push(i32::MAX);
        }
        cave.risk.push(risk_row);
        cave.tent_dist.push(tent_dist_row);
    }
    cave.height = cave.risk.len();
    cave.width = cave.risk[0].len();
    Ok(dijkstra(cave))
}

fn add_wrap(a: i32, times: i32) -> i32 {
    (a + times - 1) % 9 + 1
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut cave = Cave { risk: vec![], unvisited: HashSet::new(), tent_dist: vec![], width: 0, height: 0 };
    for (i, line) in input.lines().enumerate() {
        let mut risk_row = vec![];
        let mut tent_dist_row = vec![];
        for (j, c) in line.chars().enumerate() {
            cave.unvisited.insert(Point { x: j, y: i });
            risk_row.push(c.to_digit(10).unwrap() as i32);
            tent_dist_row.push(i32::MAX);
        }
        cave.risk.push(risk_row);
        cave.tent_dist.push(tent_dist_row);
    }
    for i in 1..5 {
        for j in 1..5 {
            let risk = cave.risk[i][j];
            cave.risk[i].push(add_wrap(risk, (i + j) as i32));
            cave.unvisited.insert(Point { x: j, y: i });
            cave.tent_dist[i].push(i32::MAX);
        }
    }
    println!("{:?}", cave.risk);
    cave.height = cave.risk.len();
    cave.width = cave.risk[0].len();
    println!("{}, {}", cave.width, cave.height);
    Ok(dijkstra(cave))
}
