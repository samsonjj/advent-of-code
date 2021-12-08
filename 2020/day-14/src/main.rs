use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn mmask(val: u64, mask: u64, flags: u64) -> u64 {
    val ^ (flags & (mask ^ val))
}

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut flags = 0u64;
    let mut mask = 0u64;
    for line in INPUT.lines() {
        let tokens = line.split(" = ").collect::<Vec<&str>>();
        match &line[0..3] {
            "mem" => {
                let address = &(tokens[0])[4..tokens[0].len() - 1].parse::<u64>().unwrap();
                let value = tokens[1].parse::<u64>().unwrap();
                memory.insert(*address, mmask(value, mask, flags));
            }
            "mas" => {
                mask = 0;
                flags = 0;
                for c in tokens[1].chars() {
                    mask = mask << 1;
                    flags = flags << 1;
                    if c == '1' {
                        mask |= 1;
                        flags |= 1;
                    } else if c == '0' {
                        flags |= 1;
                    }
                }
            }
            _ => panic!("invalid line"),
        }
    }
    println!("{}", memory.iter().map(|(_, v)| v).sum::<u64>());

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";

    for line in INPUT.lines() {
        let tokens = line.split(" = ").collect::<Vec<&str>>();
        match &line[0..3] {
            "mem" => {
                let address = &(tokens[0])[4..tokens[0].len() - 1].parse::<u64>().unwrap();
                let value = tokens[1].parse::<u64>().unwrap();
                permute(&mut memory, *address, value, 0b1, mask.chars().rev(), 0);
            }
            "mas" => {
                mask = tokens[1];
            }
            _ => panic!("invalid line"),
        }
    }
    println!("{}", memory.iter().map(|(_, v)| v).sum::<u64>());
}

fn permute(
    memory: &mut HashMap<u64, u64>,
    address: u64,
    val: u64,
    mut flag: u64,
    mut mask_chars: std::iter::Rev<std::str::Chars>,
    mut acc: u64,
) {
    loop {
        if flag == 0 {
            flag = 0b1;
        }
        match mask_chars.next() {
            Some('1') => {
                // overwrite with a 1
                acc |= flag;
                flag <<= 1;
            }
            Some('0') => {
                // unchanged (same as address)
                acc |= address & flag;
                flag <<= 1;
            }
            Some('X') => {
                permute(memory, address, val, flag << 1, mask_chars.clone(), acc);
                permute(memory, address, val, flag << 1, mask_chars, acc | flag);
                return;
            }
            None => {
                memory.insert(acc, val);
                return;
            }
            _ => panic!("invalid char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(mmask(0b01010101, 0b00110011, 0b00001111,), 0b01010011);
    }
}
