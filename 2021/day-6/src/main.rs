#![feature(box_syntax)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone, Debug)]
struct LanternFish {
    timer: i64,
}

fn fish_to_str(fish: &LanternFish) -> String {
    return fish.timer.to_string();
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<usize> {
    let mut fish: Vec<LanternFish> = input
        .trim()
        .split(',')
        .map(|f| LanternFish {
            timer: f.parse::<i64>().unwrap(),
        })
        .collect();
    for i in 0..80 {
        // println!(
        //     "{:?}",
        //     fish.iter().map(fish_to_str).collect::<Vec<String>>()
        // );
        let mut birthed: Vec<LanternFish> = Vec::new();
        for f in fish.iter_mut() {
            f.timer -= 1;
            if f.timer < 0 {
                f.timer = 6;
                birthed.push(LanternFish { timer: 8 });
            }
        }
        fish.extend(birthed);
    }
    Ok(fish.len())
}

fn part_2(input: &str) -> AocResult<usize> {
    let mut fish: Vec<i32> = input
        .trim()
        .split(',')
        .map(|f| f.parse::<i32>().unwrap())
        .collect();
    let mut fish_map = HashMap::new();
    for f in fish {
        let data = fish_map.entry(f).or_insert(0);
        *data += 1;
    }

    for i in 0..256 {
        let mut next_fish_map = HashMap::new();
        for (timer, count) in fish_map.iter() {
            if timer == &0 {
                next_fish_map.insert(8, *count);
            }
            let new_timer = match timer {
                x if x == &0 => 6,
                x => x - 1,
            };
            let data = next_fish_map.entry(new_timer).or_insert(0);
            *data += count;
        }
        fish_map = next_fish_map;
    }

    Ok(fish_map.iter().map(|(timer, count)| count).sum::<usize>())
}
