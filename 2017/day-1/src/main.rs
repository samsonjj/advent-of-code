use aoc_util::{solve_and_print, AocResult, AocSolver};

struct Temp {}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.chars().collect::<Vec<char>>();
        let mut sum = 0;
        for i in 0..input.len() {
            if input[i] == input[(i + 1) % input.len()] {
                sum += input[i].to_digit(10).unwrap();
            }
        }

        Ok(format!("{}", sum))
    }

    fn part_2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        let input = input.chars().collect::<Vec<char>>();
        let mut sum = 0;
        for i in 0..input.len() {
            if input[i] == input[(i + input.len() / 2) % input.len()] {
                sum += input[i].to_digit(10).unwrap();
            }
        }

        Ok(format!("{}", sum))
    }
}

fn main() {
    let temp = Temp {};
    temp.execute(include_str!("input.txt"));
}
