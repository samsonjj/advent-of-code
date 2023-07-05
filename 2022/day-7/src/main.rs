#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult, AocSolver};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::rc::Rc;
use std::str::FromStr;
use std::time;

mod backup;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let temp = Temp {};
    temp.execute(INPUT);
}

struct Temp {}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    size: Option<i64>,
    files: Vec<File>,
    child_dirs: Vec<Rc<RefCell<Dir>>>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i64,
}

impl Dir {
    fn new(name: &str) -> Self {
        Dir {
            name: String::from(name),
            size: None,
            files: vec![],
            child_dirs: vec![],
        }
    }
}

/**
 * non-recursive implementation
 */
fn build_dir_structure(input: &str) -> Rc<RefCell<Dir>> {
    let mut lines = input.lines().peekable();
    lines.next();

    let dir = Rc::new(RefCell::new(Dir::new("/")));

    let mut stack = vec![Rc::clone(&dir)];
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => {
                calculate_sizes(&dir);
                return dir;
            }
        };
        if &line[..1] == "$" {
            if &line[..4] == "$ cd" {
                // cd
                let name = &line[5..];

                // if name is "/", return to top level
                if name == "/" {
                    while stack.len() > 1 {
                        stack.pop();
                    }
                    continue;
                }

                if name == ".." {
                    stack.pop();
                    continue;
                }

                let child = dir
                    .borrow()
                    .child_dirs
                    .iter()
                    .filter(|c| c.borrow().name == name)
                    .next()
                    .map(|c| Rc::clone(c));

                // if no child created yet, make one
                let child = match child {
                    Some(c) => c,
                    None => {
                        let child = Rc::new(RefCell::new(Dir::new(name)));
                        stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .child_dirs
                            .push(Rc::clone(&child));
                        child
                    }
                };
                // alter the stack
                stack.push(child);
            } else if &line[..4] == "$ ls" {
                // ls - don't do anything, this will get picked up by the file processor
            }
        } else {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts[0] == "dir" {
                // dir - don't actually have to do anything (?)
            } else {
                // file
                if stack
                    .last()
                    .unwrap()
                    .borrow()
                    .files
                    .iter()
                    .any(|f| f.name == parts[1])
                {
                    // duplicate file
                    continue;
                }
                stack.last().unwrap().borrow_mut().files.push(File {
                    name: String::from(parts[1]),
                    size: parts[0].parse::<i64>().unwrap(),
                });
            }
        }
    }
}

/**
 * Do some kind of depth-first search, calcualting and storing total sizes along the way. Also create a list of directories which have a size at most 100,000.
 */
fn calculate_sizes(dir: &Rc<RefCell<Dir>>) -> i64 {
    let mut sum = 0;
    for child_dir in dir.borrow().child_dirs.iter() {
        sum += calculate_sizes(child_dir);
    }
    for file in dir.borrow().files.iter() {
        sum += file.size;
    }
    dir.borrow_mut().size = Some(sum);

    sum
}

/**
 * Returns a vec of all dirs in the file tree, under the given dir.
 */
fn get_dirs(dir: &Rc<RefCell<Dir>>) -> Vec<Rc<RefCell<Dir>>> {
    let mut dirs = vec![Rc::clone(dir)];
    for child_dir in dir.borrow().child_dirs.iter() {
        dirs.append(&mut get_dirs(child_dir));
    }
    dirs
}

impl AocSolver for Temp {
    fn part_1(&self, input: &str) -> AocResult<String> {
        let root = build_dir_structure(input);
        let answer: i64 = get_dirs(&root)
            .iter()
            .map(|dir| dir.borrow().size.unwrap())
            .filter(|size| size <= &100_000)
            .sum();
        Ok(format!("{}", answer))
    }

    fn part_2(&self, input: &str) -> AocResult<String> {
        let root = build_dir_structure(input);
        let additional_size_needed =
            30_000_000 - (70_000_000 - root.borrow().size.unwrap());

        let dirs = get_dirs(&root);

        let best_file_size = dirs
            .iter()
            .map(|dir| dir.borrow().size.unwrap())
            .filter(|size| size >= &&additional_size_needed)
            .min()
            .unwrap();

        Ok(format!("{}", best_file_size))
    }
}
