use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    // position -> count
    let width = INPUT.lines().next().unwrap().len();
    let mut hm = HashMap::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            let data = hm.entry(i).or_insert(0);
            *data += match c {
                '0' => -1,
                '1' => 1,
                _ => panic!("invalid character, {}", c),
            };
        }
    }

    let hm = hm
        .into_iter()
        .map(|(idx, count)| {
            (
                idx,
                match count {
                    x if x > 0 => '1',
                    _ => '0',
                },
            )
        })
        .collect::<HashMap<usize, char>>();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (idx, c) in hm.iter() {
        if *c == '1' {
            // if width=12, 12-11-1 = 0
            gamma_rate += 2i64.pow(width.wrapping_sub(*idx).wrapping_sub(1) as u32);
        } else {
            // if width=12, 12-11-1 = 0
            epsilon_rate += 2i64.pow(width.wrapping_sub(*idx).wrapping_sub(1) as u32);
        }
    }

    println!("{}", gamma_rate * epsilon_rate);

    let mut idx: usize = 0;
    let mut lines: HashSet<&'static str> = INPUT.lines().filter(|line| !line.is_empty()).collect();
    while lines.len() > 1 {
        lines = lines
            .into_iter()
            .filter(|line| line.as_bytes()[idx] as char == *hm.get(&idx).unwrap())
            .collect();
        idx += 1;
    }
    let oxygen_rating: i32 = i32::from_str_radix(lines.iter().next().unwrap(), 2).unwrap();

    //

    let mut idx: usize = 0;
    let mut lines: HashSet<&'static str> = INPUT.lines().filter(|line| !line.is_empty()).collect();
    while lines.len() > 1 {
        println!("lines.len()={}", lines.len());
        lines = lines
            .into_iter()
            .filter(|line| line.as_bytes()[idx] as char != *hm.get(&idx).unwrap())
            .collect();
        idx += 1;
    }
    println!("lines.len()={}", lines.len());
    let co2_rating: i32 = i32::from_str_radix(lines.iter().next().unwrap(), 2).unwrap();

    println!("{}", co2_rating * oxygen_rating);
}
