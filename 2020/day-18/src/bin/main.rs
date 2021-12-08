enum Opr {
    Add,
    Mul,
}

fn solve_paren(s: &str, precendence: bool) -> i64 {
    let mut non_paren = String::new();
    let mut start = None;
    let mut paren_count = None;
    for (i, c) in s.chars().enumerate() {
        if let Some(st) = start {
            if c == '(' {
                paren_count = Some(paren_count.unwrap() + 1);
            } else if c == ')' {
                paren_count = Some(paren_count.unwrap() - 1);
                if let Some(0) = paren_count {
                    non_paren.push_str(&(solve_paren(&s[st + 1..i], precendence).to_string()[..]));
                    paren_count = None;
                    start = None;
                }
            }
        } else {
            if c == '(' {
                start = Some(i);
                paren_count = Some(1);
                continue;
            } else {
                non_paren.push(c);
            }
        }
    }
    if precendence {
        solve_non_paren(&simplify_mults(&non_paren[..])[..])
    } else {
        solve_non_paren(&non_paren[..])
    }
}

fn simplify_mults(s: &str) -> String {
    s.split(" * ")
        .map(solve_non_paren)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" * ")
}

fn solve_non_paren(s: &str) -> i64 {
    let mut total: Option<i64> = None;
    let mut opr: Option<Opr> = None;

    for token in s.split_whitespace() {
        if let None = total {
            total = Some(token.parse().unwrap());
            continue;
        }
        match token {
            "+" => opr = Some(Opr::Add),
            "*" => opr = Some(Opr::Mul),
            _ => {
                match opr.unwrap() {
                    Opr::Add => total = Some(total.unwrap() + token.parse::<i64>().unwrap()),
                    Opr::Mul => total = Some(total.unwrap() * token.parse::<i64>().unwrap()),
                }
                opr = None;
            }
        }
    }
    return total.unwrap();
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sum: i64 = INPUT.lines().map(|line| solve_paren(line, false)).sum();
    println!("{}", sum);
    let sum: i64 = INPUT.lines().map(|line| solve_paren(line, true)).sum();
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_non_paren_test() {
        assert_eq!(solve_non_paren("1 + 2 * 3"), 9);
    }

    #[test]
    fn simplify_mults_test() {
        assert_eq!(simplify_mults("1 * 2 + 3"), String::from("1 * 5"));
    }

    #[test]
    fn solve_paren_test() {
        assert_eq!(solve_paren("1 + (2 * 3)", true), 7);
        assert_eq!(solve_paren("1 * 2 + 3", false), 5);
    }
}
