
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ConwayCube {
    dims: i32,
    data: HashSet<Vec<i32>>,
}

impl ConwayCube {
    fn parse(s: &str, dims: i32) -> Self {
        let mut data = HashSet::new();
        for (i, line) in s.lines().enumerate() {
            for (j, item) in line.chars().enumerate() {
                let mut coord = vec![i as i32, j as i32];
                while coord.len() < dims as usize {
                    coord.push(0);
                }
                if item == '#' {
                    data.insert(coord);
                }
            }
        }
        Self {
            dims,
            data,
        }
    }

    fn cycle_once(&mut self) {
        let mut counts = HashMap::new();
        let displacements = permute_displacements(self.dims);
        for coord in self.data.iter() {
            if self.data.contains(coord) {
                for d in displacements.iter() {
                    let count = counts.entry(add_vecs(coord, d)).or_insert(0);
                    *count += 1;
                }
            }
        }
        let mut new_data = HashSet::new();
        for (coord, count) in counts.into_iter() {
            if self.data.contains(&coord) {
                if count == 3 || count == 2 {
                    new_data.insert(coord);
                }
            } else {
                if count == 3 {
                    new_data.insert(coord);
                }
            }
        }
        self.data = new_data;
    }
}

fn add_vecs(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..v1.len() {
        result.push(v1[i] + v2[i]);
    }
    result
}

// we use this method to generete all the displacements
fn permute_displacements(dims: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut current = vec![];
    permute_displacements_recursive(dims, &mut current, &mut result);

    result
}

fn permute_displacements_recursive(dims: i32, current: &mut Vec<i32>, data: &mut Vec<Vec<i32>>) {
    if dims == 0 {
        if !current.iter().all(|x| *x == 0) {
            data.push(current.clone());
        }
    } else {
        current.push(-1);
        permute_displacements_recursive(dims-1, current, data);
        current.pop();

        current.push(0);
        permute_displacements_recursive(dims-1, current, data);
        current.pop();

        current.push(1);
        permute_displacements_recursive(dims-1, current, data);
        current.pop();
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut cc = ConwayCube::parse(INPUT, 3);
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    println!("{}", cc.data.len());

    let mut cc = ConwayCube::parse(INPUT, 4);
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    cc.cycle_once();
    println!("{}", cc.data.len());

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_permute() {
        let result = permute_displacements(2);
        assert_eq!(result, vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![0, -1],
            vec![0, 1],
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
        ]);
    }
}