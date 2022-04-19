#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use bits::{Bit, Biterator, NumBuilder};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::num;
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

mod bits;

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn perform_op(op: u32, values: Vec<u64>) -> u64 {
    match op {
        0 => values.into_iter().sum::<u64>(),
        1 => values.into_iter().product::<u64>(),
        2 => values.into_iter().min().unwrap(),
        3 => values.into_iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("bad op code: {:?}", op),
    }
}

fn parse_packet(biterator: &mut Biterator) -> (u32, u64) {
    let header = parse_header(biterator);
    let version = header.0;
    let type_id = header.1;

    // type ID is 4 means the packet is a literal
    if type_id == 4 {
        (version, parse_literal_body(biterator))
    } else {
        let result = parse_op_body(biterator);
        (version + result.0, perform_op(type_id, result.1))
    }
}

fn parse_op_body(biterator: &mut Biterator) -> (u32, Vec<u64>) {
    // 1 bit length type ID
    let length_type_id = biterator.next().unwrap();

    // variable bit internal packets
    if let Bit::Zero = length_type_id {
        let length = biterator.parse(15).unwrap();
        parse_op_data_total_length(biterator, length as usize)
    } else {
        let num_packets = biterator.parse(11).unwrap();
        let mut versions_total = 0;
        let mut nums = vec![];
        for _ in 0..num_packets {
            let r = parse_packet(biterator);
            versions_total += r.0;
            nums.push(r.1);
        }
        (versions_total, nums)
    }
}

fn parse_op_data_total_length(biterator: &mut Biterator, length: usize) -> (u32, Vec<u64>) {
    let starting_bits_processed = biterator.bits_processed;
    let mut versions_total = 0u32;
    let mut nums = vec![];
    while biterator.bits_processed - starting_bits_processed < length {
        let result = parse_packet(biterator);
        versions_total += result.0;
        nums.push(result.1);
    }
    (versions_total, nums)
}

fn parse_header(biterator: &mut Biterator) -> (u32, u32) {
    // 3 bits version
    let version = biterator.parse(3).unwrap();
    // 3 bits type ID
    let type_id = biterator.parse(3).unwrap();

    (version as u32, type_id as u32)
}

fn parse_literal_body(biterator: &mut Biterator) -> u64 {
    let mut num_builder = NumBuilder::new();
    loop {
        let leading_bit = biterator.next().unwrap();
        for _ in 0..4 {
            num_builder.push(biterator.next().unwrap());
        }
        if let Bit::Zero = leading_bit {
            break;
        }
    }
    num_builder.build() as u64
}

fn part_1(input: &str) -> AocResult<i64> {
    let mut biterator = Biterator::from(input);
    Ok(parse_packet(&mut biterator).0 as i64)
}

fn part_2(input: &str) -> AocResult<i64> {
    let mut biterator = Biterator::from(input);
    Ok(parse_packet(&mut biterator).1 as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_parse_literal() {
        use super::*;
        // from examples
        const DATA: &'static str = "D2FE28";
        let mut biterator = Biterator::from(DATA);
        let result = parse_packet(&mut biterator);
        assert_eq!(result.0, 6);
        assert_eq!(biterator.next(), Some(Bit::Zero));
        assert_eq!(biterator.next(), Some(Bit::Zero));
        assert_eq!(biterator.next(), Some(Bit::Zero));
        assert_eq!(biterator.next(), None);
    }

    #[test]
    fn test_parse_op() {
        use super::*;
        let inputs = vec![
            ("38006F45291200", 1),
            //
            // 001 110 0 000000000011011, 110 100 01010, 010 100 10001 00100 0000000
            // ("C200B40A82", 3),
            // ("04005AC33890", 54),
            // ("880086C3E88112", 7),
            // ("CE00C43D881120", 9),
            // ("D8005AC2A8F0", 1),
            // ("F600BC2D8F", 0),
            // ("9C005AC2F8F0", 0),
            // ("9C0141080250320F1802104A08", 1),
            // ("802C200B40A82C200B40A82", 6),
            /*

            000000 100000000010

            100111000000000001011010110000101111100011110000

            10011100000000010100000100001000000000100101000000110010000011110001100000000010000100000100101000001000
             */
        ];

        // from examples
        for input in inputs {
            let mut biterator = Biterator::from(input.0);
            assert_eq!(parse_packet(&mut biterator).1, input.1);
        }
    }

    #[test]
    fn sum_test() {
        let nums: Vec<u64> = vec![
            199071281,
            240,
            7,
            0,
            2513,
            0,
            751,
            936579,
            84,
            0,
            54640,
            363065742,
            0,
            1625637177,
            650304435,
            4575502,
            13360882,
            2138868,
            10570687,
            2456541,
            42444,
            0,
            101077955,
            37262911444,
            1891,
            138,
            497655,
            3,
            42002,
            18874,
            9,
            0,
            0,
            31080,
            0,
            4,
            94499410,
            2214,
            27060,
            0,
            3547,
            101376,
            0,
            81,
            66,
            0,
            10,
            31918501,
            61123317420,
            2195,
            0,
            0,
            14349575,
        ];

        let result = nums.into_iter().sum::<u64>();
        assert!(result > 0);
    }
}
