#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--example".to_string()) {
        temp.execute(EXAMPLE);
    } else {
        temp.execute(INPUT);
    }
}

struct Temp {}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(Val, Val),
}

#[derive(Debug, Clone, Copy)]
enum Val {
    Reg(char),
    Raw(i64),
}

fn parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Val {
    let val = iter.next().unwrap();
    match val.parse::<i64>() {
        Ok(x) => Val::Raw(x),
        Err(_) => Val::Reg(val.chars().next().unwrap()),
    }
}

fn parse_c<'a>(iter: &mut impl Iterator<Item = &'a str>) -> char {
    iter.next().unwrap().chars().next().unwrap()
}

fn parse_line(line: &str) -> Instruction {
    let mut parts = line.split_whitespace().into_iter();
    let code = parts.next().unwrap();
    match code {
        "set" => Instruction::Set(parse_c(&mut parts), parse(&mut parts)),
        "sub" => Instruction::Sub(parse_c(&mut parts), parse(&mut parts)),
        "mul" => Instruction::Mul(parse_c(&mut parts), parse(&mut parts)),
        "jnz" => Instruction::Jnz(parse(&mut parts), parse(&mut parts)),
        _ => unreachable!(),
    }
}

struct Program {
    instructions: Vec<Instruction>,
    registers: [i64; 8],
    pc: i64,
    mul_count: usize,
}

impl Program {
    fn new(s: &str) -> Self {
        let registers = [0i64; 8];
        Self {
            instructions: s.lines().map(parse_line).collect(),
            registers,
            pc: 0,
            mul_count: 0,
        }
    }

    #[inline(always)]
    fn reg(&mut self, c: char) -> &mut i64 {
        &mut self.registers[c as usize - 'a' as usize]
    }

    fn run_single(&mut self) -> bool {
        if self.pc < 0 || self.pc as usize >= self.instructions.len() {
            return true;
        }
        let instruction = &self.instructions[self.pc as usize];
        dbg!(instruction);
        match &instruction {
            Instruction::Set(c, val) => *self.reg(*c) = self.val(*val),
            Instruction::Sub(c, val) => *self.reg(*c) -= self.val(*val),
            Instruction::Mul(c, val) => {
                dbg!(c, val);
                *self.reg(*c) *= self.val(*val);
                self.mul_count += 1;
            }
            Instruction::Jnz(val, offset) => {
                let val = self.val(*val);
                dbg!(val);
                if val != 0 {
                    self.pc += self.val(*offset);
                    self.pc -= 1;
                }
            }
        }
        self.pc += 1;

        false
    }

    #[inline(always)]
    fn val(&self, val: Val) -> i64 {
        match val {
            Val::Raw(x) => x,
            Val::Reg(c) => self.registers[c as usize - 'a' as usize],
        }
    }
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let instructions: Vec<Instruction> = input.lines().map(|line| parse_line(line)).collect();
        let mut program = Program::new(input);
        loop {
            if program.run_single() {
                break;
            }
        }

        Ok(format!("{}", program.mul_count))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        use primes;

        let mut b = 108400;
        let mut h = 0;
        while b != 125417 {
            if !primes::is_prime(b) {
                h += 1;
            }
            b += 17;
        }

        Ok(format!("{}", h))
    }
}
