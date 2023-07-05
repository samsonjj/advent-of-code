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

#[derive(Clone, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from_usizes(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    fn as_usizes(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
    fn add(&mut self, delta: &(i32, i32)) {
        self.x += delta.0;
        self.y += delta.1;
    }
    fn difference(&self, other: &Position) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut visited_positions: HashSet<Position> = HashSet::new();
        let mut head = Position::from_usizes(0, 0);
        let mut tail = Position::from_usizes(0, 0);

        visited_positions.insert(tail.clone());

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap();
            let num_steps = parts.next().unwrap().parse::<i32>().unwrap();

            let delta: (i32, i32) = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            for _ in 0..num_steps {
                // move the head
                head.add(&delta);
                // move the tail
                let distance_vec = head.difference(&tail);
                match distance_vec {
                    (x, y) if x.abs() < 2 && y.abs() < 2 => {} // they touch, do nothing
                    (dx, 0) => tail.add(&(dx.signum(), 0)),
                    (0, dy) => tail.add(&(0, dy.signum())),
                    (dx, dy) => tail.add(&(dx.signum(), dy.signum())),
                }

                visited_positions.insert(tail.clone());
            }
        }

        let answer = visited_positions.len();
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut visited_positions: HashSet<Position> = HashSet::new();
        let mut knots: Vec<Position> = (0..10).map(|_| Position::from_usizes(0, 0)).collect();

        visited_positions.insert(knots.last().unwrap().clone());

        for line in input.lines() {
            let mut parts = line.split_whitespace();
            let direction = parts.next().unwrap();
            let num_steps = parts.next().unwrap().parse::<i32>().unwrap();

            let delta: (i32, i32) = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            for _ in 0..num_steps {
                print_rope(&knots);
                knots[0].add(&delta);
                for i in 1..knots.len() {
                    let distance_vec = knots[i - 1].difference(&knots[i]);
                    match distance_vec {
                        (x, y) if x.abs() < 2 && y.abs() < 2 => {} // they touch, do nothing
                        (dx, 0) => knots[i].add(&(dx.signum(), 0)),
                        (0, dy) => knots[i].add(&(0, dy.signum())),
                        (dx, dy) => knots[i].add(&(dx.signum(), dy.signum())),
                    }
                }
                visited_positions.insert(knots.last().unwrap().clone());
            }
        }
        print_rope(&knots);

        let answer = visited_positions.len();
        Ok(format!("{}", answer))
    }
}

fn print_rope(knots: &Vec<Position>) {
    for pos in knots.iter() {
        print!("({}, {}), ", pos.x, pos.y);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
