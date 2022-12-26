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

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Point(i32, i32);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Point {
    fn manhatten_distance(&self, other: Point) -> i32 {
        return (self.0 - other.0).abs() + (self.1 - other.1).abs();
    }
}

fn intersect_point_candidate(point_1: Point, d1: i32, point_2: Point, d2: i32) {
    // y = mx + b
    // y = -x +
    // p1 = (p.x, p.y - d)
    // p2 = (px + d, p.y)
    // m = ((p.y - d) - p.y) / (p.x - (p.x + d))
    // p.y = m * p.x
}

use regex::Regex;

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let given_row = 2000000;
        // let given_row = 10;
        let re = Regex::new(
            r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$",
        )
        .unwrap();

        let sensor_beacons = input.lines().map(|line| {
            dbg!(line);
            let cap = re.captures(line).unwrap();
            let sensor = Point(
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let beacon = Point(
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            (sensor, beacon)
        });

        let mut impossible_points: HashSet<i32> = HashSet::new();
        for (sensor, beacon) in sensor_beacons.clone() {
            let manhatten_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            let radius_at_given_row = manhatten_distance - (sensor.1 - given_row).abs();
            for x in sensor.0 - radius_at_given_row..=sensor.0 + radius_at_given_row {
                impossible_points.insert(x);
            }
        }
        for (_, beacon) in sensor_beacons {
            if beacon.1 == given_row {
                impossible_points.remove(&beacon.0);
            }
        }
        Ok(format!("{}", impossible_points.len()))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let re = Regex::new(
            r"^Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)$",
        )
        .unwrap();
        let sensor_beacons = input.lines().map(|line| {
            dbg!(line);
            let cap = re.captures(line).unwrap();
            let sensor = Point(
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let beacon = Point(
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            (sensor, beacon)
        });
        let mut sensors: Vec<Sensor> = vec![];
        let mut beacons = HashSet::new();
        for (sensor, beacon) in sensor_beacons {
            sensors.push(Sensor {
                location: sensor,
                radius: sensor.manhatten_distance(beacon),
            });
            beacons.insert(beacon);
        }

        let mut lines: Vec<Line> = vec![];
        for i in 0..sensors.len() {
            for j in i + 1..sensors.len() {
                if let Some(line) = Line::from_sensors(&sensors[i], &sensors[j]) {
                    lines.push(line);
                }
            }
        }

        let mut points: Vec<Point> = vec![];
        for i in 0..lines.len() {
            for j in i..lines.len() {
                if let Some(point) = lines[i].intersect(&lines[j]) {
                    points.push(point);
                }
            }
        }

        for point in points {
            if point.0 < 0 || point.0 > 4_000_000 || point.1 < 0 || point.1 > 4_000_000 {
                continue;
            }
            if beacons.contains(&point) {
                continue;
            }
            if sensors
                .iter()
                .any(|sensor| sensor.location.manhatten_distance(point) <= sensor.radius)
            {
                continue;
            }
            return Ok(format!("{}", point.0 * 4_000_000 + point.1));
        }
        let answer = 5;
        Ok(format!("{}", answer))
    }
}

struct Sensor {
    location: Point,
    radius: i32,
}

impl Sensor {
    fn manhattan_distance(&self, other: &Sensor) -> i32 {
        return self.location.manhatten_distance(other.location);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    m: i32,
    b: i32,
}

impl Line {
    /// a "magic line" is the line in between two sensors, on which a beacon could exist.
    /// The line is "magic" because it is the magic distance between both sensors. i.e. if
    /// sensor 1 has a radius of d1, and sensor 2 has a radius of d2, the line represents
    /// the points which are a distance of d1 + 1 from sensor 1, and d2 + 1 from sensor 2.
    /// If there is no magic line (i.e. the sensors are not the perfect distane apart,
    /// which is d1 + d2 + 1) then None is returned.
    fn from_sensors(s1: &Sensor, s2: &Sensor) -> Option<Line> {
        let d = s1.manhattan_distance(&s2);
        if d == 0 {
            return None;
        }
        dbg!(s1.manhattan_distance(&s2), s1.radius, s2.radius);
        // there needs to be two extra distance, so that the middle line is
        // 1 away from both prohibited zones.
        if s1.manhattan_distance(&s2) == s1.radius + s2.radius + 2 {
            let dx = s2.location.0 - s1.location.0;
            let dy = s2.location.1 - s1.location.1;

            let dx_unit = dx.signum();
            let dy_unit = dy.signum();

            // this may seem opposite, but that's because the middle line is
            // perpendicular to the line between the two points
            let m = if dx_unit == dy_unit { -1 } else { 1 };

            let sample_point = s1.location + Point(dx_unit, 0) * (s1.radius + 1);
            dbg!(sample_point);

            let b = sample_point.1 - (m * sample_point.0);

            Some(Line { m, b })
        } else {
            None
        }
    }

    fn intersect(&self, other: &Line) -> Option<Point> {
        if self.m == other.m {
            return None;
        }
        let x = (other.b - self.b) / (self.m - other.m);
        let y = self.m * x + self.b;
        Some(Point(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_none_calc_magic_line() {
        let s1 = Sensor {
            location: Point(2, 18),
            radius: 7,
        };
        let s2 = Sensor {
            location: Point(9, 16),
            radius: 2,
        };
        let line = Line::from_sensors(&s1, &s2);
        match line {
            Some(_) => assert!(false),
            None => {} // good
        }

        let s1 = Sensor {
            location: Point(2, 18),
            radius: 4,
        };
        let s2 = Sensor {
            location: Point(9, 16),
            radius: 2,
        };
        let line = Line::from_sensors(&s1, &s2);
        match line {
            Some(_) => assert!(false),
            None => {} // good
        }
    }

    #[test]
    fn test_some_calc_magic_line() {
        let s1 = Sensor {
            location: Point(2, 18),
            radius: 5,
        };
        let s2 = Sensor {
            location: Point(9, 16),
            radius: 2,
        };
        let line = Line::from_sensors(&s1, &s2);
        dbg!(line);
        match line {
            Some(line) => assert_eq!(line, Line { m: 1, b: 10 }),
            None => assert!(false), // good
        }
    }

    #[test]
    fn test_intersect() {
        let line_1 = Line { m: 1, b: 10 };
        let line_2 = Line { m: -1, b: 16 };

        let intersect = line_1.intersect(&line_2).unwrap();
        assert_eq!(intersect, Point(3, 13));
    }
}
