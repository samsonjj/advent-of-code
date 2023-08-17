#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let temp = Temp {};
    if args.contains(&"--example".to_string()) {
        temp.execute(EXAMPLE);
    } else {
        temp.execute(INPUT);
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(char),
    Jgz(Val, Val),
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
    match parts.next().unwrap() {
        "snd" => Instruction::Snd(parse(&mut parts)),
        "set" => Instruction::Set(parse_c(&mut parts), parse(&mut parts)),
        "add" => Instruction::Add(parse_c(&mut parts), parse(&mut parts)),
        "mul" => Instruction::Mul(parse_c(&mut parts), parse(&mut parts)),
        "mod" => Instruction::Mod(parse_c(&mut parts), parse(&mut parts)),
        "rcv" => Instruction::Rcv(parse_c(&mut parts)),
        "jgz" => Instruction::Jgz(parse(&mut parts), parse(&mut parts)),
        _ => unreachable!(),
    }
}

struct Duet {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    pc: i64,
    last_sound: i64,
}

impl Duet {
    fn new(s: &str) -> Self {
        Self {
            instructions: s.lines().map(parse_line).collect(),
            registers: HashMap::new(),
            pc: 0,
            last_sound: 0,
        }
    }

    fn reg(&mut self, c: char) -> &mut i64 {
        self.registers.entry(c).or_insert(0)
    }

    fn run_single(&mut self) -> Option<Option<i64>> {
        if self.pc < 0 || self.pc as usize >= self.instructions.len() {
            return None;
        }
        let mut result = None;
        match self.instructions[self.pc as usize] {
            Instruction::Snd(val) => self.last_sound = self.val(val),
            Instruction::Set(c, val) => *self.reg(c) = self.val(val),
            Instruction::Add(c, val) => *self.reg(c) += self.val(val),
            Instruction::Mul(c, val) => *self.reg(c) *= self.val(val),
            Instruction::Mod(c, val) => *self.reg(c) %= self.val(val),
            Instruction::Rcv(c) => {
                if self.val(Val::Reg(c)) != 0 {
                    result = Some(self.last_sound);
                }
            }
            Instruction::Jgz(val, offset) => {
                let val = self.val(val);
                if val > 0 {
                    self.pc += self.val(offset);
                    self.pc -= 1;
                }
            }
        }
        self.pc += 1;
        Some(result)
    }

    fn val(&mut self, val: Val) -> i64 {
        match val {
            Val::Raw(x) => x,
            Val::Reg(c) => *self.registers.entry(c).or_insert(0),
        }
    }
}

struct Duet2 {
    instructions: Vec<Instruction>,
    registers: HashMap<char, i64>,
    pc: i64,
    send_count: i64,
    queue: Rc<RefCell<VecDeque<i64>>>,
    other_queue: Rc<RefCell<VecDeque<i64>>>,
    id: usize,
    debug: bool,
}

impl Duet2 {
    fn new(
        s: &str,
        queue: Rc<RefCell<VecDeque<i64>>>,
        other_queue: Rc<RefCell<VecDeque<i64>>>,
        id: usize,
    ) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', id as i64);
        Self {
            instructions: s.lines().map(parse_line).collect(),
            registers,
            pc: 0,
            send_count: 0,
            queue,
            other_queue,
            id,
            debug: std::env::args().find(|x| x == "--debug").is_some(),
        }
    }

    fn reg(&mut self, c: char) -> &mut i64 {
        self.registers.entry(c).or_insert(0)
    }

    /// returns true when paused / done
    fn run_single(&mut self) -> bool {
        if self.pc < 0 || self.pc as usize >= self.instructions.len() {
            return true;
        }
        let instruction = self.instructions[self.pc as usize];
        self.display_instruction(instruction);
        match instruction {
            Instruction::Snd(val) => {
                let val = self.val(val);
                self.other_queue.borrow_mut().push_back(val);
                self.send_count += 1;
            }
            Instruction::Set(c, val) => *self.reg(c) = self.val(val),
            Instruction::Add(c, val) => *self.reg(c) += self.val(val),
            Instruction::Mul(c, val) => *self.reg(c) *= self.val(val),
            Instruction::Mod(c, val) => *self.reg(c) %= self.val(val),
            Instruction::Rcv(c) => {
                let rec_value = self.queue.borrow_mut().pop_front();
                match rec_value {
                    Some(x) => *self.reg(c) = x,
                    None => return true,
                }
            }
            Instruction::Jgz(val, offset) => {
                let val = self.val(val);
                if val > 0 {
                    self.pc += self.val(offset);
                    self.pc -= 1;
                }
            }
        }
        self.display_state();
        self.pc += 1;

        false
    }

    fn val(&mut self, val: Val) -> i64 {
        match val {
            Val::Raw(x) => x,
            Val::Reg(c) => *self.registers.entry(c).or_insert(0),
        }
    }

    fn display_instruction(&mut self, instruction: Instruction) {
        if !self.debug {
            return;
        }
        let mut v = |w: Val| match w {
            Val::Raw(x) => format!("{x}"),
            Val::Reg(c) => format!("{c}(={0})", self.val(w)),
        };

        let label = match instruction {
            Instruction::Snd(x) => format!("send {}", v(x)),
            Instruction::Set(x, y) => format!("set {} to {}", x, v(y)),
            Instruction::Add(x, y) => format!("add {} to {}", x, v(y)),
            Instruction::Mul(x, y) => format!("mul {} to {}", x, v(y)),
            Instruction::Mod(x, y) => format!("mod {} to {}", x, v(y)),
            Instruction::Rcv(x) => format!("rcv into {x} ({0:?})", self.queue.borrow().get(0)),
            Instruction::Jgz(x, y) => format!("jgz {} _ {}", v(x), v(y)),
        };

        println!("[{0}] - {1}", self.id, label);
    }

    fn display_state(&mut self) {
        if !self.debug {
            return;
        }
        println!(
            "{0:?}, [{1:?}, {2:?}]",
            self.registers, self.queue, self.other_queue
        )
    }
}

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let mut duet = Duet::new(input);
        let result: i64 = loop {
            let result = duet.run_single();
            match result {
                None => break duet.last_sound,
                Some(result) => match result {
                    Some(sound) => {
                        println!("found sound: {sound}");
                        break sound;
                    }
                    None => continue,
                },
            }
        };
        Ok(format!("{result}"))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let queue_0 = Rc::new(RefCell::new(VecDeque::new()));
        let queue_1 = Rc::new(RefCell::new(VecDeque::new()));

        let mut duet_0 = Duet2::new(input, queue_0.clone(), queue_1.clone(), 0);
        let mut duet_1 = Duet2::new(input, queue_1.clone(), queue_0.clone(), 1);

        loop {
            if duet_0.run_single() && duet_1.run_single() {
                break;
            }
        }

        Ok(format!("{}", duet_1.send_count))
    }
}
