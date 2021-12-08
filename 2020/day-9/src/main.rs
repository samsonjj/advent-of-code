use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

fn valid(num: i64, hs: &HashSet<i64>) -> bool {
    for val in hs.iter() {
        if hs.contains(&(num - val)) {
            return true;
        }
    }
    false
}

fn main() {
    let mut nums: Vec<i64> = vec![];
    for line in INPUT.lines() {
        nums.push(line.parse::<i64>().unwrap());
    }

    let mut v = VecDeque::new();
    let mut hs = HashSet::new();
    let mut count = 0;
    let mut solution = None;
    for num in nums.iter() {
        if count >= 25 {
            if !valid(*num, &hs) {
                println!("{}", num);
                solution = Some(*num);
                break;
            }
            hs.remove(&v.pop_front().unwrap());
        }
        v.push_back(*num);
        hs.insert(*num);
        count += 1;
    }

    let mut min = 0;
    let mut max = 0;
    let mut contiguous: Option<&[i64]> = None;
    loop {
        let sum: i64 = (&nums[min..max]).iter().sum();
        match sum - solution.unwrap() {
            0 => {
                contiguous = Some(&nums[min..max]);
                break;
            }
            x if x < 0 => {
                max += 1;
                if max == nums.len() {
                    break;
                }
            }
            x if x > 0 => {
                min += 1;
            }
            _ => panic!("ahhh!!!"),
        }

        if max < min {
            max = min;
        }
    }

    println!(
        "{}",
        (contiguous.unwrap()).iter().min().unwrap() + (contiguous.unwrap()).iter().max().unwrap()
    );
}
