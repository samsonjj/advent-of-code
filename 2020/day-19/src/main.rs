#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

pub enum Rule {
    Seq(Vec<Box<Rule>>),
    Any(Vec<Box<Rule>>),
    Literal(char),
    Ref(usize),
}

fn main() {
    solve_and_print(EXAMPLE, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut iter = input.split("\n\n");
    let mut rules = iter.next().unwrap();
    let mut messages = iter.next().unwrap().lines();
    let mut hm = HashMap::new();

    for line in rules.lines() {
        let mut iter = line.split(": ");
        let index = iter.next().unwrap().parse::<usize>().unwrap();
        let eithers = iter
            .next()
            .unwrap()
            .split(" | ")
            .map(|either| {
                either
                    .split(' ')
                    .map(|token| {
                        return match token.parse::<usize>() {
                            Ok(x) => box Rule::Ref(x),
                            Err(_) => {
                                assert_eq!(token.len(), 3);
                                box Rule::Literal(token.chars().skip(1).next().unwrap())
                            }
                        };
                    })
                    .collect::<Vec<Box<Rule>>>()
            })
            .collect::<Vec<Vec<Box<Rule>>>>();

        let rule = Rule::Any(
            eithers
                .into_iter()
                .map(|rules| box Rule::Seq(rules))
                .collect::<Vec<Box<Rule>>>(),
        );

        hm.insert(index, rule);
    }

    let mut sum = 0;
    for message in messages {
        let message_chars = message.chars().collect::<Vec<char>>();
        let result = fits_rule(&message_chars[..], &(hm.get(&0).unwrap()), &hm);
        if result.0 && result.1.len() == 0 {
            sum += 1;
        }
    }

    Ok(sum)
}

fn fits_rule<'a>(
    mut message: &'a [char],
    rule: &Rule,
    hm: &HashMap<usize, Rule>,
) -> (bool, &'a [char]) {
    if message.len() == 0 {
        return (false, message);
    }
    match rule {
        Rule::Literal(c) => (message[0] == *c, &message[1..]),
        Rule::Any(rules) => {
            for rule in rules {
                let result = fits_rule(message, rule, hm);
                if result.0 {
                    return result;
                }
            }
            (false, message)
        }
        Rule::Seq(rules) => {
            for rule in rules {
                let result = fits_rule(message, rule, hm);
                if result.0 == false {
                    return (false, message);
                }
                message = result.1;
            }
            (true, message)
        }
        Rule::Ref(index) => fits_rule(message, hm.get(index).unwrap(), hm),
    }
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut iter = input.split("\n\n");
    let mut rules = iter.next().unwrap();
    let mut messages = iter.next().unwrap().lines();
    let mut hm = HashMap::new();

    let mut rules = rules.lines().collect::<Vec<&str>>();
    rules.extend(["8: 42 | 42 8", "11: 42 31 | 42 11 31"]);
    for line in rules.iter() {
        let mut iter = line.split(": ");
        let index = iter.next().unwrap().parse::<usize>().unwrap();
        let eithers = iter
            .next()
            .unwrap()
            .split(" | ")
            .map(|either| {
                either
                    .split(' ')
                    .map(|token| {
                        return match token.parse::<usize>() {
                            Ok(x) => box Rule::Ref(x),
                            Err(_) => {
                                assert_eq!(token.len(), 3);
                                box Rule::Literal(token.chars().skip(1).next().unwrap())
                            }
                        };
                    })
                    .collect::<Vec<Box<Rule>>>()
            })
            .collect::<Vec<Vec<Box<Rule>>>>();

        let rule = Rule::Any(
            eithers
                .into_iter()
                .map(|rules| box Rule::Seq(rules))
                .collect::<Vec<Box<Rule>>>(),
        );

        hm.insert(index, rule);
    }

    let mut sum = 0;
    for message in messages {
        let message_chars = message.chars().collect::<Vec<char>>();
        let result = fits_rule(&message_chars[..], &(hm.get(&0).unwrap()), &hm);
        if result.0 && result.1.len() == 0 {
            sum += 1;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
