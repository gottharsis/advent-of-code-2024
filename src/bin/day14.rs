use advent_of_code_2024::{vec2::Vec2, Grid};
use itertools::Itertools;
use std::{fs::read_to_string, ops::Mul};

type Num = i32;

struct Robot {
    position: Vec2<Num>,
    velocity: Vec2<Num>,
}

impl Robot {
    fn step_inplace(&mut self, n_steps: Num) -> &Self {
        self.position += self.velocity * n_steps;
        self
    }

    fn step(&self, n_steps: Num) -> Self {
        Robot {
            position: self.position + self.velocity * n_steps,
            velocity: self.velocity,
        }
    }
}

fn _extract(s: &str) -> Option<Vec2<Num>> {
    let (x_str, y_str) = s.split_once("=")?.1.split_once(",")?;
    let x = x_str.parse::<Num>().ok()?;
    let y = y_str.parse::<Num>().ok()?;
    Some(Vec2::new(x, y))
}

fn read_input() -> Vec<Robot> {
    read_to_string("input/day14.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (p_str, v_str) = line.split_once(" ").expect("could not parse line");
            let p = _extract(p_str).expect("Could not parse point");
            let v = _extract(v_str).expect("Could not parse velocity");
            Robot {
                position: p,
                velocity: v,
            }
        })
        .collect()
}

fn part1() -> i32 {
    let robots = read_input();
    let robots = robots.into_iter().map(|r| {
        let mut robot = r.step(100);
        robot.position.x = robot.position.x.rem_euclid(101);
        robot.position.y = robot.position.y.rem_euclid(103);
        robot
    });

    let quadrant_counts = robots.fold([0, 0, 0, 0], |mut acc, r| {
        // ignore the middle row & col
        if r.position.x == 50 || r.position.y == 51 {
            return acc;
        }

        let top = (r.position.y < 51) as usize;
        let left = (r.position.x < 50) as usize;
        let idx: usize = top << 1 | left;

        acc[idx] += 1;
        acc
    });

    quadrant_counts.into_iter().product::<i32>()
}

fn part2() -> i32 {
    // search for 10 robots in a horizontal line to represent the "christmas tree"
    // pattern should repeat every 101 * 103 time steps, so we need only check that much

    let mut robots = read_input();
    let mut shape = Grid::new(103, 101, '.');

    for time in 1..101 * 103 {
        shape.set_all('.');
        robots.iter_mut().for_each(|robot| {
            robot.step_inplace(1);
            let c = robot.position.x.rem_euclid(101).try_into().expect("Invalid index");
            let r = robot.position.y.rem_euclid(103).try_into().expect("Invalid index");
            shape[&(r, c)] = '*';
        });

        let has_tree = shape
            .iter_rows()
            .map(|row| row.iter().collect::<String>())
            .any(|s| s.contains("**********"));
        if has_tree {
            //for row in shape.iter_rows() {
            //    println!("{}", row.iter().collect::<String>());
            //}
            return time;
        }
    }

    -1
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
