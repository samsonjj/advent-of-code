#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

#[derive(Clone, Debug)]
enum Operation {
    Mul,
    Add,
}

#[derive(Clone, Debug)]
enum Operand {
    Num(i64),
    Old,
}

struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    operand: Operand,
    divisor: i64,
    if_true: usize,
    if_false: usize,
    count: i64,
}

fn perform_operation(val: i64, operation: &Operation, operand: &Operand) -> i64 {
    let operand = match operand {
        Operand::Old => val,
        Operand::Num(x) => *x,
    };
    match operation {
        Operation::Add => val + operand,
        Operation::Mul => val * operand,
    }
}

macro_rules! ternary {
    ($condition: expr, $_true: expr, $_false: expr) => {
        if $condition {
            $_true
        } else {
            $_false
        }
    };
}

fn create_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let monkey_texts = input.split("\n\n");

    let operation_re = Regex::new(r"^Operation: new = old (\+|\*) (.*)$").unwrap();
    let test_re = Regex::new(r"^Test: divisible by (.*)$").unwrap();
    let true_condition_re = Regex::new(r"^If true: throw to monkey (.*)$").unwrap();
    let false_condition_re = Regex::new(r"^If false: throw to monkey (.*)$").unwrap();

    for monkey_text in monkey_texts {
        let mut lines = monkey_text.lines().skip(1);

        let mut starting_items = lines.next().unwrap().split(":").nth(1).unwrap().to_string();
        starting_items.retain(|c| !c.is_whitespace());
        let starting_items: Vec<i64> = starting_items
            .split(",")
            .map(|item| item.parse::<i64>().unwrap())
            .collect();

        let cap = operation_re.captures(lines.next().unwrap().trim()).unwrap();
        let operation = match cap.get(1).unwrap().as_str() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => panic!("invalid operation"),
        };
        let operand = match cap.get(2).unwrap().as_str() {
            "old" => Operand::Old,
            x => Operand::Num(x.parse::<i64>().unwrap()),
        };

        let divisor = test_re
            .captures(lines.next().unwrap().trim())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();

        let if_true = true_condition_re
            .captures(lines.next().unwrap().trim())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let if_false = false_condition_re
            .captures(lines.next().unwrap().trim())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        monkeys.push(Monkey {
            items: starting_items,
            operand,
            operation,
            if_true,
            if_false,
            divisor,
            count: 0,
        })
    }
    monkeys
}

fn calc_monkey_business_level(monkeys: &Vec<Monkey>) -> i64 {
    let counts: Vec<i64> = monkeys.iter().map(|m| m.count).collect();
    let mut maxes = vec![0, 0];
    for count in counts {
        if count > maxes[0] {
            maxes[0] = count;
        }
        maxes.sort();
    }

    maxes[0] * maxes[1]
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut monkeys = create_monkeys(input);
        for round in 0..20 {
            for monkey_index in 0..monkeys.len() {
                while !monkeys[monkey_index].items.is_empty() {
                    let (item, to_monkey_index) = {
                        let monkey = &mut monkeys[monkey_index];
                        monkey.count += 1;
                        let mut item = monkey.items.remove(0);
                        item = perform_operation(item, &monkey.operation, &monkey.operand);
                        item = item / 3;
                        (
                            item,
                            ternary!(item % monkey.divisor == 0, monkey.if_true, monkey.if_false),
                        )
                    };
                    monkeys[to_monkey_index].items.push(item);
                }
            }
        }

        Ok(format!("{}", calc_monkey_business_level(&monkeys)))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let mut monkeys = create_monkeys(input);

        // performing a modulus divison by this number reduces the size of the "worry", without losing any information for future modulo operations.
        let magic_number = monkeys
            .iter()
            .map(|m| m.divisor)
            .fold(1, |acc, element| acc * element);

        for round in 0..10000 {
            for monkey_index in 0..monkeys.len() {
                while !monkeys[monkey_index].items.is_empty() {
                    let (item, to_monkey_index) = {
                        let monkey = &mut monkeys[monkey_index];
                        monkey.count += 1;
                        let mut item = monkey.items.remove(0);
                        item = perform_operation(item, &monkey.operation, &monkey.operand);
                        item = item % magic_number;
                        (
                            item,
                            ternary!(item % monkey.divisor == 0, monkey.if_true, monkey.if_false),
                        )
                    };
                    monkeys[to_monkey_index].items.push(item);
                }
            }
        }

        Ok(format!("{}", calc_monkey_business_level(&monkeys)))
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
