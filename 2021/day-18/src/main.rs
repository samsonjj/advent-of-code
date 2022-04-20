#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Pair { left: Box<Node>, right: Box<Node> },
    Num(u64),
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        let char_peeker = &mut s.chars().peekable();

        fn parse_number(iter: &mut Peekable<Chars>) -> u64 {
            let mut token = String::new();
            loop {
                let c = iter.peek().unwrap();
                if c == &',' || c == &']' {
                    break;
                }
                token.push(iter.next().unwrap());
            }
            token.parse::<u64>().unwrap()
        }

        fn parse_node(iter: &mut Peekable<Chars>) -> Node {
            let c = iter.peek().unwrap();
            if c == &'[' {
                parse_pair(iter)
            } else {
                Node::Num(parse_number(iter))
            }
        }

        fn parse_pair(iter: &mut Peekable<Chars>) -> Node {
            let _ = iter.next(); // should be '['

            // parse left node
            let left = parse_node(iter);
            // ','
            let _ = iter.next();
            // parse right node
            let right = parse_node(iter);

            // ']'
            iter.next();

            Node::Pair {
                left: box left,
                right: box right,
            }
        }

        parse_pair(char_peeker)
    }
}

impl Node {
    fn explode(&mut self) {
        // assumes that this node is
    }

    /// applies the reduction rules one time. returns true if a change was made.
    fn reduce(&mut self, depth: i32) -> bool {
        match self {
            Node::Pair { left: l, right: r } => {
                if depth == 4 {
                    // explode
                    return true;
                }

                if l.reduce(depth + 1) {
                    return true;
                }
                return r.reduce(depth + 1);
            }

            Node::Num(x) => {
                if *x > 10 {
                    // split
                    return true;
                }
            }
        }

        false
    }

    pub fn add(self, other: Node) -> Self {
        let mut result = Node::Pair {
            left: box self,
            right: box other,
        };
        while result.reduce(0) {}
        result
    }
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    println!("{:?}", Node::from(input));
    Ok(3)
}

fn part_2(input: &str) -> AocResult<i32> {
    Ok(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_test() {
        // [[1, 2],[5,[7,[3,9]]]]
        let expected = Node::Pair {
            left: box Node::Pair {
                left: box Node::Num(1),
                right: box Node::Num(2),
            },
            right: box Node::Pair {
                left: box Node::Num(5),
                right: box Node::Pair {
                    left: box Node::Num(7),
                    right: box Node::Pair {
                        left: box Node::Num(3),
                        right: box Node::Num(9),
                    },
                },
            },
        };
        let actual = Node::from("[[1,2],[5,[7,[3,9]]]]");
        assert_eq!(expected, actual);
    }
}
