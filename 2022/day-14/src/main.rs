#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct Point(i32, i32);

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct Vector(i32, i32);

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct World {
    /// for purposes of this problem, any sand that comes to rest
    /// will also get added to this hash set.
    rock: HashSet<Point>,
    source: Point,
    sand: Option<Point>,
    lowest_rock: i32,
    initial_rock_count: i32,
}

fn points_between(mut a: Point, b: Point) -> Vec<Point> {
    let mut points = vec![];

    // this implementation works for lines that go in any cardinal direction
    let dir = if a.0 == b.0 {
        Vector(0, (b.1 - a.1).signum())
    } else {
        Vector((b.0 - a.0).signum(), 0)
    };
    points.push(a);

    while a != b {
        a = a + dir;
        points.push(a);
    }
    points
}

impl World {
    /// returns whether the world is done simulating
    fn iterate_once(&mut self, use_floor: bool) -> bool {
        // if sand is None, try to spawn sand, then return
        if self.sand == None {
            if self.rock.contains(&self.source) {
                return true;
            }
            self.sand = Some(self.source);
        }

        if use_floor && self.sand.unwrap().1 >= self.lowest_rock + 1 {
            self.rock.insert(self.sand.unwrap());
            self.sand = None;
            return false;
        }

        if !use_floor && self.sand.unwrap().1 >= self.lowest_rock {
            return true;
        }

        // let the sand fall
        let below = self.sand.unwrap() + Vector(0, 1);
        if !self.rock.contains(&below) {
            self.sand = Some(below);
            return false;
        }

        let below = self.sand.unwrap() + Vector(-1, 1);
        if !self.rock.contains(&below) {
            self.sand = Some(below);
            return false;
        }

        let below = self.sand.unwrap() + Vector(1, 1);
        if !self.rock.contains(&below) {
            self.sand = Some(below);
            return false;
        }

        // sand comes to rest
        self.rock.insert(self.sand.unwrap());
        self.sand = None;
        false
    }
    fn new(input: &str) -> Self {
        let mut result = Self {
            rock: HashSet::new(),
            source: Point(500, 0),
            sand: None,
            lowest_rock: i32::MIN,
            initial_rock_count: 0,
        };
        for line in input.lines() {
            let mut points = line.split("->").map(|point| {
                let mut coords = point.split(",").map(|coord| coord.trim());
                let x = coords.next().unwrap().parse::<i32>().unwrap();
                let y = coords.next().unwrap().parse::<i32>().unwrap();
                Point(x, y)
            });
            let mut last = points.next().unwrap();
            for current in points {
                for point in points_between(last, current) {
                    result.rock.insert(point);
                    // TODO make sure this works
                    result.lowest_rock = std::cmp::max(result.lowest_rock, point.1);
                    last = current;
                }
            }
        }
        result.initial_rock_count = result.rock.len() as i32;
        result
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut world = World::new(input);
        while !world.iterate_once(false) {}

        Ok(format!(
            "{}",
            world.rock.len() as i32 - world.initial_rock_count
        ))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut world = World::new(input);
        while !world.iterate_once(true) {}

        Ok(format!(
            "{}",
            world.rock.len() as i32 - world.initial_rock_count
        ))
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
