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

type Grid = HashMap<(i32, i32), State>;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Carrier {
    pos: (i32, i32),
    dir: Direction,
    infection_count: i32,
}

impl Carrier {
    fn new(middle: (i32, i32)) -> Self {
        Self {
            pos: middle,
            dir: Direction::Up,
            infection_count: 0,
        }
    }
    fn iterate(&mut self, grid: &mut Grid) {
        let state = grid.entry(self.pos).or_insert(State::Clean);
        if *state == State::Infected {
            self.dir = match self.dir {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
            };
            *state = State::Clean;
        } else {
            self.dir = match self.dir {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            };
            *state = State::Infected;
            self.infection_count += 1;
        }

        match self.dir {
            Direction::Up => self.pos = (self.pos.0, self.pos.1 + 1),
            Direction::Down => self.pos = (self.pos.0, self.pos.1 - 1),
            Direction::Left => self.pos = (self.pos.0 - 1, self.pos.1),
            Direction::Right => self.pos = (self.pos.0 + 1, self.pos.1),
        }
    }
    fn iterate_part_2(&mut self, grid: &mut Grid) {
        let state = grid.entry(self.pos).or_insert(State::Clean);
        match state {
            State::Infected => {
                self.dir = match self.dir {
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                };
                *state = State::Flagged;
            }
            State::Clean => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                };
                *state = State::Weakened;
            }
            State::Flagged => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                };
                *state = State::Clean;
            }
            State::Weakened => {
                *state = State::Infected;
                self.infection_count += 1;
            }
        }

        match self.dir {
            Direction::Up => self.pos = (self.pos.0, self.pos.1 + 1),
            Direction::Down => self.pos = (self.pos.0, self.pos.1 - 1),
            Direction::Left => self.pos = (self.pos.0 - 1, self.pos.1),
            Direction::Right => self.pos = (self.pos.0 + 1, self.pos.1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn parse_grid(input: &str) -> (Grid, Carrier) {
    let mut grid: Grid = HashMap::new();
    let initial_state: Vec<Vec<State>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        State::Infected
                    } else {
                        State::Clean
                    }
                })
                .collect()
        })
        .collect();
    for (y, row) in initial_state.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            grid.insert((x as i32, -(y as i32)), *val);
        }
    }
    let middle = (
        initial_state.len() as i32 / 2,
        -(initial_state.len() as i32 / 2),
    );
    let carrier = Carrier::new(middle);
    (grid, carrier)
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let (mut grid, mut carrier) = parse_grid(input);
        for _ in 0..10000 {
            carrier.iterate(&mut grid);
        }
        Ok(format!("{}", carrier.infection_count))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let (mut grid, mut carrier) = parse_grid(input);
        for _ in 0..10000000 {
            carrier.iterate_part_2(&mut grid);
        }
        Ok(format!("{}", carrier.infection_count))
    }
}
