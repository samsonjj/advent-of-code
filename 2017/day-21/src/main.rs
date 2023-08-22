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

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: vec![
                vec!['.', '#', '.'],
                vec!['.', '.', '#'],
                vec!['#', '#', '#'],
            ],
        }
    }

    fn display(&self) {
        for row in self.data.iter() {
            for col in row {
                print!("{col}");
            }
            println!();
        }
    }

    fn iterate(&mut self, rules: &Vec<Rule>) {
        let width = self.data.len();
        let chunk_size = if width % 2 == 0 { 2 } else { 3 };
        let num_chunks = width / chunk_size;
        let resulting_chunk_size = chunk_size + 1;
        let resulting_width = num_chunks * resulting_chunk_size;
        let mut result = vec![vec!['.'; resulting_width]; resulting_width];

        for i in 0..num_chunks {
            for j in 0..num_chunks {
                let y = i * chunk_size;
                let x = j * chunk_size;
                let matched_rule = rules
                    .iter()
                    .find(|rule| rule.matches(&self.data, x, y, chunk_size))
                    .unwrap();
                for k in 0..resulting_chunk_size {
                    for l in 0..resulting_chunk_size {
                        result[y + i + k][x + j + l] = matched_rule.to[k][l];
                    }
                }
            }
        }

        self.data = result;
    }
}

#[derive(Clone, Debug)]
struct Rule {
    from: Vec<Vec<char>>,
    to: Vec<Vec<char>>,
}

struct Transform {
    matrix: Vec<Vec<usize>>,
}

const rotate_3: [[(usize, usize); 3]; 3] = [
    [(0, 2), (1, 2), (2, 2)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (2, 0)],
];
const rotate_2: [[(usize, usize); 2]; 2] = [[(0, 1), (1, 1)], [(0, 0), (1, 0)]];

const flip_3: [[(usize, usize); 3]; 3] = [
    [(2, 2), (1, 2), (0, 2)],
    [(2, 1), (1, 1), (0, 1)],
    [(2, 0), (1, 0), (0, 0)],
];
const flip_2: [[(usize, usize); 2]; 2] = [[(1, 1), (0, 1)], [(1, 0), (0, 0)]];

fn flip(square: (usize, usize), width: usize) -> (usize, usize) {
    let result = if width == 2 {
        flip_2[square.0][square.1]
    } else {
        flip_3[square.0][square.1]
    };

    result
}

fn rotate(square: (usize, usize), width: usize) -> (usize, usize) {
    if width == 2 {
        rotate_2[square.0][square.1]
    } else {
        rotate_3[square.0][square.1]
    }
}

fn parse_rule_part(mut s: &str) -> Vec<Vec<char>> {
    s.split("/")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

impl Rule {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(" => ");
        let from = parse_rule_part(parts.next().unwrap());
        let to = parse_rule_part(parts.next().unwrap());
        Self { from, to }
    }

    fn matches(&self, data: &Vec<Vec<char>>, x: usize, y: usize, width: usize) -> bool {
        if self.from.len() != width {
            return false;
        }
        for rotate_count in 0..4 {
            'outer: for flip_count in 0..2 {
                for dy in 0..width {
                    for dx in 0..width {
                        let (mut dy2, mut dx2) = (dy, dx);
                        for _ in 0..rotate_count {
                            (dy2, dx2) = rotate((dy2, dx2), width);
                        }
                        for _ in 0..flip_count {
                            (dy2, dx2) = flip((dy2, dx2), width);
                        }
                        if data[y + dy2][x + dx2] != self.from[dy][dx] {
                            continue 'outer;
                        }
                    }
                }
                return true;
            }
        }
        false
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut grid = Grid::new();
        let rules: Vec<Rule> = input.lines().map(|line| Rule::parse(line)).collect();

        for i in 0..5 {
            grid.iterate(&rules);
        }

        let answer: i32 = grid
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if c == &'#' { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum();

        Ok(format!("{answer}"))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut grid = Grid::new();
        let rules: Vec<Rule> = input.lines().map(|line| Rule::parse(line)).collect();
        for i in 0..18 {
            grid.iterate(&rules);
        }
        let answer: i32 = grid
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if c == &'#' { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum();

        Ok(format!("{answer}"))
    }
}

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
