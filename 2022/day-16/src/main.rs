#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

mod game;
mod parse;

use game::{Action, Game};
use parse::Graph;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

pub type Matrix = Vec<Vec<i32>>;

#[derive(Clone, Debug)]
pub struct Edge {
    distance: i32,
    node: Rc<RefCell<Node>>,
}

#[derive(Clone, Debug)]
pub struct Node {
    index: i32,
    label_string: String,
    edges: Vec<Edge>,
}

use regex::Regex;

fn run_elephant_game<'a>(game: &'a mut Game) -> (i32, Vec<Action>) {
    let mut elephant_game = game.get_elephant_game(26);
    permute(&mut elephant_game)
}

fn permute<'a>(game: &'a mut Game) -> (i32, Vec<Action>) {
    if game.nodes.len() == 0 {
        let mut result = game.run_the_clock();
        let mut actions = game.actions.clone();

        if game.should_use_elephant {
            let mut elephant_game: Game = game.get_elephant_game(26);
            // let (elephant_result, mut elephant_actions) = permute(&mut elephant_game);

            // result += elephant_result;
            // actions.append(&mut elephant_actions);
        }

        game.rewind();
        return (result, actions);
    }

    let nodes_list: Vec<usize> = game.nodes.clone().into_iter().map(|x| x).collect();

    let mut max_result = 0;
    let mut max_actions = vec![];

    for node_index in nodes_list.iter() {
        let result = game.goto_node(*node_index);
        let (result, actions) = match result {
            Some(steam_released) => (steam_released, game.actions.clone()), // game over, compare to previous results, keep max
            None => {
                // game not over, continue permutations
                let x = permute(game);
                x
            }
        };
        game.rewind();
        if result > max_result {
            max_result = result;
            max_actions = actions;
        }
    }

    (max_result, max_actions)
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let graph = parse::Graph::parse_graph(input);

        // only search nodes that have flow_rate > 0
        let mut marked_nodes: HashSet<usize> = graph
            .label_to_node
            .values()
            .filter(|node| node.flow_rate > 0)
            .map(|node| node.index)
            .collect();

        let start_node = graph.label_to_node.get(&"AA".to_string()).unwrap().index;

        let mut game = Game::new(graph, 30, marked_nodes, start_node, false);

        let (answer, actions) = permute(&mut game);

        dbg!(actions);
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let graph = parse::Graph::parse_graph(input);

        // only search nodes that have flow_rate > 0
        let mut marked_nodes: HashSet<usize> = graph
            .label_to_node
            .values()
            .filter(|node| node.flow_rate > 0)
            .map(|node| node.index)
            .collect();

        let start_node = graph.label_to_node.get(&"AA".to_string()).unwrap().index;

        let mut game = Game::new(graph, 26, marked_nodes, start_node, true);

        let (answer, actions) = permute(&mut game);

        dbg!(actions);
        Ok(format!("{}", answer))
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
