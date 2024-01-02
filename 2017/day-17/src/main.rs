#![feature(linked_list_cursors)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
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

use std::collections::LinkedList;

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let steps: usize = input.parse()?;
        let mut list: HashMap<usize, usize> = HashMap::new();
        list.insert(0, 0);
        let mut current = 0;
        dbg!(steps);

        for i in 1..=2017 {
            let mod_steps = steps % list.len();
            for _ in 0..mod_steps {
                current = list[&current];
            }

            let tmp = list[&current];
            list.insert(current, i);
            list.insert(i, tmp);
            current = list[&current];
        }

        Ok(format!("{}", list[&2017]))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let steps: usize = input.parse()?;
        let mut current_index = 0;
        let mut val_after_zero = 0;

        for i in 1..=50_000_000 {
            current_index = (current_index + steps) % i;
            if current_index == 0 {
                val_after_zero = i;
            }
            current_index += 1;
        }

        Ok(format!("{}", val_after_zero))
    }
}
