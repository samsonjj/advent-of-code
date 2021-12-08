const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug)]
enum State {
    Floor,
    Empty,
    Occupied,
}

const DISPLACEMENTS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Layout = Vec<Vec<State>>;

fn count_adjacent_occupied(layout: &Layout, x: i32, y: i32) -> i32 {
    let mut sum = 0;
    for d in DISPLACEMENTS.iter() {
        let xx = x + d.0;
        let yy = y + d.1;
        if xx < 0 || xx >= layout[0].len() as i32 || yy < 0 || yy >= layout.len() as i32 {
            continue;
        }
        if matches!(layout[yy as usize][xx as usize], State::Occupied) {
            sum += 1;
        }
    }
    sum
}

fn count_visible_occupied(layout: &Layout, x: i32, y: i32) -> i32 {
    let mut sum = 0;
    for d in DISPLACEMENTS.iter() {
        let mut xx = x + d.0;
        let mut yy = y + d.1;
        while !(xx < 0 || xx >= layout[0].len() as i32 || yy < 0 || yy >= layout.len() as i32) {
            if matches!(layout[yy as usize][xx as usize], State::Occupied) {
                sum += 1;
                break;
            } else if matches!(layout[yy as usize][xx as usize], State::Empty) {
                break;
            }

            xx += d.0;
            yy += d.1;
        }
    }
    sum
}

fn main() {
    let mut layout: Layout = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => panic!("invalid char {}", c),
                })
                .collect()
        })
        .collect();

    let layout_copy = layout.clone();

    loop {
        let mut altered = false;
        let mut new_layout = layout.clone();
        for (y, row) in layout.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                let count = count_adjacent_occupied(&layout, x as i32, y as i32);
                new_layout[y][x] = match state {
                    State::Empty => {
                        if count == 0 {
                            altered = true;
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        if count >= 4 {
                            altered = true;
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    State::Floor => State::Floor,
                }
            }
        }
        if !altered {
            break;
        }
        layout = new_layout.clone();
    }

    let mut sum = 0;
    for row in layout.iter() {
        for state in row.iter() {
            if matches!(state, State::Occupied) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);

    // part 2

    layout = layout_copy.clone();

    loop {
        let mut altered = false;
        let mut new_layout = layout.clone();
        for (y, row) in layout.iter().enumerate() {
            for (x, state) in row.iter().enumerate() {
                let count = count_visible_occupied(&layout, x as i32, y as i32);
                new_layout[y][x] = match state {
                    State::Empty => {
                        if count == 0 {
                            altered = true;
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        if count >= 5 {
                            altered = true;
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    State::Floor => State::Floor,
                }
            }
        }
        if !altered {
            break;
        }
        // if count == 2 {
        //     layout = new_layout.clone();
        //     println!("{:?}", layout);
        //     break;
        // }
        layout = new_layout;
    }

    let mut sum = 0;
    for row in layout.iter() {
        for state in row.iter() {
            if matches!(state, State::Occupied) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}
