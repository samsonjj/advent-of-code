use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    let mut nums: Vec<i32> = INPUT
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    nums.sort();

    let device = nums[nums.len() - 1] + 3;
    nums.push(device);

    let mut memo: HashMap<i32, i64> = HashMap::new();
    memo.insert(0, 1);
    let mut one_diff_count = 0;
    let mut three_diff_count = 0;
    let mut last = 0i32;

    for num in nums.iter() {
        match num - last {
            1 => one_diff_count += 1,
            3 => three_diff_count += 1,
            _ => {}
        }

        let mut sum = 0;
        for diff in [1, 2, 3].iter() {
            if let Some(val) = memo.get(&(num - diff)) {
                sum += val;
            }
        }
        memo.insert(*num, sum);

        last = *num;
    }

    println!("{}", one_diff_count * three_diff_count);
    println!("{}", memo.get(&device).unwrap());
}
