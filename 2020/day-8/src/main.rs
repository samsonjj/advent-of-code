use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let tokens = s.trim().split_whitespace().collect::<Vec<&str>>();

        let num = tokens[1].parse::<i32>().unwrap();
        match tokens[0] {
            "nop" => Instruction::Nop(num),
            "acc" => Instruction::Acc(num),
            "jmp" => Instruction::Jmp(num),
            _ => panic!("invalid operation {}", tokens[0]),
        }
    }
}

struct Pcb {
    program_counter: usize,
    instructions: Vec<Instruction>,
    accumulator: i32,
}

impl Pcb {
    fn new() -> Self {
        Self {
            program_counter: 0,
            instructions: vec![],
            accumulator: 0,
        }
    }

    fn run(&mut self) {
        match self.instructions[self.program_counter] {
            Instruction::Nop(_) => {
                self.program_counter += 1;
            }
            Instruction::Acc(x) => {
                self.accumulator += x;
                self.program_counter += 1;
            }
            Instruction::Jmp(x) => {
                self.program_counter = self.program_counter.wrapping_add(x as usize);
            }
        }
    }
}

fn main() {
    let mut pcb: Pcb = Pcb::new();
    for line in INPUT.lines() {
        pcb.instructions.push(Instruction::parse(line));
    }

    // loop and record visited

    let mut visited = HashSet::new();
    loop {
        if visited.contains(&pcb.program_counter) {
            println!("{}", pcb.accumulator);
            break;
        }
        visited.insert(pcb.program_counter);
        pcb.run();
    }

    // test changing each instruction

    let vals_to_test = visited.clone();
    let mut done = false;
    for start in vals_to_test.iter() {
        pcb.program_counter = *start;
        pcb.instructions[*start] = match &pcb.instructions[*start] {
            Instruction::Nop(x) => Instruction::Jmp(*x),
            Instruction::Jmp(x) => Instruction::Nop(*x),
            Instruction::Acc(x) => Instruction::Acc(*x),
        };

        loop {
            if pcb.program_counter == pcb.instructions.len() {
                done = true;
                break;
            }
            pcb.run();
            if visited.contains(&pcb.program_counter) {
                break;
            }
            visited.insert(pcb.program_counter);
        }

        if done {
            break;
        }

        pcb.instructions[*start] = match &pcb.instructions[*start] {
            Instruction::Nop(x) => Instruction::Jmp(*x),
            Instruction::Jmp(x) => Instruction::Nop(*x),
            Instruction::Acc(x) => Instruction::Acc(*x),
        };
    }

    // run the program one more time

    pcb.accumulator = 0;
    pcb.program_counter = 0;
    loop {
        if pcb.program_counter == pcb.instructions.len() {
            break;
        }
        pcb.run();
    }
    println!("{}", pcb.accumulator);
}
