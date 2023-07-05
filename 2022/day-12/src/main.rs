#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::str::FromStr;
use std::time;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

struct Node {
    neighbors: Vec<Rc<Node>>,
}

static DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
use core::iter::Map;
use core::slice::Iter;
fn get_adjacent_nodes(
    node: &(usize, usize),
    height_map: &Vec<Vec<usize>>,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let nodex = node.0;
    let nodey = node.1;
    let width = height_map[0].len();
    let height = height_map.len();
    Box::new(
        DIRS.iter()
            .map(move |dir| (nodex as i32 - dir.0, nodey as i32 - dir.1))
            .filter(move |(row, col)| {
                row < &(height as i32) && row >= &0 && col < &(width as i32) && col >= &0
            })
            .map(|(x, y)| (x as usize, y as usize)),
    )
}

fn create_graph(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<usize>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let height_map: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row, col);
                        'a' as usize
                    } else if c == 'E' {
                        end = (row, col);
                        'z' as usize
                    } else {
                        c as usize
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (start, end, height_map)
}

fn can_traverse(height_map: &Vec<Vec<usize>>, from: &(usize, usize), to: &(usize, usize)) -> bool {
    height_map[to.0][to.1] as i32 - height_map[from.0][from.1] as i32 <= 1
}

fn breadth_first_search(
    height_map: &Vec<Vec<usize>>,
    queue: &mut VecDeque<(usize, usize)>,
    explored: &mut HashMap<(usize, usize), usize>,
    end: (usize, usize),
) -> usize {
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if v == end {
            break;
        }
        for node in
            get_adjacent_nodes(&v, &height_map).filter(|node| can_traverse(&height_map, &v, node))
        {
            if !explored.contains_key(&node) {
                explored.insert(node, explored[&v] + 1);
                queue.push_back(node);
            }
        }
    }

    explored[&end]
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let (start, end, height_map) = create_graph(input);

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut explored = HashMap::new();
        explored.insert(start, 0);

        queue.push_back(start);
        let result = breadth_first_search(&height_map, &mut queue, &mut explored, end);

        Ok(format!("{}", result))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let (start, end, height_map) = create_graph(input);

        // init the queues as if there is an extra start node, connected to all 'a'
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut explored = HashMap::new();
        for row in 0..height_map.len() {
            for col in 0..height_map[0].len() {
                if height_map[row][col] == 'a' as usize {
                    queue.push_back((row, col));
                    explored.insert((row, col), 0);
                }
            }
        }

        let result = breadth_first_search(&height_map, &mut queue, &mut explored, end);

        Ok(format!("{}", result))
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
