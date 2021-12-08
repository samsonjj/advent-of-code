const INPUT: &str = include_str!("input.txt");

use std::collections::{HashMap, HashSet};

fn main() {
    let mut edges: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut reverse_edges: HashMap<&'static str, Vec<(&'static str, i32)>> = HashMap::new();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        // child is the bag that "contains"
        let child = parts[0];
        // parent in the bag that is contained
        for parent_str in parts[1].split(", ") {
            if parent_str == "no other bags." {
                continue;
            }
            let mut record = 0;
            let mut min = 0;
            let mut max = 0;
            for (i, c) in parent_str.char_indices() {
                if c == ' ' {
                    record += 1;
                    if record == 3 {
                        max = i;
                        break;
                    } else if record == 1 {
                        min = i + 1;
                        continue;
                    }
                }
            }
            let parent = &parent_str[min..max];
            let num = &parent_str[0..min - 1];
            edges.entry(parent).or_insert(vec![]).push(child);
            reverse_edges
                .entry(child)
                .or_insert(vec![])
                .push((parent, num.parse::<i32>().unwrap()));
        }
    }

    for child in edges.get("shiny gold").unwrap_or(&vec![]) {
        explore_node(child, &edges, &mut visited);
    }

    println!("{}", visited.len());
    println!(
        "{}",
        explore_node_count("shiny gold", &reverse_edges, 0) - 1
    );
}

fn explore_node(
    current_node: &'static str,
    edges: &HashMap<&'static str, Vec<&'static str>>,
    visited: &mut HashSet<&'static str>,
) {
    if visited.contains(current_node) {
        return;
    }
    if let Some(children) = edges.get(current_node) {
        for child in children {
            explore_node(child, edges, visited);
        }
    }

    visited.insert(current_node);
}

fn explore_node_count(
    current_node: &'static str,
    reverse_edges: &HashMap<&'static str, Vec<(&'static str, i32)>>,
    depth: usize,
) -> i32 {
    if let Some(children) = reverse_edges.get(current_node) {
        let mut sum = 0;
        for (child, n) in children {
            sum += n * explore_node_count(child, reverse_edges, depth + 1);
        }
        sum + 1
    } else {
        1
    }
}
