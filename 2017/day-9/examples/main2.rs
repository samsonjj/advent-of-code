#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::time;

static INPUT: &str = include_str!("../src/input.txt");
static EXAMPLE: &str = include_str!("../src/example.txt");

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

struct CancelParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CancelParser<'a> {
    fn peek(&mut self) -> char {
        self.consume_cancelled();
        *self.chars.peek().unwrap()
    }

    fn next(&mut self) -> char {
        self.consume_cancelled();
        self.chars.next().unwrap()
    }

    fn take(&mut self, c: char) -> char {
        assert_eq!(self.next(), c);
        c
    }

    fn take_until(&mut self, c: char) -> i32 {
        let mut garbage = 0;
        while self.next() != c {
            garbage += 1;
        }
        garbage
    }

    fn consume_cancelled(&mut self) {
        while let Some('!') = self.chars.peek() {
            self.chars.next();
            self.chars.next();
        }
    }
}

// recrusive descent parser
struct Parser<'a> {
    chars: CancelParser<'a>,
    garbage: i32,
    sum: i32,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        let cancel_parser = CancelParser {
            chars: s.chars().peekable(),
        };
        Self {
            chars: cancel_parser,
            garbage: 0,
            sum: 0,
        }
    }

    fn parse_garbage(&mut self, x: i32) {
        self.chars.take('<');
        self.garbage += self.chars.take_until('>');
    }

    fn parse_group(&mut self, mut x: i32) {
        x += 1;
        self.sum += x;

        self.chars.take('{');
        let result = self.parse_group_set(x);
        self.chars.take('}');
    }

    fn parse_group_or_garbage(&mut self, x: i32) {
        match self.chars.peek() {
            '{' => self.parse_group(x),
            '<' => self.parse_garbage(x),
            _ => unreachable!(),
        }
    }

    fn parse_group_set(&mut self, x: i32) {
        if self.chars.peek() == '}' {
            return;
        }

        loop {
            self.parse_group_or_garbage(x);
            if self.chars.peek() != ',' {
                break;
            }
            self.chars.next();
        }
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut parser = Parser::new(input);
        parser.parse_group(0);
        Ok(format!("{:?}", parser.sum))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut parser = Parser::new(input);
        parser.parse_group(0);
        Ok(format!("{:?}", parser.garbage))
    }
}
