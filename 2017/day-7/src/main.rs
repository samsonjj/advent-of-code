#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;
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

struct Node {
    name: String,
    weight: i32,
    children: Vec<String>,
    parent: String,
}

type Label = Rc<str>;

#[derive(Debug)]
struct Graph {
    child_to_parent: HashMap<Label, Label>,
    parent_to_children: HashMap<Label, Vec<Label>>,
    weights: HashMap<Label, i32>,
}

impl Graph {
    fn from_string(input: &str) -> Self {
        let mut child_to_parent = HashMap::new();
        let mut parent_to_children = HashMap::new();
        let mut weights = HashMap::new();

        let mut lookup = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(" -> ");
            let mut name_weight = parts.next().unwrap().split_whitespace();
            let name = name_weight.next().unwrap();
            let weight = name_weight.next().unwrap();
            let weight = weight[1..weight.len() - 1].parse::<i32>().unwrap();
            let children: Vec<&str> = parts
                .next()
                .unwrap_or("")
                .split(", ")
                .filter(|c| *c != "")
                .collect();

            let name: Rc<str> = Rc::clone(lookup.entry(name).or_insert(Rc::from(name)));
            let children = children
                .iter()
                .map(|s| Rc::clone(lookup.entry(s).or_insert(Rc::from(*s))))
                .collect::<Vec<Rc<str>>>();

            let parent_to_children_entry = parent_to_children.entry(name.clone()).or_insert(vec![]);

            weights.insert(name.clone(), weight);
            for child in children {
                child_to_parent.insert(child.clone(), name.clone());

                parent_to_children_entry.push(child.clone());
            }
        }

        Graph {
            child_to_parent,
            parent_to_children,
            weights,
        }
    }

    fn compute_weight(&self, node: impl AsRef<str>) -> i32 {
        dbg!(node.as_ref());
        let mut weight = self.weights.get(node.as_ref()).unwrap().clone();
        for child in self.children_of(node.as_ref()) {
            weight += self.compute_weight(child.as_ref());
        }
        weight
    }

    fn is_balanced(&self, node: impl AsRef<str>) -> bool {
        let mut weights = vec![];
        for child in self.children_of(node.as_ref()) {
            weights.push(self.compute_weight(child.as_ref()));
        }
        dbg!(node.as_ref(), &weights);
        let mut iter = weights.iter();
        let first = iter.next().unwrap();
        for weight in iter {
            if weight != first {
                return false;
            }
        }
        true
    }

    fn find_unbalanced(&self, node: impl AsRef<str>) -> Option<&Label> {
        for child in self.children_of(node.as_ref()) {
            if !self.is_balanced(child.as_ref()) {
                return Some(child);
            }
        }

        None
    }

    fn children_of(&self, x: impl AsRef<str>) -> &Vec<Label> {
        self.parent_to_children.get(x.as_ref()).unwrap()
    }

    fn parent_of(&self, x: impl AsRef<str>) -> Option<&Label> {
        self.child_to_parent.get(x.as_ref())
    }

    fn get_bottom(&self) -> &Label {
        let mut current = self.parent_to_children.keys().next().unwrap();
        while let Some(parent) = self.parent_of(current.as_ref()) {
            current = parent;
        }
        current
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let graph = Graph::from_string(input);

        Ok(format!("{}", graph.get_bottom()))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let answer = 5;
        let graph = Graph::from_string(input);
        dbg!(&graph);
        let mut node = graph.get_bottom();
        dbg!(&node);
        while let Some(child) = graph.find_unbalanced(node) {
            node = child;
            dbg!(&node);
        }

        // let mut correct_sibling = None;
        // while let None = {
        //     // fancy do-while
        //     let parent = graph.parent_of(node).unwrap();
        //     let siblings = graph.children_of(node);
        //     node = parent;
        //     correct_sibling = siblings.iter().find(|s| *s != node);
        //     correct_sibling
        // } {}

        Ok(format!(
            "{} - {:?}",
            node,
            graph
                .children_of(node)
                .iter()
                .map(|c| (
                    graph.weights.get(c.as_ref()),
                    graph.compute_weight(c.as_ref())
                ))
                .collect::<Vec<_>>()
        ))
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
