#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut result = 0;
        let mut index = 1;
        let lines = input.lines().collect::<Vec<&str>>();
        for chunk in lines.chunks(3) {
            let left = chunk[0].parse::<Node>().unwrap();
            let right = chunk[1].parse::<Node>().unwrap();

            if compare(&left, &right).unwrap() {
                result += index;
            } else {
            }
            index += 1;
        }
        Ok(format!("{}", result))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let nodes = input
            .lines()
            .filter(|line| line.len() > 0)
            .map(|line| line.parse::<Node>().unwrap())
            .collect::<Vec<Node>>();
        let node_1 = Node::List(vec![Node::List(vec![Node::Num(2)])]);
        let node_2 = Node::List(vec![Node::List(vec![Node::Num(6)])]);
        let mut node_refs: Vec<&Node> = nodes.iter().collect();
        node_refs.push(&node_1);
        node_refs.push(&node_2);

        node_refs.sort();

        let position = node_refs
            .iter()
            .map(|r| (*r) as *const _)
            .position(|r| r == (&node_1) as *const _)
            .unwrap();
        let position_2 = node_refs
            .iter()
            .map(|r| (*r) as *const _)
            .position(|r| r == (&node_2) as *const _)
            .unwrap();

        Ok(format!("{}", (position + 1) * (position_2 + 1)))
    }
}

fn compare_num_list(left: &Node, right: &Node) -> Option<bool> {
    match (left, right) {
        (x @ &Node::Num(val), a @ &Node::List(_)) => compare(&Node::List(vec![Node::Num(val)]), a),
        _ => unreachable!(),
    }
}

/// returns if the pair is in the right order (i.e. left), or None if they are equal.
/// for the pruposes of this problem, no nodes should ever be equal.
fn compare(left: &Node, right: &Node) -> Option<bool> {
    match (left, right) {
        (&Node::Num(x), &Node::Num(y)) => {
            if x == y {
                None
            } else {
                Some(x < y)
            }
        }
        (&Node::List(ref a), Node::List(ref b)) => {
            for (left_child, right_child) in a.iter().zip(b) {
                if let result @ Some(_) = compare(left_child, right_child) {
                    return result;
                }
            }
            if a.len() == b.len() {
                None
            } else {
                Some(a.len() < b.len())
            }
        }
        (x @ &Node::Num(_), a @ &Node::List(_)) => compare_num_list(x, a),
        (a @ &Node::List(_), x @ &Node::Num(_)) => compare_num_list(x, a).map(|val| !val),
    }
}

#[derive(Debug)]
enum Node {
    List(Vec<Node>),
    Num(i32),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match compare(&self, &other) {
            None => Some(std::cmp::Ordering::Equal),
            Some(true) => Some(std::cmp::Ordering::Less),
            Some(false) => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match compare(&self, &other) {
            None => std::cmp::Ordering::Equal,
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
        }
    }
}

struct NodeParser<'a> {
    chars: Peekable<std::str::Chars<'a>>,
}

impl<'a> NodeParser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
        }
    }

    pub fn parse(mut self) -> Node {
        self.parse_node()
    }

    fn skip_next_whitespaces(&mut self) {
        while char::is_whitespace(*self.chars.peek().unwrap()) {
            self.chars.next().unwrap();
        }
    }

    fn get(&mut self) -> char {
        let c = self.chars.next().unwrap();
        c
    }

    fn get_char(&mut self, c: char) -> char {
        self.skip_next_whitespaces();
        let result = self.get();
        assert_eq!(result, c);
        return result;
    }

    fn peek(&mut self) -> &char {
        self.skip_next_whitespaces();
        self.chars.peek().unwrap()
    }

    fn get_i32(&mut self) -> i32 {
        let mut s = String::new();
        while char::is_numeric(*self.chars.peek().unwrap()) {
            s.push(self.get());
        }
        s.parse::<i32>().unwrap()
    }

    fn parse_node(&mut self) -> Node {
        if self.peek() == &'[' {
            self.parse_list()
        } else {
            self.parse_num()
        }
    }

    fn parse_num(&mut self) -> Node {
        let val = self.get_i32();
        Node::Num(val)
    }

    fn parse_list(&mut self) -> Node {
        let mut items = Vec::new();
        let c = self.get_char('[');
        let mut first = true;
        while self.peek() != &']' {
            if !first {
                self.get_char(',');
            } else {
                first = false;
            }
            items.push(self.parse_node());
        }
        self.get_char(']');
        Node::List(items)
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NodeParser::new(s).parse())
    }
}
