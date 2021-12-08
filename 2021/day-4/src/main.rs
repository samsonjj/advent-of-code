#![feature(box_syntax)]
static INPUT: &str = include_str!("input.txt");

use aoc_util::{solve_and_print, AocResult};
use std::collections::HashMap;
use std::str::FromStr;

type Board<T> = [[T; 5]; 5];

#[derive(Clone, Debug)]
pub struct BingoBoard {
    pub numbers: HashMap<i32, (usize, usize)>,
    pub marked: Board<bool>,
}

impl BingoBoard {
    pub fn hit(&mut self, num: i32) {
        if let Some((i, j)) = self.numbers.get(&num) {
            self.marked[*i][*j] = true;
        }
    }

    pub fn won(&self) -> bool {
        return (0..5).into_iter().any(|a| {
            (0..5).into_iter().all(|b| self.marked[a][b])
                || (0..5).into_iter().all(|b| self.marked[b][a])
        });
    }

    pub fn score(&self, num: i32) -> i32 {
        self.numbers
            .iter()
            .filter(|(_, pos)| !self.marked[pos.0][pos.1])
            .map(|(num, _)| num)
            .sum::<i32>()
            * num
    }
}

impl std::str::FromStr for BingoBoard {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: HashMap<i32, (usize, usize)> = HashMap::new();
        for (i, line) in s.lines().enumerate() {
            for (j, token) in line.split_whitespace().enumerate() {
                numbers.insert(token.parse::<i32>()?, (i, j));
            }
        }

        Ok(BingoBoard {
            numbers,
            marked: [[false; 5]; 5],
        })
    }
}

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut parts = input.split("\n\n");
    let numbers: Vec<i32> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let boards: Vec<BingoBoard> = parts.flat_map(BingoBoard::from_str).collect();
    (numbers, boards)
}

fn part_1(input: &str) -> AocResult<Box<i32>> {
    let (numbers, mut boards) = parse_input(input);
    let mut score = None;
    for num in numbers.iter() {
        for ref mut board in boards.iter_mut() {
            board.hit(*num);
            if board.won() {
                score = Some(board.score(*num));
                break;
            }
        }
        if let Some(_) = score {
            break;
        }
    }

    Ok(box score.unwrap())
}

fn part_2(input: &str) -> AocResult<Box<i32>> {
    let (numbers, mut boards) = parse_input(input);
    let mut score = None;

    for num in numbers.iter() {
        if boards.len() > 1 {
            for board in boards.iter_mut() {
                board.hit(*num);
            }
            boards.retain(|board| !board.won());
        } else {
            boards[0].hit(*num);
            if boards[0].won() {
                score = Some(boards[0].score(*num));
                break;
            }
        }
    }

    Ok(box score.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    static EXAMPLE: &'static str = include_str!("example.txt");

    #[test]
    fn won() {
        let mut bingo_board = BingoBoard::from_str(
            "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        )
        .unwrap();

        assert_eq!(bingo_board.won(), false);

        bingo_board.marked[2][0] = true;
        bingo_board.marked[2][1] = true;
        bingo_board.marked[2][2] = true;
        bingo_board.marked[2][3] = true;
        assert_eq!(bingo_board.won(), false);

        bingo_board.marked[2][4] = true;
        assert_eq!(bingo_board.won(), true);
    }

    #[test]
    fn part_1_matches_example() {
        use super::part_1;
        assert_eq!(part_1(EXAMPLE).unwrap(), box 4512);
    }

    #[test]
    fn part_2_matches_example() {
        use super::part_2;
        assert_eq!(part_2(EXAMPLE).unwrap(), box 1924);
    }
}
