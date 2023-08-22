#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
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

#[derive(Clone, Copy, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

impl std::ops::Add<Velocity> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Velocity) -> Self::Output {
        match rhs {
            Velocity::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Velocity::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Velocity::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Velocity::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    Horizontal,
    Vertical,
    Intersection,
    Empty,
    Letter(char),
}

#[derive(Copy, Clone, Debug)]
enum Velocity {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Diagram {
    board: Vec<Vec<Path>>,
    seen_letters: Vec<char>,
    position: Point2,
    velocity: Velocity,
    step_count: i32,
}

impl Diagram {
    fn new(input: &str) -> Self {
        let board: Vec<Vec<Path>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '-' => Path::Horizontal,
                        '|' => Path::Vertical,
                        ' ' => Path::Empty,
                        '+' => Path::Intersection,
                        c => Path::Letter(c),
                    })
                    .collect::<Vec<Path>>()
            })
            .collect();

        let entry_point = board[0]
            .iter()
            .enumerate()
            .find_map(|(x, p)| {
                if let Path::Vertical = p {
                    Some(x)
                } else {
                    None
                }
            })
            .unwrap();

        Self {
            board,
            seen_letters: vec![],
            position: Point2 {
                x: entry_point as i32,
                y: 0,
            },
            velocity: Velocity::Down,
            step_count: 1, // count the initial step into frame
        }
    }

    fn go(&mut self) -> bool {
        if let Path::Empty = self.at(self.position + self.velocity) {
            match self.velocity {
                Velocity::Up | Velocity::Down => {
                    if self.at(self.position + Velocity::Left) != Path::Empty {
                        self.velocity = Velocity::Left
                    } else if self.at(self.position + Velocity::Right) != Path::Empty {
                        self.velocity = Velocity::Right
                    } else {
                        return true;
                    }
                }
                Velocity::Left | Velocity::Right => {
                    if self.at(self.position + Velocity::Up) != Path::Empty {
                        self.velocity = Velocity::Up
                    } else if self.at(self.position + Velocity::Down) != Path::Empty {
                        self.velocity = Velocity::Down
                    } else {
                        return true;
                    }
                }
            }
        }

        self.position = self.position + self.velocity;
        self.step_count += 1;

        if let Path::Letter(c) = self.at(self.position) {
            self.seen_letters.push(c);
        }

        false
    }

    fn at(&self, p: Point2) -> Path {
        if p.x < 0 || p.x >= self.board[0].len() as i32 || p.y < 0 || p.y >= self.board.len() as i32
        {
            Path::Empty
        } else {
            self.board[p.y as usize][p.x as usize]
        }
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut diagram = Diagram::new(input);
        while !diagram.go() {}
        Ok(format!(
            "{}",
            diagram.seen_letters.iter().collect::<String>()
        ))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut diagram = Diagram::new(input);
        while !diagram.go() {}
        Ok(format!("{}", diagram.step_count))
    }
}
