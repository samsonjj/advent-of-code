use std::collections::{HashMap, HashSet};
use std::error::Error;

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, PartialEq, Eq)]
enum CommonDigit {
    Zero,
    One,
    Equal,
}

impl From<char> for CommonDigit {
    fn from(c: char) -> Self {
        match c {
            '1' => Self::One,
            '0' => Self::Zero,
            _ => panic!("invalid char: {}", c),
        }
    }
}

struct CommonDigitMap {
    len: usize,
    hm: HashMap<usize, CommonDigit>,
}

impl CommonDigitMap {
    pub fn new(lines: Vec<&str>) -> Self {
        let len = lines.iter().next().unwrap().len();
        let mut hm = HashMap::new();
        for line in lines {
            for (idx, c) in line.chars().enumerate() {
                let data = hm.entry(idx).or_insert(0);
                *data += match c {
                    '0' => -1,
                    _ => 1,
                };
            }
        }
        let hm = hm
            .into_iter()
            .map(|(idx, count)| {
                (
                    idx,
                    match count {
                        x if x > 0 => CommonDigit::One,
                        x if x == 0 => CommonDigit::Equal,
                        _ => CommonDigit::Zero,
                    },
                )
            })
            .collect();

        Self { len, hm }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn common_char_at(&self, idx: usize) -> CommonDigit {
        if idx >= self.len {
            panic!("index is {}, but len is {}", idx, self.len);
        }
        *self.hm.get(&idx).unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let common_chars = CommonDigitMap::new(INPUT.lines().collect());
    let mut gamma = 0;
    let mut epsilon = 0;
    for idx in 0..common_chars.len() {
        match common_chars.common_char_at(idx) {
            CommonDigit::One | CommonDigit::Equal => {
                gamma += 2i32.pow(common_chars.len().wrapping_sub(idx + 1) as u32)
            }
            CommonDigit::Zero => {
                epsilon += 2i32.pow(common_chars.len().wrapping_sub(idx + 1) as u32)
            }
        }
    }
    println!("{}", gamma * epsilon);

    let mut lines_left: HashSet<&'static str> = INPUT.lines().collect();
    let mut idx = 0;
    while lines_left.len() > 1 {
        let common_chars = CommonDigitMap::new(lines_left.clone().into_iter().collect());
        let keep = match common_chars.common_char_at(idx) {
            CommonDigit::Equal => CommonDigit::One,
            x => x,
        };
        lines_left = lines_left
            .into_iter()
            .filter(|line| CommonDigit::from(line.as_bytes()[idx] as char) == keep)
            .collect();
        idx += 1;
    }
    let oxygen = i32::from_str_radix(lines_left.iter().next().unwrap(), 2).unwrap();

    let mut lines_left: HashSet<&'static str> = INPUT.lines().collect();
    let mut idx = 0;
    while lines_left.len() > 1 {
        let common_chars = CommonDigitMap::new(lines_left.clone().into_iter().collect());
        let keep = match common_chars.common_char_at(idx) {
            CommonDigit::Equal => CommonDigit::Zero,
            CommonDigit::Zero => CommonDigit::One,
            CommonDigit::One => CommonDigit::Zero,
        };
        lines_left = lines_left
            .into_iter()
            .filter(|line| CommonDigit::from(line.as_bytes()[idx] as char) == keep)
            .collect();
        idx += 1;
    }
    let co2 = i32::from_str_radix(lines_left.iter().next().unwrap(), 2).unwrap();

    println!("{}", oxygen * co2);

    Ok(())
}
