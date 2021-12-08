use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Range {
    min: i32,
    max: i32,
}

#[derive(Clone, Debug)]
struct TicketClass {
    name: String,
    ranges: Vec<Range>,
}

impl Eq for TicketClass {}

impl std::hash::Hash for TicketClass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for TicketClass {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut classes = vec![];
    let sections = INPUT.split("\n\n").collect::<Vec<&str>>();

    // parse classes
    for line in sections[0].lines() {
        let tokens: Vec<&str> = line.split(": ").collect();
        let ranges: Vec<Range> = tokens[1]
            .split(" or ")
            .map(|s| {
                let mut iter = s.split("-");
                Range {
                    min: iter.next().unwrap().parse::<i32>().unwrap(),
                    max: iter.next().unwrap().parse::<i32>().unwrap(),
                }
            })
            .collect();
        classes.push(TicketClass {
            name: tokens[0].to_string(),
            ranges,
        });
    }

    // parse ticket
    let your_ticket = parse_ticket(sections[1].lines().skip(1).next().unwrap());

    // parse nearby tickets
    let nearby_tickets: Vec<Vec<i32>> = sections[2]
        .lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    let mut valid_tickets = vec![];
    let mut sum = 0;
    for ticket in nearby_tickets {
        let mut result = true;
        for val in &ticket {
            if val_in_ranges(*val, classes.iter().map(|c| &c.ranges).flatten().collect()).is_empty()
            {
                sum += val;
                result = false;
                break;
            }
        }
        if result {
            valid_tickets.push(ticket);
        }
    }

    println!("{}", sum);

    let num_positions = valid_tickets[0].len();
    let mut class_positions = HashMap::new();
    let mut classes_left: HashSet<TicketClass> = classes.clone().into_iter().collect();
    while class_positions.len() < num_positions {
        // test if any class can only be one position
        for class in classes_left.clone() {
            let possible_positions =
                (0..num_positions).filter(|p| !class_positions.contains_key(p));
            let mut record = vec![];
            for position in possible_positions {
                let is_valid_position = valid_tickets
                    .iter()
                    .all(|t| ticket_matches_class_position(&t, &class, &position));
                if is_valid_position {
                    record.push(position);
                }
            }
            if record.len() == 1 {
                class_positions.insert(record[0], class);
            }
        }
    }

    let mut product = 1i64;
    for (position, class) in class_positions.iter() {
        if class.name.starts_with("departure") {
            product *= your_ticket[*position as usize] as i64;
        }
    }
    println!("{}", product);
}

fn val_in_ranges(val: i32, ranges: Vec<&Range>) -> Vec<Range> {
    let mut result = vec![];
    for range in ranges {
        if val_in_range(val, range) {
            result.push(range.clone());
        }
    }
    result
}

fn val_in_range(val: i32, range: &Range) -> bool {
    val >= range.min && val <= range.max
}

fn parse_ticket(s: &str) -> Vec<i32> {
    s.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn ticket_matches_class_position(ticket: &Vec<i32>, class: &TicketClass, position: &usize) -> bool {
    class
        .ranges
        .iter()
        .any(|r| val_in_range(ticket[*position], r))
}
