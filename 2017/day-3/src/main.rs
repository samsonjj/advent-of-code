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

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn delta(&self) -> Vec2 {
        match self {
            Direction::Right => Vec2(1, 0),
            Direction::Up => Vec2(0, 1),
            Direction::Left => Vec2(-1, 0),
            Direction::Down => Vec2(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(i32, i32);

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Vec2 {
    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let num = input.parse::<i32>().unwrap();

        let mut pos = Vec2(1, 0);
        let mut dir = Direction::Up;
        for i in 2..num {
            if pos.0.abs() == pos.1.abs() {
                if pos.0 > 0 && pos.1 < 0 {
                    pos = pos + dir.delta();
                    dir = dir.next();
                    continue;
                } else {
                    dir = dir.next();
                }
            }
            pos += dir.delta();
        }

        Ok(format!("{}", pos.manhattan()))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let num = input.parse::<i32>().unwrap();

        let mut pos = Vec2(1, 0);
        let mut dir = Direction::Up;
        let mut grid: HashMap<Vec2, i32> = HashMap::new();
        grid.insert(Vec2(0, 0), 1);
        grid.insert(Vec2(1, 0), 1);
        loop {
            if pos.0.abs() == pos.1.abs() {
                if pos.0 > 0 && pos.1 < 0 {
                    pos = pos + dir.delta();
                    dir = dir.next();
                } else {
                    dir = dir.next();
                    pos += dir.delta();
                }
            } else {
                pos += dir.delta();
            }

            let mut sum = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    let p = pos + Vec2(x, y);
                    if let Some(v) = grid.get(&p) {
                        sum += v;
                    }
                }
            }

            if sum > num {
                return Ok(format!("{}", sum));
            }

            dbg!(sum, pos);

            grid.insert(pos, sum);
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
