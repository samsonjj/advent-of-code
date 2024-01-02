#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use itertools::Itertools;
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

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Cube {
    RED,
    GREEN,
    BLUE,
}

#[derive(Debug)]
struct Game {
    id: i32,
    cube_sets: Vec<Vec<(Cube, i32)>>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(':').collect_vec();

            let game_id = parts[0]
                .split(' ')
                .skip(1)
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let cube_sets = parts[1]
                .split(';')
                .map(|trial| {
                    trial[1..]
                        .split(", ")
                        .map(|cube_str| {
                            let mut iter = cube_str.split(' ');

                            let count = iter.next().unwrap().parse::<i32>().unwrap();
                            let color = match iter.next().unwrap() {
                                "red" => Cube::RED,
                                "green" => Cube::GREEN,
                                "blue" => Cube::BLUE,
                                _ => unreachable!(),
                            };
                            (color, count)
                        })
                        .collect_vec()
                })
                .collect_vec();

            Game {
                id: game_id,
                cube_sets,
            }
        })
        .collect_vec()
}

fn game_is_possible(game: &Game) -> bool {
    for cube_set in game.cube_sets.iter() {
        for (cube, count) in cube_set.iter() {
            let available = match cube {
                Cube::RED => 12,
                Cube::GREEN => 13,
                Cube::BLUE => 14,
            };
            if count > &available {
                return false;
            }
        }
    }
    true
}

fn power(game: &Game) -> i32 {
    let mut hm = HashMap::new();
    hm.insert(Cube::RED, 0);
    hm.insert(Cube::GREEN, 0);
    hm.insert(Cube::BLUE, 0);

    for cube_set in game.cube_sets.iter() {
        for (cube, count) in cube_set.iter() {
            if hm[&cube] < *count {
                hm.insert(*cube, *count);
            }
        }
    }

    hm[&Cube::RED] * hm[&Cube::GREEN] * hm[&Cube::BLUE]
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let games = parse_input(input);
        let answer: i32 = games
            .iter()
            .filter(|game| game_is_possible(&game))
            .map(|game| game.id)
            .sum();

        Ok(format!("{answer}"))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let games = parse_input(input);
        let answer: i32 = games.iter().map(|game| power(game)).sum();
        Ok(format!("{answer}"))
    }
}
