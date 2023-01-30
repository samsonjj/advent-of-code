use regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::Node;

pub fn capture_input(input: &str) {}

pub struct Graph {
    pub edges: Vec<HashSet<usize>>,
    pub label_to_node: HashMap<String, NodeValue>,
    pub index_to_node: HashMap<usize, NodeValue>,
}

impl Graph {
    pub fn parse_graph(input: &str) -> Self {
        let input_items = parse_input(input);

        let mut index_to_node: HashMap<usize, NodeValue> = HashMap::new();
        let mut nodes: HashMap<String, NodeValue> = HashMap::new();
        for (i, item) in input_items.iter().enumerate() {
            nodes.insert(
                item.label.clone(),
                NodeValue {
                    index: i,
                    flow_rate: item.flow_rate,
                    label: item.label.clone(),
                },
            );
            index_to_node.insert(
                i,
                NodeValue {
                    index: i,
                    flow_rate: item.flow_rate,
                    label: item.label.clone(),
                },
            );
        }

        let mut edges = vec![HashSet::new(); nodes.len()];
        for item in input_items.iter() {
            let left = nodes.get(&item.label).unwrap().index;
            for neighbor in item.neighbors.iter() {
                let right = nodes.get(neighbor).unwrap().index;
                edges[left].insert(right);
            }
        }

        Self {
            edges: edges,
            label_to_node: nodes,
            index_to_node,
        }
    }

    pub fn index(&self, label: &str) {
        self.label_to_node.get(&label.to_string()).unwrap();
    }

    pub fn connection_matrix(&self) -> Vec<Vec<i32>> {
        // 1) Repeat for each node:
        // 2) bfs the node
        // 3) mark the distance to each in the connection matrix
        let mut matrix = vec![vec![0; self.label_to_node.len()]; self.label_to_node.len()];
        fn bfs(node: usize, edges: &Vec<HashSet<usize>>, matrix: &mut Vec<Vec<i32>>) {
            let mut visited = HashSet::new();
            visited.insert(node);
            let mut queue = vec![node];
            while queue.len() > 0 {
                let s = queue.pop().unwrap();
                for r in edges.get(s).unwrap() {
                    if !visited.contains(r) {
                        if node == 3 && *r == 9 {
                            dbg!(&matrix, s);
                        }
                        matrix[node][*r] = matrix[node][s] + 1;
                        queue.push(*r);
                        visited.insert(*r);
                    }
                }
            }
        }
        for node in self.label_to_node.values() {
            bfs(node.index, &self.edges, &mut matrix);
        }
        matrix
    }
}

pub struct NodeValue {
    pub flow_rate: i32,
    pub index: usize,
    pub label: String,
}

pub struct InputItem {
    label: String,
    flow_rate: i32,
    neighbors: Vec<String>,
}

pub fn parse_input(input: &str) -> Vec<InputItem> {
    let pattern = "^Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)$";
    let re = regex::Regex::new(pattern).unwrap();
    let mut input_items = vec![];
    for (i, line) in input.lines().enumerate() {
        let captures = re.captures(line).unwrap();
        let label = captures.get(1).unwrap().as_str().to_string();
        let flow_rate = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let mut neighbors = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        input_items.push(InputItem {
            flow_rate,
            neighbors,
            label,
        });
    }

    input_items
}
