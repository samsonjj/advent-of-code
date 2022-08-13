#![feature(box_syntax)]
#![allow(dead_code, unused_imports, unused_variables)]
use aoc_util::{solve_and_print, AocResult};
use std::collections::{HashMap, HashSet};
use std::num;
use std::str::FromStr;

static INPUT: &str = include_str!("input.txt");
static EXAMPLE: &str = include_str!("example.txt");

pub struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

const AREA: Area = Area {
    x_min: 150,
    x_max: 193,
    y_min: -136,
    y_max: -86,
};

fn main() {
    solve_and_print(INPUT, box part_1, box part_2);
}

fn part_1(input: &str) -> AocResult<i32> {
    // By the time the max_height is achieved, the x velocity is likely zero.
    // We can then only look at the y_velocity.
    //
    // Notice that if at any step the probe is traveling upwards, then as the
    // prob falls it will cross the same y coordinate. This is because the y
    // velocity values are symmetric. For example, consider a probe travling
    // upwards at an initial y velocity of 5. The velocities at each step will
    // be the following: [5, 4, 3, 2, 1, 0, -1, 2, 3, 4, 5]. The falling
    // velocities will cancel out the upwards velocities, so the probe will
    // visit the same y coordinate twice.
    //
    // We want to find the max height, which assumes a maximum upwards
    // initial y velocity. We know since we are starting at (0, 0) that the
    // probe will touch y=0 on the way down, with a velocity that is the
    // exact negative of the initial velocity (because of the symmetry).
    // We then need to maximize this downwards velocity, and work backwards
    // to find the inital velocity.
    //
    // The lowest y in the target area is y_min, so to maximize the negative
    // velocity, the y velocity after crossing y=0 going downwards should be
    // excactly equal to -y_min.
    //
    // Therefore the initial velocity is the negative of the velocity on the
    // step before. Therefore, intial y_vel = -y_min - 1
    let y_velocity = -AREA.y_min - 1;

    // max height is sum of all y velocities for the positive portion
    // i.e. sum(y_velocity..0) or sum(0..y_velocity)
    // sum of positive numbers from 1 to n is n * (n + 1) / 2
    let max_height = y_velocity * (y_velocity + 1) / 2;
    Ok(max_height)
}

fn part_2(input: &str) -> AocResult<i32> {
    // for each step, we will store the dx and dy values which happen to land
    // in the target areas range on the given step, for that axis.
    let mut dx_step_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut dy_step_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    // probe travels up and then down:
    //
    // ...............#..#............
    // ...........#........#..........
    // ...............................
    // ......#..............#.........
    // ...............................
    // ...............................
    // S....................#.........
    // ...............................
    // ...............................
    // ...............................
    // .....................#.........
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT
    // ....................TTTTTTTTTTT
    // ....................T#TTTTTTTTT
    // ....................TTTTTTTTTTT
    //
    // We know initial velocity is (-AREA.y_min - 1) and final velocity
    // is AREA.y_min. We therefore take the difference to find how many
    // turns after the first (adding one to account for the first) it
    // will take to get there.
    // (-AREA.y_min - 1) - AREA.y_min + 1 = (-2 * AREA.y_min)
    let num_steps_to_calculate = (-2 * AREA.y_min);

    // loop all possible starting dx values
    for dxi in 0..(AREA.x_max + 2000) {
        let mut x = 0;
        let mut dx = dxi;
        // loop through steps
        for step in 0..num_steps_to_calculate {
            x += dx;

            // dx is guaranteed positive for this input
            if dx > 0 {
                dx -= 1;
            }

            if x >= AREA.x_min && x <= AREA.x_max {
                dx_step_map
                    .entry(step)
                    .or_insert(HashSet::new())
                    .insert(dxi);
            }

            if x > AREA.x_max {
                continue;
            }
        }
    }

    // loop all possible starting dy values
    for dyi in (AREA.y_min - 2000)..(-AREA.y_min - 1 + 2000) {
        let mut y = 0;
        let mut dy = dyi;
        // loop through steps
        for step in 0..num_steps_to_calculate {
            y += dy;
            dy -= 1;

            if y >= AREA.y_min && y <= AREA.y_max {
                dy_step_map
                    .entry(step)
                    .or_insert(HashSet::new())
                    .insert(dyi);
            }

            if y < AREA.y_max {
                continue;
            }
        }
    }

    // compute all possible (dx, dy) combinations
    let mut vels: HashSet<(i32, i32)> = HashSet::new();
    for x in dx_step_map {
        let x_set = &x.1;
        if let Some(y_set) = dy_step_map.get(&x.0) {
            for y_val in y_set {
                for x_val in x_set {
                    vels.insert((*x_val, *y_val));
                }
            }
        }
    }

    Ok(vels.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two() {}
}
