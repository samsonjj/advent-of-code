#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use ndarray::{Array1, Array2};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i64> {
    compute(input, 10)
}

fn part_2(input: &str) -> AocResult<i64> {
    compute(input, 40)
}

fn compute(input: &str, iterations: usize) -> AocResult<i64> {
    let mut parts = input.split("\n\n");
    let mut template: String = parts.next().unwrap().to_string();
    let rules: HashMap<(char, char), char> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let pair = parts.next().unwrap();
            let c = parts.next().unwrap().chars().next().unwrap();
            let mut iter = pair.chars();
            ((iter.next().unwrap(), iter.next().unwrap()), c)
        })
        .collect();

    let mut index = 0;
    // bigram_indices
    let mut bi = HashMap::new();
    for (bigram, _) in rules.iter() {
        bi.insert(*bigram, index);
        index += 1;
    }

    let mut transition_matrix = Array2::<i64>::zeros((bi.len(), bi.len()));
    for (bigram, c) in rules.iter() {
        let i = bi.get(bigram).unwrap();
        transition_matrix[[*i, *bi.get(&(bigram.0, *c)).unwrap()]] = 1;
        transition_matrix[[*i, *bi.get(&(*c, bigram.1)).unwrap() as usize]] = 1;
    }

    let mut in_vec = Array1::<i64>::zeros(bi.len());

    let mut last_char = None;
    for c2 in template.chars() {
        if let Some(c1) = last_char {
            let index = bi.get(&(c1, c2)).unwrap();
            in_vec[*index] += 1;
        }
        last_char = Some(c2);
    }

    let start = std::time::Instant::now();

    for i in 0..iterations {
        in_vec = in_vec.dot(&transition_matrix);
    }
    println!(
        "took {} milliseconds to do matrix exponentiation",
        (std::time::Instant::now() - start).as_millis()
    );

    let mut counts = HashMap::new();
    for (pair, i) in bi.into_iter() {
        let count = in_vec[i];
        let data = counts.entry(pair.0).or_insert(0);
        *data += count;

        let data = counts.entry(pair.1).or_insert(0);
        *data += count;
    }

    let data = counts.entry(template.chars().next().unwrap()).or_insert(0);
    *data += 1;
    let data = counts
        .entry(template.chars().rev().next().unwrap())
        .or_insert(0);
    *data += 1;

    let mut max = ('a', 0);
    let mut min = ('a', i64::MAX);
    for (c, count) in counts.iter() {
        if count / 2 > max.1 {
            max = (*c, *count / 2);
        }
        if count / 2 < min.1 {
            min = (*c, *count / 2);
        }
    }
    Ok(max.1 - min.1)
}
