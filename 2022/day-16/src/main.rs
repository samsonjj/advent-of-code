#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

mod parse;

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

/// calculates the connection matrix for the graph
fn breadth_first_search(nodes: &HashMap<String, Rc<RefCell<Node>>>) -> Matrix {
    let mut connection_matrix = vec![vec![0; nodes.len()]; nodes.len()];

    fn bfs(node: Rc<RefCell<Node>>, matrix: &mut Vec<Vec<i32>>) {
        let mut visited: HashSet<i32> = HashSet::new();

        let mut queue: Vec<Rc<RefCell<Node>>> = vec![];
        queue.push(Rc::clone(&node));

        let mut distances = HashMap::new();
        distances.insert(node.borrow().label_string.clone(), 0);
        while queue.len() > 0 {
            let s = queue.pop().unwrap();
            let distance = distances.get(&s.borrow().label_string[..]).unwrap() + 1;

            for edge in s.borrow().edges.iter() {
                if !visited.contains(&edge.node.borrow().index) {
                    distances.insert(edge.node.borrow().label_string.clone(), distance);
                    let index = edge.node.borrow().index;
                    matrix[index as usize][s.borrow().index as usize] = distance;

                    queue.push(Rc::clone(&edge.node));
                    visited.insert(index);
                }
            }
        }
    }

    for (label, node) in nodes.iter() {
        bfs(Rc::clone(node), &mut connection_matrix);
    }

    connection_matrix
}

use regex::Regex;

#[derive(Clone, Debug)]
enum Action {
    Move {
        delta_flow: i32,
        time: i32,
        from_node_index: usize,
        node_index: usize,
        label: String,
    },
    SpinClock {
        time: i32,
    },
    SpinClock2 {
        time: i32,
    },
}

#[derive(Debug)]
struct Game {
    graph: Graph,
    nodes: HashSet<usize>,
    current_node: usize,
    flow_rate: i32,
    steam_released: i32,
    time_left: i32,
    actions: Vec<Action>,
}

impl Game {
    pub fn new(graph: Graph, nodes: HashSet<usize>, start_node: usize) -> Self {
        Game {
            graph,
            nodes,
            current_node: start_node,
            flow_rate: 0,
            steam_released: 0,
            time_left: 30,
            actions: vec![],
        }
    }

    pub fn goto_node(&mut self, node_index: usize) -> Option<i32> {
        let time = self.graph.distance(self.current_node, node_index) + 1;

        if time >= self.time_left {
            // dbg!(self.steam_released);
            self.actions.push(Action::SpinClock {
                time: self.time_left,
            });
            self.tick(self.time_left, 0);
            Some(self.steam_released)
        } else {
            let delta_flow = self.graph.node_by_index(node_index).flow_rate;
            self.actions.push(Action::Move {
                time,
                delta_flow,
                from_node_index: self.current_node,
                node_index,
                label: self.graph.node_by_index(node_index).label.clone(),
            });
            self.travel(node_index);
            self.tick(time, delta_flow);
            None
        }
    }

    fn travel(&mut self, node_index: usize) {
        self.current_node = node_index;
        self.nodes.remove(&node_index);
    }

    fn untravel(&mut self, node_index: usize) {
        self.nodes.insert(self.current_node);
        self.current_node = node_index;
    }

    pub fn tick(&mut self, time: i32, delta_flow: i32) {
        self.time_left -= time;
        self.steam_released += time * self.flow_rate;
        self.flow_rate += delta_flow;
    }

    pub fn untick(&mut self, time: i32, delta_flow: i32) {
        self.flow_rate -= delta_flow;
        self.steam_released -= time * self.flow_rate;
        self.time_left += time;
    }

    /// waste the rest of the time
    pub fn run_the_clock(&mut self) -> i32 {
        self.actions.push(Action::SpinClock2 {
            time: self.time_left,
        });
        self.tick(self.time_left, 0);
        self.steam_released
    }

    pub fn rewind(&mut self) {
        // dbg!(&self.actions);
        let action = self.actions.pop().unwrap();
        match action {
            Action::SpinClock { time } => {
                self.untick(time, 0);
            }
            Action::SpinClock2 { time } => {
                self.untick(time, 0);
            }
            Action::Move {
                delta_flow,
                time,
                from_node_index,
                node_index,
                label,
            } => {
                self.untick(time, delta_flow);
                self.untravel(from_node_index);
            }
        }
    }
}

fn permute(game: &mut Game) -> (i32, Vec<Action>) {
    if game.nodes.len() == 0 {
        let result = game.run_the_clock();
        let actions = game.actions.clone();
        // dbg!(game.steam_released);
        game.rewind();
        // dbg!(game.steam_released);
        return (result, actions);
    }

    let nodes_list: Vec<usize> = game.nodes.clone().iter().map(|x| *x).collect();

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
        let marked_nodes: HashSet<usize> = graph
            .label_to_node
            .values()
            .filter(|node| node.flow_rate > 0)
            .map(|node| node.index)
            .collect();

        let start_node = graph.label_to_node.get(&"AA".to_string()).unwrap().index;

        dbg!(start_node);
        let mut game = Game::new(graph, marked_nodes, start_node);

        let (answer, actions) = permute(&mut game);

        dbg!(actions);
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 5;
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
