use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut hs: HashSet<char> = HashSet::new();

    let mut sum = 0;
    for data in INPUT.split("\n\n") {
        hs.clear();
        for c in data.chars() {
            match c {
                'a'..='z' => {
                    hs.insert(c);
                }
                _ => {}
            }
        }
        sum += hs.len();
    }

    println!("{}", sum);

    let mut sum = 0;
    for data in INPUT.split("\n\n") {
        let mut hm: HashMap<char, i32> = HashMap::new();
        let mut num_lines = 0;
        for c in data.chars() {
            match c {
                'a'..='z' => {
                    *hm.entry(c).or_insert(0) += 1;
                }
                '\n' => num_lines += 1,
                _ => {}
            }
        }
        sum += hm
            .iter()
            .map(|(_, v)| v == &(num_lines + 1))
            .filter(|b| *b)
            .count();
    }

    println!("{}", sum);
}
