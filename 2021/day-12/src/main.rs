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
use std::cell::{Ref, RefCell};

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

fn explore(curr: Rc<RefCell<Node>>, mut allow_one_repeat: bool) -> i32 {
    if &curr.borrow().id[..] == "end" {
        return 1
    }

    // handle repeat
    let mut is_repeat = false;
    if curr.borrow().visited {
        if allow_one_repeat && curr.borrow().id != "start" {
            allow_one_repeat = false;
            is_repeat = true;
        } else {
            return 0
        }
    }

    // set visited
    if LOWERCASE_RE.is_match(&curr.borrow().id[..]) {
        curr.borrow_mut().visited = true;
    }

    // recurse
    let neighbors = curr.borrow().neighbors.clone();
    let mut sum = 0;
    for neighbor in neighbors {
        sum += explore(neighbor, allow_one_repeat)
    }

    // set unvisited
    if !is_repeat {
        curr.borrow_mut().visited = false;
    }

    sum
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn parse_input(input: &str) -> Rc<RefCell<Node>> {
    let tokens = input.lines().flat_map(|line| line.split("-"));

    // create nodes
    let hm: HashMap<&str, Rc<RefCell<Node>>> = tokens.map(|token|
        (token, Rc::new(RefCell::new(Node::new(token.to_string()))))
    ).collect();

    // add neighbors
    for line in input.lines() {
        let keys: Vec<&str> = line.split("-").collect();
        for (i, j) in [(0, 1), (1, 0)] {
            let n1 = hm.get(keys[i]).unwrap();
            let n2 = hm.get(keys[j]).unwrap();
            n1.borrow_mut().neighbors.push(Rc::clone(n2));
        }
    }

    Rc::clone(hm.get("start").unwrap())
}

fn part_1(input: &str) -> AocResult<i32> {
    Ok(explore(parse_input(input), false))
}

fn part_2(input: &str) -> AocResult<i32> {
    Ok(explore(parse_input(input), true))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(part_1(input).unwrap(), 10);
        assert_eq!(part_2(input).unwrap(), 36);
    }
}
