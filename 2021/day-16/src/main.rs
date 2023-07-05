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

#[derive(Debug, Clone, Copy)]
enum Bit {
    Zero,
    One,
}

struct BitIterator<'a> {
    digit: usize,
    char_iter: std::str::Chars<'a>,
    curr_val: Option<u8>,
}

fn c_to_i(c: char) -> u8 {
    c.to_digit(16).unwrap() as u8
}

impl<'a> BitIterator<'a> {
    fn new(data: &'a str) -> Self {
        let mut char_iter = data.chars();
        let curr_val = Self::next_val(&mut char_iter);
        Self {
            char_iter,
            digit: 4,
            curr_val,
        }
    }
    fn next_n(&mut self, n: usize) -> Vec<Bit> {
        let mut result = vec![];
        for _ in 0..n {
            if let Some(x) = self.next() {
                result.push(x)
            } else {
                return result;
            }
        }
        result
    }
    fn next_val(cs: &mut std::str::Chars) -> Option<u8> {
        if let Some(c) = cs.next() {
            Some(c_to_i(c))
        } else {
            None
        }
    }
}

struct IntBuilder {
    data: i64,
}

impl IntBuilder {
    fn new() -> Self {
        Self { data: 0i64 }
    }
    fn add_bit(&mut self, bit: Bit) {
        self.data = (self.data << 1)
            | match bit {
                Bit::One => 1,
                Bit::Zero => 0,
            };
    }
    fn build(self) -> i64 {
        self.data
    }
}

fn to_i64(v: Vec<Bit>) -> i64 {
    let mut builder = IntBuilder::new();
    for bit in v.iter() {
        builder.add_bit(*bit);
    }
    builder.build()
}

impl<'a> std::iter::Iterator for BitIterator<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digit == 0 {
            self.digit = 3;
            self.curr_val = Self::next_val(&mut self.char_iter);
        } else {
            self.digit -= 1;
        }
        if let Some(c) = self.curr_val {
            println!("{} - {}", c, self.digit);
            if c >> self.digit & 1u8 > 0 {
                println!("One");
                Some(Bit::One)
            } else {
                println!("Two");
                Some(Bit::Zero)
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Payload {
    Literal(i64),
    Other,
}

#[derive(Debug, Clone, Copy)]
struct Packet {
    version: u8,
    ptype: u8,
    payload: Payload,
}

struct PacketParser<'a> {
    iter: BitIterator<'a>,
}

impl<'a> PacketParser<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: BitIterator::new(s),
        }
    }
    fn parse(&mut self) -> Vec<Packet> {
        let mut result = vec![];
        let version = to_i64(self.iter.next_n(3));
        let ptype = to_i64(self.iter.next_n(3));
        result.push(Packet {
            version: version as u8,
            ptype: ptype as u8,
            payload: Payload::Other,
        });
        result
    }
}

fn part_1(input: &str) -> AocResult<i32> {
    let mut iter = BitIterator::new(input);
    println!("bits: {:?}", iter.take(10).collect::<Vec<Bit>>());

    let mut packets: Vec<Packet> = PacketParser::new(input).parse();
    println!("{:?}", packets);

    Ok(3)
}

fn part_2(input: &str) -> AocResult<i32> {
    Ok(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_bit_iterating() {
        let iter = BitIterator::new("D2FE28");
        let result = iter
            .map(|b| match b {
                Bit::One => '1',
                Bit::Zero => '0',
            })
            .collect::<String>();
        assert_eq!(result, "110100101111111000101000");
    }
}
