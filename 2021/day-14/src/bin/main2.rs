#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i64> {
    let mut parts = input.split("\n\n");
    let mut template: String = parts.next().unwrap().to_string();
    let rules: HashMap<&str, char> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let pair = parts.next().unwrap();
            let c = parts.next().unwrap().chars().next().unwrap();
            (pair, c)
        })
        .collect();

    for i in 0..10 {
        let mut result = String::new();
        for j in 0..template.len() - 1 {
            let s = &template[j..j + 2];
            if let Some(c) = rules.get(s) {
                result = format!("{}{}{}", result, &s[0..1], c);
            } else {
                result = format!("{}{}", result, &s[0..1]);
            }
        }
        result = format!("{}{}", result, &template[template.len() - 1..]);
        template = result;
    }

    let mut hm = HashMap::new();
    for c in template.chars() {
        let data = hm.entry(c).or_insert(0);
        *data += 1;
    }
    let mut max = ('a', 0);
    let mut min = ('a', i32::MAX);
    for (c, count) in hm.iter() {
        if count > &max.1 {
            max = (*c, *count);
        }
        if count < &min.1 {
            min = (*c, *count);
        }
    }
    Ok((max.1 - min.1) as i64)
}

fn part_2(input: &str) -> AocResult<i64> {
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

    let mut pairs = HashMap::new();
    let mut last_char = None;
    for c2 in template.chars() {
        if let Some(c1) = last_char {
            pairs.insert((c1, c2), 1);
        }
        last_char = Some(c2);
    }

    const ITERATIONS: i32 = 40;
    for i in 0..ITERATIONS {
        let mut new_pairs = HashMap::new();
        for ((c1, c2), count) in pairs {
            let list = if let Some(c) = rules.get(&(c1, c2)) {
                vec![(c1, *c), (*c, c2)]
            } else {
                vec![(c1, c2)]
            };
            for pair in list {
                let data = new_pairs.entry(pair).or_insert(0);
                *data += count;
            }
        }
        pairs = new_pairs;
    }

    let mut counts = HashMap::new();
    for (pair, count) in pairs {
        let data = counts.entry(pair.1).or_insert(0);
        *data += count;
    }

    let data = counts.entry(template.chars().next().unwrap()).or_insert(0);
    *data += 1;

    let mut max = ('a', 0);
    let mut min = ('a', i64::MAX);
    for (c, count) in counts.iter() {
        if count > &max.1 {
            max = (*c, *count);
        }
        if count < &min.1 {
            min = (*c, *count);
        }
    }
    Ok(max.1 - min.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
