#![feature(box_syntax)]
#![allow(dead_code, unused_variables, unused_imports)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::HashMap;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

static KNOWN_LENGTHS: [usize; 4] = [2, 4, 3, 7];
static DIGIT_NUM: [usize; 10] = [0, 0, 1, 7, 4, 0, 0, 8, 0, 0];

fn part_1(input: &str) -> AocResult<i32> {
    let mut sum = 0;
    for line in input.trim().lines() {
        let parts = line.trim().split(" | ").collect::<Vec<&str>>();
        let output = parts[1];

        let output_digits = output.split(" ");
        for digit in output_digits {
            if KNOWN_LENGTHS.contains(&digit.len()) {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

fn get_number(s: &str, one: &str, four: &str, seven: &str, eight: &str) -> i32 {
    match s.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        // is 0 or 6 or 9
        6 => {
            if !seven.chars().all(|c| s.contains(c)) {
                6
            } else if four.chars().all(|c| s.contains(c)) {
                9
            } else {
                0
            }
        }
        // is 2 or 3 or 5
        5 => {
            if seven.chars().all(|c| s.contains(c)) {
                3
            } else if four.chars().filter(|c| s.contains(*c)).count() == 2 {
                2
            } else {
                5
            }
        }
        _ => panic!("invalid length"),
    }
}

fn part_2(input: &str) -> AocResult<i32> {
    let mut sum = 0;
    for line in input.trim().lines() {
        let parts = line.trim().split(" | ").collect::<Vec<&str>>();
        let ten_digits = parts[0];

        let mut hm = HashMap::new();
        let mut num_ref = HashMap::new();
        for digit in ten_digits.split(" ") {
            if KNOWN_LENGTHS.contains(&digit.len()) {
                hm.insert(digit, DIGIT_NUM[digit.len()]);
                num_ref.insert(DIGIT_NUM[digit.len()], digit);
            }
        }
        // for digit in ten_digits.split(" ") {
        //     hm.insert(
        //         digit,
        //         get_number(
        //             digit,
        //             num_ref.get(&1usize).unwrap(),
        //             num_ref.get(&4usize).unwrap(),
        //             num_ref.get(&7usize).unwrap(),
        //             num_ref.get(&8usize).unwrap(),
        //         ) as usize,
        //     );
        // }

        let mut result = String::new();
        for digit in parts[1].split(" ") {
            result = format!(
                "{}{}",
                result,
                get_number(
                    digit,
                    num_ref.get(&1usize).unwrap(),
                    num_ref.get(&4usize).unwrap(),
                    num_ref.get(&7usize).unwrap(),
                    num_ref.get(&8usize).unwrap(),
                ) as usize
            );
        }
        sum += result.parse::<i32>().unwrap();
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

    #[test]
    fn get_number_test() {
        assert_eq!(get_number("cdfbe", "ab", "eafb", "dab", "acedgfb"), 5);
    }
}
