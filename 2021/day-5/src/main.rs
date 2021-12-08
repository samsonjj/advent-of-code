#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.abs();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',');
        Ok(Self {
            x: iter.next().unwrap().parse::<i32>()?,
            y: iter.next().unwrap().parse::<i32>()?,
        })
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn iter(&self) -> LineSegmentIterator {
        LineSegmentIterator::new(self)
    }
}

impl FromStr for LineSegment {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        Ok(LineSegment {
            start: parts.next().unwrap().parse()?,
            end: parts.next().unwrap().parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct LineSegmentIterator<'a> {
    line_segment: &'a LineSegment,
    slope: Point,
    current_point: Point,
}

impl<'a> LineSegmentIterator<'a> {
    pub fn new(line_segment: &'a LineSegment) -> Self {
        let slope = Point {
            x: line_segment.end.x - line_segment.start.x,
            y: line_segment.end.y - line_segment.start.y,
        };
        let gcd_val = gcd(slope.x, slope.y);
        Self {
            line_segment,
            slope: Point {
                x: slope.x / gcd_val,
                y: slope.y / gcd_val,
            },
            current_point: line_segment.start,
        }
    }
}

impl<'a> Iterator for LineSegmentIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point == self.line_segment.end + self.slope {
            None
        } else {
            self.current_point = self.current_point + self.slope;
            Some(self.current_point - self.slope)
        }
    }
}

fn part_1(input: &str) -> AocResult<i32> {
    let line_segments = input
        .trim()
        .lines()
        .map(LineSegment::from_str)
        .map(|seg| seg.unwrap());
    let mut point_map = HashMap::new();
    for seg in line_segments {
        if seg.start.x != seg.end.x && seg.start.y != seg.end.y {
            continue;
        }
        for point in seg.iter() {
            let data = point_map.entry(point).or_insert(0);
            *data += 1;
        }
    }
    let mut sum = 0;
    for (point, count) in point_map.iter() {
        if count >= &2 {
            sum += 1;
        }
    }
    Ok(sum)
}

fn part_2(input: &str) -> AocResult<i32> {
    let line_segments = input
        .trim()
        .lines()
        .map(LineSegment::from_str)
        .map(|seg| seg.unwrap());
    let mut point_map = HashMap::new();
    for seg in line_segments {
        for point in seg.iter() {
            let data = point_map.entry(point).or_insert(0);
            *data += 1;
        }
    }
    let mut sum = 0;
    for (point, count) in point_map.iter() {
        if count >= &2 {
            sum += 1;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    #[test]
    fn line_segment_iterator() {
        let line_segment = LineSegment {
            start: Point { x: 8, y: 5 },
            end: Point { x: 2, y: 2 },
        };
        let points = line_segment.iter().collect::<Vec<Point>>();
        assert_eq!(
            points,
            vec![
                Point { x: 8, y: 5 },
                Point { x: 6, y: 4 },
                Point { x: 4, y: 3 },
                Point { x: 2, y: 2 },
            ]
        );
    }
}
