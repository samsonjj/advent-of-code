use std::collections::HashMap;

fn main() {
    algorithm(2020);
    algorithm(30_000_000);
}

fn algorithm(loops: i64) {
    let input = vec![5, 1, 9, 18, 13, 8, 0];

    let mut starting_numbers_iter = input.iter();

    let mut hm = HashMap::new();
    let mut said = 0;
    let mut turn_last_said = None;

    for i in 0..loops {
        let next = starting_numbers_iter.next();
        if let Some(val) = next {
            said = *val;
        } else {
            if let Some(val) = turn_last_said {
                said = (i - 1) - val;
            } else {
                said = 0;
            }
        }
        turn_last_said = hm.get(&said).map(i64::to_owned);
        hm.insert(said, i);
    }

    println!("{}", said);
}
