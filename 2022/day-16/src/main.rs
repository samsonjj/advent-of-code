#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

mod parse;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(EXAMPLE);
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
            dbg!(&s.borrow().label_string[..]);
            let distance = distances.get(&s.borrow().label_string[..]).unwrap() + 1;

            for edge in s.borrow().edges.iter() {
                if !visited.contains(&edge.node.borrow().index) {
                    println!(
                        "distances of {} to {} is {}",
                        edge.node.borrow().index,
                        s.borrow().index,
                        distance
                    );
                    distances.insert(edge.node.borrow().label_string.clone(), distance);
                    let index = edge.node.borrow().index;
                    matrix[index as usize][s.borrow().index as usize] = distance;
                    // matrix[s.borrow().index as usize][index as usize] = distance;

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
struct Action {
    delta_flow: i32,
    time: i32,
    node: String,
}

#[derive(Debug)]
struct Game {
    flow_rate: i32,
    steam_released: i32,
    time_left: i32,
    actions: Vec<Action>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            flow_rate: 0,
            steam_released: 0,
            time_left: 30,
            actions: vec![],
        }
    }
    // returns an i32 (amount of steam released) if time runs out
    // in that case, does not update time_left
    pub fn tick(&mut self, time: i32, delta_flow: i32, node: String) -> Option<i32> {
        if time >= self.time_left {
            // dbg!(&self);
            return Some(self.time_left * self.flow_rate + self.steam_released);
        }
        self.time_left -= time;
        self.steam_released += time * self.flow_rate;
        self.flow_rate += delta_flow;

        self.actions.push(Action {
            time,
            delta_flow,
            node,
        });

        None
    }
    pub fn rewind(&mut self) {
        let action = self.actions.pop().unwrap();
        self.time_left += action.time;

        // order of these two is important
        self.flow_rate -= action.delta_flow;
        self.steam_released -= action.time * self.flow_rate;
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        // let answer = 3;
        // let regex = Regex::new(
        //     r"^Valve ([A-Z][A-Z]) has flow rate=(\d*); tunnels lead to valves ([A-Z,]*$",
        // )
        // .unwrap();
        // let mut labelToIndexMap: HashMap<String, i32> = HashMap::new();
        // let mut edges: Vec<(String, String)> = vec![];
        // let mut nodes: HashMap<i32, RefCell<Node>> = HashMap::new();
        // let data: Vec<(String, i32, Vec<String>, usize)> = input
        //     .lines()
        //     .enumerate()
        //     .map(|(index, line)| {
        //         let caps = regex.captures(line).unwrap();
        //         let label = caps.get(1).unwrap().as_str().to_string();
        //         let flow_rate = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        //         let valves = caps
        //             .get(2)
        //             .unwrap()
        //             .as_str()
        //             .split(", ")
        //             .map(|s| String::from(s))
        //             .collect::<Vec<_>>();
        //         (label, flow_rate, valves, index)
        //     })
        //     .collect();

        // for (label, flow_rate, neighbors, index) in data.into_iter() {
        //     labelToIndexMap.insert(label, index as i32);
        //     let data = nodes.insert(
        //         index as i32,
        //         RefCell::new(Node {
        //             index: index as i32,
        //             label_string: label.clone(),
        //             edges: vec![],
        //         }),
        //     );
        //     for neighbor in neighbors.into_iter() {
        //         edges.push((label.clone(), neighbor));
        //     }
        // }
        // for edge in edges.into_iter() {
        //     let neighbor = nodes.get("");
        //     let mut data = nodes.entry(*labelToIndexMap.get(&edge.0).unwrap());
        //     *data.borrow_mut().edges.push();
        // }

        // let connection_matrix = breadth_first_search(nodes);
        // println!("{:?}", connection_matrix);
        // let answer = 5;
        // let nodes = parse::parse_input(input);

        // println!(
        //     "{:?}",
        //     nodes
        //         .get(&"AA".to_string())
        //         .unwrap()
        //         .borrow()
        //         .edges
        //         .iter()
        //         .map(|edge| edge.node.borrow().label_string.clone())
        //         .collect::<Vec<_>>()
        // );

        // let connection_matrix = breadth_first_search(&nodes);
        // println!(
        //     "AA index = {}",
        //     nodes.get(&"AA".to_string()).unwrap().borrow().index
        // );
        let graph = parse::Graph::parse_graph(input);
        let matrix = graph.connection_matrix();

        // now that we have the matrix, we just have to check the result for
        // each possible permutation of the nodes. We can ignore nodes that
        // have zero flow_rate, when calculating the permutations.

        // nodes that have flow_rate > 0
        let mut marked_nodes: HashSet<usize> = graph
            .label_to_node
            .values()
            .filter(|node| node.flow_rate > 0)
            .map(|node| node.index)
            .collect();

        dbg!(&marked_nodes);

        let mut game = Game::new();

        fn permute(
            nodes: &mut HashSet<usize>,
            current_node: usize,
            matrix: &Vec<Vec<i32>>,
            graph: &parse::Graph,
            game: &mut Game,
        ) -> (i32, Vec<Action>) {
            if nodes.len() == 0 {
                let result = game.tick(game.time_left, 0, "timeout".to_string()).unwrap();
                let actions = game.actions.clone();
                return (result, actions);
                // do calculation, idk
            }
            let nodes_list: Vec<usize> = nodes.clone().iter().map(|x| *x).collect();
            let mut max_result = 0;
            let mut max_actions = vec![];
            for node in nodes_list.iter() {
                let distance = matrix[current_node][*node];
                let delta_flow = graph.index_to_node.get(node).unwrap().flow_rate;

                nodes.remove(node);
                let result = game.tick(
                    distance + 1,
                    delta_flow,
                    graph.index_to_node.get(node).unwrap().label.clone(),
                );
                let (result, actions) = match result {
                    Some(steam_released) => (steam_released, game.actions.clone()), // game over, compare to previous results, keep max
                    None => {
                        // game not over, continue permutations
                        let x = permute(nodes, *node, matrix, graph, game);
                        game.rewind();
                        x
                    }
                };
                if result > max_result {
                    max_result = result;
                    max_actions = actions;
                }
                nodes.insert(*node);
            }

            (max_result, max_actions)
        }

        let (answer, actions) = permute(&mut marked_nodes, 0usize, &matrix, &graph, &mut game);

        dbg!(actions);

        println!("{:?}", matrix);
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

/*
 * Say there are N nodes. We can build a connection matrix (NxN) which details the distance between any two nodes.
 *
 * [0 1 4 ..]
 * [1 0 2 ..]
 * [2 1 0 ..]
 * [........]
 *
 * At any time, we must choose which node to go to next. That is the key decision.
 *
 * Decision Factors:
 * - time it takes to get there (distance)
 * - flow rate
 * - time remaining
 * - other nodes
 * - which nodes have been visited
 *
 * How maximize steam release?
 *
 * This is likely not a shortest path problem... It seems like maximization problem
 *
 * Lets try brute force!
 */
