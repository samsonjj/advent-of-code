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

struct BitIterator {
    data: String,
    index: usize,
    digit_index: usize,
}

impl BitIterator {}

impl std::iter::Iterator for BitIterator {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {}
}

struct HexadecimalStr<'a>(&'a str);

impl HexadecimalStr {
    fn into_bit_iterator() -> BitIterator {}
}

pub enum Bit {
    ZERO,
    ONE,
}

impl Bit {
    fn bits_from_char(c: char) -> Result<Vec<Self>, ()> {
        let mut digit = c.to_digit(16).ok_or(Err(()))?;
        bits = vec![];
        for i in 0..4 {
            bits.push(if digit & 1 { Bit::ONE } else { Bit::ZERO });
            digit <<= 1;
        }
        bits
    }
}

pub struct Packet {
    version: u8,
}

fn parse_input(input: &str) -> Vec<Packet> {
    // &str -> bits
    let bytes = input
        .chars()
        .flat_map(|c| Bit::bits_from_char(c))
        .collect::<Vec<Bit>>();
    println!("bits={:?}", bits);

    vec![]
}

fn part_1(input: &str) -> AocResult<i32> {
    parse_input(input);
    Ok(3)
}

fn part_2(input: &str) -> AocResult<i32> {
    Ok(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
