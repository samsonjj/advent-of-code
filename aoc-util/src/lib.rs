use std::error::Error;
use std::fmt;
use std::time;

pub type AocResult<T> = anyhow::Result<T, Box<dyn Error>>;

pub type AocPartSolver<T> = Box<dyn FnOnce(&str) -> AocResult<T>>;

pub fn solve_and_print<T: fmt::Debug + fmt::Display>(
    input: &str,
    part_1: AocPartSolver<T>,
    part_2: AocPartSolver<T>,
) {
    let start_total = time::Instant::now();
    let start_part1 = time::Instant::now();
    println!(
        "Part 1: {} in {:?}",
        match part_1(input.trim()) {
            Ok(d) => d.to_string(),
            Err(e) => e.to_string(),
        },
        start_part1.elapsed()
    );
    let start_part2 = time::Instant::now();
    println!(
        "Part 2: {} in {:?}",
        match part_2(input.trim()) {
            Ok(d) => d.to_string(),
            Err(e) => e.to_string(),
        },
        start_part2.elapsed()
    );
    println!("Total: {:?}", start_total.elapsed());
}

pub trait AocSolver {
    fn part_1(&self, input: &str) -> Result<String, Box<dyn Error>>;
    fn part_2(&self, input: &str) -> Result<String, Box<dyn Error>>;

    fn execute(&self, input: &str) {
        let start_total = time::Instant::now();
        let start_part1 = time::Instant::now();
        println!(
            "Part 1: {} in {:?}",
            match self.part_1(input) {
                Ok(d) => d.to_string(),
                Err(e) => e.to_string(),
            },
            start_part1.elapsed()
        );
        let start_part2 = time::Instant::now();
        println!(
            "Part 2: {} in {:?}",
            match self.part_2(input) {
                Ok(d) => d.to_string(),
                Err(e) => e.to_string(),
            },
            start_part2.elapsed()
        );
        println!("Total: {:?}", start_total.elapsed());
    }
}
