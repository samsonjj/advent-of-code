const INPUT: &str = include_str!("input.txt");

struct Data {
    min: i32,
    max: i32,
    c: char,
    password: String,
}

impl Data {
    fn parse(s: &str) -> Self {
        let tokens = s.split(' ').collect::<Vec<&str>>();

        let min_max_strs = tokens[0].split('-').collect::<Vec<&str>>();

        let min = min_max_strs[0].parse::<i32>().unwrap();
        let max = min_max_strs[1].parse::<i32>().unwrap();

        let c = tokens[1].chars().next().unwrap();

        let password = tokens[2].to_string();

        return Self {
            min,
            max,
            c,
            password,
        };
    }
}

fn main() {
    let mut count = 0;
    for line in INPUT.lines() {
        let data = Data::parse(line);
        let char_count = data
            .password
            .chars()
            .map(|c| c == data.c)
            .filter(|b| *b)
            .count() as i32;
        if char_count >= data.min && char_count <= data.max {
            count += 1
        }
    }
    println!("{}", count);

    let mut count = 0;
    for line in INPUT.lines() {
        let data = Data::parse(line);
        let chars = data.password.chars().collect::<Vec<char>>();
        let mut char_count = 0;
        if chars[data.min as usize - 1] == data.c {
            char_count += 1
        }
        if chars[data.max as usize - 1] == data.c {
            char_count += 1
        }
        if char_count == 1 {
            count += 1
        }
    }
    println!("{}", count);
}
