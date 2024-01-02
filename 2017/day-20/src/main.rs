#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ffi::FromVecWithNulError;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3(i32, i32, i32);

impl Point3 {
    fn manhatten(&self) -> i32 {
        return self.0.abs() + self.1.abs() + self.2.abs();
    }
}

impl std::ops::Add for Point3 {
    type Output = Point3;

    fn add(self, rhs: Self) -> Self::Output {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Mul<i32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div<i32> for Point3 {
    type Output = Point3;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Particle {
    position: Point3,
    velocity: Point3,
    acceleration: Point3,
}

impl Particle {
    fn pos_at(&self, time: i32) -> Point3 {
        // v=2, a=3, t=3 -> dp = 24
        self.position
            + ((self.velocity + self.acceleration) + (self.velocity + (self.acceleration * time)))
                * time
                / 2
    }
}

fn parse_point_3(s: &str) -> Point3 {
    let mut parts = s.split(",");
    Point3(
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let regex_pattern = regex::Regex::new(r"p=<(.*)>, v=<(.*)>, a=<(.*)>")?;
        let particles: Vec<Particle> = input
            .lines()
            .map(|line| {
                let captures = regex_pattern.captures(line).unwrap();
                Particle {
                    position: parse_point_3(captures.get(1).unwrap().as_str()),
                    velocity: parse_point_3(captures.get(2).unwrap().as_str()),
                    acceleration: parse_point_3(captures.get(3).unwrap().as_str()),
                }
            })
            .collect();

        // particles.iter().for_each(|particle| {
        //     dbg!(particle, particle.pos_at(1_000_000));
        // });

        let min_particle = particles
            .iter()
            .enumerate()
            .min_by(|p1, p2| {
                p1.1.acceleration
                    .manhatten()
                    .cmp(&p2.1.acceleration.manhatten())
            })
            .unwrap();

        Ok(format!("{}", min_particle.0))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let regex_pattern = regex::Regex::new(r"p=<(.*)>, v=<(.*)>, a=<(.*)>")?;
        let mut particles: HashMap<usize, Particle> = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let captures = regex_pattern.captures(line).unwrap();
                (
                    i,
                    Particle {
                        position: parse_point_3(captures.get(1).unwrap().as_str()),
                        velocity: parse_point_3(captures.get(2).unwrap().as_str()),
                        acceleration: parse_point_3(captures.get(3).unwrap().as_str()),
                    },
                )
            })
            .collect();

        for i in 0..1000 {
            let mut occupied = HashMap::with_capacity(particles.len());
            for (i, particle) in particles.iter_mut() {
                particle.velocity = particle.acceleration + particle.velocity;
                particle.position = particle.position + particle.velocity;
                *occupied.entry(particle.position).or_insert(0) += 1;
            }

            particles.retain(|i, particle| occupied[&particle.position] <= 1);

            // let to_remove: Vec<usize> = particles
            //     .iter()
            //     .filter(|(_, particle)| occupied[&particle.position] > 1)
            //     .map(|(i, _)| *i)
            //     .collect();

            // for i in to_remove {
            //     particles.remove(&i);
            // }
        }

        Ok(format!("{}", particles.len()))
    }
}
