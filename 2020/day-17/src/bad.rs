#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::VecDeque;

const INPUT: &str = include_str!("input.txt");

type VecD<T> = VecDeque<T>;

macro_rules! vecd {
    ($($y:expr),+) => (
        VecDeque::from(vec![$($y),+])
    )
}

trait Relative<T> {
    fn first(&self) -> &T;
    fn last(&self) -> &T;
}

impl<T> Relative<T> for VecDeque<T> {
    fn first(&self) -> &T {
        &self[0]
    }
    fn last(&self) -> &T {
        &self[self.len()-1]
    }
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {x,y,z}
    }
}

impl std::ops::Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Point3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
enum CubeState {
    Active,
    Inactive,
}

const X: CubeState = CubeState::Active;
const O: CubeState = CubeState::Inactive;

impl std::fmt::Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CubeState::Active => "#",
            _ => "."
        })
    }
}

#[derive(Clone, Debug)]
struct ConwayCube {
    size: (i32, i32, i32),
    // index like data[x][y][z]
    data: VecD<VecD<VecD<CubeState>>>,
}

fn inactive_cube(dims: (i32, i32, i32)) -> VecD<VecD<VecD<CubeState>>> {
    (0..dims.0).map(|_| (0..dims.1).map(|_| (0..dims.2).map(|_| CubeState::Inactive).collect()).collect()).collect()
}
fn inactive_slice(dims: (i32, i32)) -> VecD<VecD<CubeState>> {
    (0..dims.0).map(|_| (0..dims.1).map(|_| CubeState::Inactive).collect()).collect()
}
fn inactive_row(dim: i32) -> VecD<CubeState> {
    (0..dim).map(|_| CubeState::Inactive).collect()
}

impl ConwayCube {
    fn parse(s: &str) -> Self {
        let into_state = |x| {
            match x {
                '#' => CubeState::Active,
                _ => CubeState::Inactive,
            }
        };
        let into_state_row = |s: &str| {
            s.chars().map(into_state).collect::<VecD<CubeState>>()
        };
        let slice = s.lines().map(into_state_row).collect::<VecD<VecD<CubeState>>>();

        let data = vecd![
            inactive_slice((slice.len() as i32, slice[0].len() as i32)),
            slice.clone(),
            inactive_slice((slice.len() as i32, slice[0].len() as i32))
        ];
        Self {
            size: (data.len() as i32, data[0].len() as i32, data[0][0].len() as i32),
            data,
        }
    }

    fn new(data: VecD<VecD<CubeState>>) -> Self {
        let lens = (
            data.len(),
            data[0].len(),
        );
        assert_eq!(lens.0, lens.1);
        assert_eq!(lens.0 % 2, 1);
        assert_eq!(lens.1 % 2, 1);

        // must be 3, will have to be adjusted to allow any len
        assert_eq!(lens.0, 3);

        let c = (lens.0 / 2) as i32;
        Self {
            size: (1, lens.0 as i32, lens.1 as i32),
            data: vecd![
                data
            ],
        } 
    }

    fn print(&self) {
        for (x, slice) in self.data.iter().enumerate() {
            println!("x={}", x as i32);
            for row in slice.iter() {
                for val in row.iter() {
                    print!("{}", val);
                }
                println!();
            }
            println!();
        }
    }

    fn turns_into(&self, point: Point3D) -> CubeState {
        let mut count = 0;
        for dx in -1..=1 {
            let x = dx + point.x;
            if x < 0 || x >= self.size.0 {
                continue;
            }

            for dy in -1..=1 {
                let y = dy + point.y;
                if y < 0 || y >= self.size.1 {
                    continue;
                }

                for dz in -1..=1 {
                    let z = dz + point.z;
                    if z < 0 || z >= self.size.2 {
                        continue;
                    }
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }

                    if self.data[x as usize][y as usize][z as usize] == CubeState::Active {
                        count += 1;
                    }
                }
            }
        }

        if self.data[point.x as usize][point.y as usize][point.z as usize] == CubeState::Active {
            if count == 2 || count == 3 {
                return CubeState::Active;
            } 
        } else if count == 3 {
            return CubeState::Active;
        }

