const INPUT: &str = include_str!("input.txt");

const ANGLES: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

const WAYPOINT_ANGLES: [((i32, i32), (i32, i32)); 4] = [
    ((1, 0), (0, 1)),
    ((0, 1), (-1, 0)),
    ((-1, 0), (0, -1)),
    ((0, -1), (1, 0)),
];

#[derive(Debug)]
enum Command {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    F(i32),
    L(i32),
    R(i32),
}

impl Command {
    fn parse(s: &str) -> Self {
        let c = &s[0..1];
        let n = &s[1..s.len()].parse::<i32>().unwrap();

        match c {
            "N" => Command::N(*n),
            "S" => Command::S(*n),
            "E" => Command::E(*n),
            "W" => Command::W(*n),
            "F" => Command::F(*n),
            "L" => Command::L(*n),
            "R" => Command::R(*n),
            _ => panic!("invalid char"),
        }
    }
}

fn main() {
    let mut current_angle = 0;
    let mut x = 0;
    let mut y = 0;

    for line in INPUT.lines() {
        let command = Command::parse(&line);

        match command {
            Command::N(n) => y += n,
            Command::S(n) => y -= n,
            Command::E(n) => x += n,
            Command::W(n) => x -= n,
            Command::F(n) => {
                x += ANGLES[current_angle % 4].0 * n;
                y += ANGLES[current_angle % 4].1 * n;
            }
            Command::L(n) => {
                current_angle = current_angle.wrapping_sub((n / 90) as usize);
            }
            Command::R(n) => {
                current_angle = current_angle.wrapping_add((n / 90) as usize);
            }
        };
    }
    println!("{}", x.abs() + y.abs());

    // part 2

    let mut x = 0;
    let mut y = 0;
    let mut waypoint = (10, 1);
    for line in INPUT.lines() {
        let command = Command::parse(&line);

        match command {
            Command::N(n) => waypoint.1 += n,
            Command::S(n) => waypoint.1 -= n,
            Command::E(n) => waypoint.0 += n,
            Command::W(n) => waypoint.0 -= n,
            Command::F(n) => {
                x += waypoint.0 * n;
                y += waypoint.1 * n;
            }
            Command::L(n) => {
                let angle = WAYPOINT_ANGLES[((-n / 90).rem_euclid(4)) as usize];
                waypoint = (
                    angle.0 .0 * waypoint.0 + angle.0 .1 * waypoint.1,
                    angle.1 .0 * waypoint.0 + angle.1 .1 * waypoint.1,
                );
            }
            Command::R(n) => {
                let angle = WAYPOINT_ANGLES[((n / 90).rem_euclid(4)) as usize];
                waypoint = (
                    angle.0 .0 * waypoint.0 + angle.0 .1 * waypoint.1,
                    angle.1 .0 * waypoint.0 + angle.1 .1 * waypoint.1,
                );
            }
        };
    }
    println!("{}", x.abs() + y.abs());
}
