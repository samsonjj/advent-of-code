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

    let mut memoize: HashMap<(char, char, i32), HashMap<char, i64>> = HashMap::new();
    let mut pairs = vec![];
    let mut last_char = None;
    for c2 in template.chars() {
        if let Some(c1) = last_char {
            pairs.push((c1, c2));
        }
        last_char = Some(c2);
    }

    let mut counts = HashMap::new();
    let data = counts.entry(pairs[0].0).or_insert(0);
    *data += 1;

    const ITERATIONS: i32 = 40;
    for pair in pairs {
        compute(pair, ITERATIONS, &mut memoize, &rules);
        for (c, count) in memoize.get(&(pair.0, pair.1, ITERATIONS)).unwrap().iter() {
            let data = counts.entry(*c).or_insert(0);
            *data += count;
        }
    }

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

// just fill up the memoize table
fn compute<'a>(
    pair: (char, char),
    iterations: i32,
    memoize: &'a mut HashMap<(char, char, i32), HashMap<char, i64>>,
    rules: &HashMap<(char, char), char>,
) {
    if let Some(result) = memoize.get(&(pair.0, pair.1, iterations)) {
        return;
    } else if iterations == 0 {
        let mut counts = HashMap::new();
        // leave this out so we don't double count letters
        // will have to recount the first letter later
        // counts.insert(pair.0, 1);
        counts.insert(pair.1, 1);
        memoize.insert((pair.0, pair.1, 0), counts);
        return;
    } else {
        let pairs = if let Some(c) = rules.get(&pair) {
            vec![(pair.0, *c), (*c, pair.1)]
        } else {
            vec![pair]
        };
        let mut counts = HashMap::new();
        for pair in pairs {
            compute(pair, iterations - 1, memoize, rules);
            add_counts(
                &mut counts,
                memoize.get(&(pair.0, pair.1, iterations - 1)).unwrap(),
            );
        }
        memoize.insert((pair.0, pair.1, iterations), counts);
        return;
    }
}

fn add_counts(hm1: &mut HashMap<char, i64>, hm2: &HashMap<char, i64>) {
    for item in hm2.into_iter() {
        let data = hm1.entry(*item.0).or_insert(0);
        *data += item.1;
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
