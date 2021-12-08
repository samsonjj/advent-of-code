const INPUT: &str = include_str!("input.txt");

fn main() {
    let lines: Vec<&str> = INPUT.split("\n").collect();

    let start_time = lines[0].parse::<i32>().unwrap();

    let mut bus_ids: Vec<i32> = lines[1]
        .split(",")
        .filter(|token| token != &"x")
        .map(|token| token.parse::<i32>().unwrap())
        .collect();

    bus_ids.sort();

    let mut current_time = start_time;
    let mut done = false;
    loop {
        for bus_id in bus_ids.iter() {
            if current_time % bus_id == 0 {
                println!("{}", (current_time - start_time) * bus_id);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
        current_time += 1;
    }

    // part 2

    let lines: Vec<&str> = INPUT.split("\n").collect();

    let bus_ids: Vec<i64> = lines[1]
        .split(",")
        .map(|token| {
            if token == "x" {
                0
            } else {
                token.parse::<i64>().unwrap()
            }
        })
        .collect();

    let mut a: Vec<i64> = vec![];
    let mut n: Vec<i64> = vec![];
    for (i, bus_id) in bus_ids.iter().enumerate() {
        if bus_id == &0 {
            continue;
        }
        n.push(*bus_id);
        a.push((*bus_id - i as i64).rem_euclid(*bus_id) as i64);
    }

    let mut x = a[0];
    let mut acc = n[0];
    for i in 1..a.len() {
        let mut possible_x = x;
        while possible_x.rem_euclid(n[i as usize]) != a[i as usize] {
            possible_x += acc;
        }
        x = possible_x;
        acc = acc * n[i];
    }

    println!("{}", x);
}