        CubeState::Inactive
    }

    fn x_slice_dimensions(&self) -> (i32, i32) {
        (self.size.1, self.size.2)
    }
    fn y_slice_dimensions(&self) -> (i32, i32) {
        (self.size.0, self.size.2)
    }
    fn z_slice_dimensions(&self) -> (i32, i32) {
        (self.size.0, self.size.1)
    }


    // expands the data vectors so that there will definietly be enough space
    // to cycle once more
    pub fn safe_space(&mut self) {
        // check first and last z
        if self.data.iter().any(|slice| slice.iter().any(|row| *row.first() == CubeState::Active)) {
            for slice in self.data.iter_mut() {
                for row in slice.iter_mut() {
                    row.push_front(CubeState::Inactive);
                }
            }
            self.size = (self.size.0, self.size.1, self.size.2 + 1);
        }
        if self.data.iter().any(|slice| slice.iter().any(|row| *row.last() == CubeState::Active)) {
            for slice in self.data.iter_mut() {
                for row in slice.iter_mut() {
                    row.push_back(CubeState::Inactive);
                }
            }
            self.size = (self.size.0, self.size.1, self.size.2 + 1);
        }

        // check first and last y
        if self.data.iter().any(|slice| slice.first().iter().any(|val| *val == CubeState::Active)) {
            for slice in self.data.iter_mut() {
                slice.push_front(inactive_row(self.size.2));
            }
            self.size = (self.size.0, self.size.1 + 1, self.size.2);
        }
        if self.data.iter().any(|slice| slice.last().iter().any(|val| *val == CubeState::Active)) {
            for slice in self.data.iter_mut() {
                slice.push_back(inactive_row(self.size.2));
            }
            self.size = (self.size.0, self.size.1 + 1, self.size.2);
        }

        // check first and last x
        if self.data.first().iter().flatten().any(|x| *x == CubeState::Active) {
            self.data.push_front(inactive_slice(self.x_slice_dimensions()));
            self.size = (self.size.0 + 1, self.size.1, self.size.2);
        }
        if self.data.last().iter().flatten().any(|x| *x == CubeState::Active) {
            self.data.push_back(inactive_slice(self.x_slice_dimensions()));
            self.size = (self.size.0 + 1, self.size.1, self.size.2);
        }
    }

    fn cycle_once(&mut self) {
        self.safe_space();

        let mut new_data = inactive_cube((
            self.size.0,
            self.size.1,
            self.size.2,
        ));

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                for z in 0..self.size.2 {
                    new_data[x as usize][y as usize][z as usize] = self.turns_into(Point3D::new(x,y,z));
                }
            }
        }

        self.data = new_data;
    }
}

fn main() {
    let mut cc = ConwayCube::new(vecd![
        vecd![O, X, O],
        vecd![O, O, X],
        vecd![X, X, X]
    ]);

    let mut cc = ConwayCube::parse(INPUT);

    println!("Cycle=0");
    cc.print();
    println!("Cycle=1");
    cc.cycle_once();
    cc.print();
    println!("Cycle=2");
    cc.cycle_once();
    cc.print();
    println!("Cycle=3");
    cc.cycle_once();
    cc.print();

    let to_int = |&x| match x {
        CubeState::Active => 1,
        _ => 0,
    };
    println!("{}", cc.data.iter().flatten().flatten().map(to_int).sum::<i32>());

    println!("Cycle=4");
    cc.cycle_once();
    cc.print();
    println!("Cycle=5");
    cc.cycle_once();
    cc.print();
    println!("Cycle=6");
    cc.cycle_once();
    cc.print();

    let to_int = |&x| match x {
        CubeState::Active => 1,
        _ => 0,
    };
    println!("{}", cc.data.iter().flatten().flatten().map(to_int).sum::<i32>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_space() {
        let mut cc = ConwayCube::new(vecd![
            vecd![O, X, O],
            vecd![O, O, X],
            vecd![X, X, X]
        ]);
        // call twice, only the first one should have an effect
        cc.safe_space();
        cc.safe_space();
        assert_eq!(cc.data, vecd!(
            inactive_slice((5,5)),
            vecd![
                vecd![O, O, O, O, O],
                vecd![O, O, X, O, O],
                vecd![O, O, O, X, O],
                vecd![O, X, X, X, O],
                vecd![O, O, O, O, O]
            ],
            inactive_slice((5,5))
        ));
    }
}