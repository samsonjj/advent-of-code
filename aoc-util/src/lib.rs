use std::error::Error;
use std::fmt::{Debug, Display};
use std::time;

pub trait DisplayDebug: Display + Debug {}
impl<T: Display + Debug> DisplayDebug for T {}

pub type AocResult<T> = Result<T, Box<dyn Error>>;

pub type AocPartSolver<T> = Box<dyn FnOnce(&str) -> AocResult<T>>;

pub fn solve_and_print<T: DisplayDebug>(
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
