#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use regex::Regex;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");
static EXAMPLE_2: &str = include_str!("example2.txt");

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LOWERCASE_RE: Regex = Regex::new(r"[a-z]+").unwrap();
}

use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    id: String,
    neighbors: Vec<Rc<RefCell<Node>>>,
    visited: bool,
}

impl Node {
    fn new(id: String) -> Self {
        Self {
            id,
            neighbors: vec![],
            visited: false,
        }
    }
}

/// explores the graph. mutates nodes to mark them as visited
fn explore(curr: Rc<RefCell<Node>>) -> i32 {

    if &curr.borrow().id[..] == "end" {
        // println!("{}", curr.borrow().id);
        return 1
    }
    if curr.borrow().visited {
        return 0
    }

    // println!("{}", curr.borrow().id);
    // set visited
    if LOWERCASE_RE.is_match(&curr.borrow().id[..]) {
        curr.borrow_mut().visited = true;
    }

    // recurse
    // be careful not to borrow curr in the outside scope, or else nested calls will not be able to
    // borrow mut
    let neighbors = curr.borrow().neighbors.clone();
    let mut sum = 0;
    for neighbor in neighbors {
        sum += explore(neighbor)
    }

    // set unvisited
    curr.borrow_mut().visited = false;
    sum
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut hm: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    // create nodes
    for line in input.lines() {
        for token in line.split("-") {
            if !hm.contains_key(token) {
                hm.insert(token, Rc::new(RefCell::new(Node::new(token.to_string()))));
            }
        }
    }
    // add neighbors
    for line in input.lines() {
        let keys: Vec<&str> = line.split("-").collect();
        for (i, j) in [(0, 1), (1, 0)] {
            let n1 = hm.get(keys[i]).unwrap();
            let n2 = hm.get(keys[j]).unwrap();
            n1.borrow_mut().neighbors.push(Rc::clone(n2));
        }
    }

    let start_node = Rc::clone(hm.get("start").unwrap());
    drop(hm);

    Ok(explore(start_node))
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut hm: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    // create nodes
    for line in input.lines() {
        for token in line.split("-") {
            if !hm.contains_key(token) {
                hm.insert(token, Rc::new(RefCell::new(Node::new(token.to_string()))));
            }
        }
    }
    // add neighbors
    for line in input.lines() {
        let keys: Vec<&str> = line.split("-").collect();
        for (i, j) in [(0, 1), (1, 0)] {
            let n1 = hm.get(keys[i]).unwrap();
            let n2 = hm.get(keys[j]).unwrap();
            n1.borrow_mut().neighbors.push(Rc::clone(n2));
        }
    }

    let start_node = Rc::clone(hm.get("start").unwrap());
    drop(hm);

    Ok(explore(start_node))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
