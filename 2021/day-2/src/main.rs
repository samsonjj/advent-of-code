use std::error::Error;

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse(s: &str) -> Result<Direction, Box<dyn Error>> {
    let mut parts = s.split_whitespace().rev();
    let num = parts.next().unwrap().parse::<i32>()?;
    Ok(match parts.next().unwrap() {
        "forward" => Direction::Forward(num),
        "down" => Direction::Down(num),
        "up" => Direction::Up(num),
        _ => panic!("invalid direction")
    })
}

const INPUT: &str = include_str!("input.txt");

fn main() {

    let mut horizontal = 0;
    let mut depth = 0;
    INPUT.lines()
        .map(|line| parse(line).unwrap())
        .for_each(|direction| {
            match direction {
                Direction::Down(x) => depth += x,
                Direction::Up(x) => depth -= x,
                Direction::Forward(x) => horizontal += x,
            }
        });
    println!("{}", horizontal * depth);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    INPUT.lines()
        .map(|line| parse(line).unwrap())
        .for_each(|direction| {
            match direction {
                Direction::Down(x) => aim += x,
                Direction::Up(x) => aim -= x,
                Direction::Forward(x) => {
                    horizontal += x;
                    depth += aim * x;
                }
            }
        });
    println!("{}", horizontal * depth);

}
