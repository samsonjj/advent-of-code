#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Paper {
    pub dots: HashSet<(i32, i32)>,
    pub folds: Vec<Fold>,
    pub current_fold: usize,
}

#[derive(Clone, Debug)]
pub struct Fold {
    pub axis: Axis,
    pub index: i32,
}

impl std::str::FromStr for Fold {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `fold along x=163`
        Ok(Self {
            axis: match &s[11..12] {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("invalid something {}", &s[11..12]),
            },
            index: s[13..].parse::<i32>()?,
        })
    }
}

impl Paper {
    pub fn new(s: &str) -> Self {
        let mut dots = HashSet::new();
        let mut iter = s.split("\n\n");
        for line in iter.next().unwrap().lines() {
            let mut nums = line.split(",").map(|token| token.parse::<i32>().unwrap());
            let coord = (nums.next().unwrap(), nums.next().unwrap());
            dots.insert(coord);
        }
        let folds = iter
            .next()
            .unwrap()
            .lines()
            .map(Fold::from_str)
            .map(|f| f.unwrap())
            .collect::<Vec<Fold>>();
        Self {
            dots,
            folds,
            current_fold: 0,
        }
    }

    pub fn bounds(&self) -> (i32, i32) {
        let mut max = (0, 0);
        for (x, y) in self.dots.iter() {
            if x > &max.0 {
                max.0 = *x;
            }
            if y > &max.1 {
                max.1 = *y;
            }
        }
        (max.0 + 1, max.1 + 1)
    }

    pub fn plane(&self) -> Vec<Vec<bool>> {
        let mut max = (0, 0);
        for (x, y) in self.dots.iter() {
            if x > &max.0 {
                max.0 = *x;
            }
            if y > &max.1 {
                max.1 = *y;
            }
        }
        let mut result = vec![];
        for j in 0..=max.1 {
            let mut row = vec![];
            for i in 0..=max.0 {
                if self.dots.contains(&(i as i32, j as i32)) {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            result.push(row);
        }
        result
    }

    pub fn perform_next_fold(&mut self) -> HashMap<(i32, i32), (i32, i32)> {
        let result = self.fold(self.folds[self.current_fold].clone());
        self.current_fold += 1;
        result
    }

    fn fold(&mut self, fold: Fold) -> HashMap<(i32, i32), (i32, i32)> {
        let mut result = HashSet::new();
        let mut hm = HashMap::new();
        for (x, y) in self.dots.iter() {
            let to_point = match fold.axis {
                Axis::X => {
                    if x > &fold.index {
                        (fold.index - (x - fold.index), *y)
                    } else {
                        (*x, *y)
                    }
                }
                Axis::Y => {
                    if y > &fold.index {
                        (*x, fold.index - (y - fold.index))
                    } else {
                        (*x, *y)
                    }
                }
            };
            result.insert(to_point);
            hm.insert((*x, *y), to_point);
        }
        self.dots = result;
        hm
    }
}

#[derive(Clone, Debug)]
pub enum Axis {
    X,
    Y,
}

pub fn print_paper(paper: &HashSet<(i32, i32)>) {
    let mut max = (0, 0);
    for (x, y) in paper.iter() {
        if x > &max.0 {
            max.0 = *x;
        }
        if y > &max.1 {
            max.1 = *y;
        }
    }
    for j in 0..=max.1 {
        for i in 0..=max.0 {
            if paper.contains(&(i as i32, j as i32)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {
        assert_eq!(2 + 2, 4);
    }
}
