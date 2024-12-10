use std::collections::VecDeque;
use std::{collections::HashSet, fs::read_to_string};

use advent_of_code_2024::{Dir, Grid, Loc};

fn read_input() -> Grid<u32> {
    Grid(
        read_to_string("input/day10.txt")
            .unwrap()
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|ch| ch.to_digit(10).expect("Could not parse digit"))
                    .collect()
            })
            .collect(),
    )
}

fn count_trails(grid: &Grid<u32>, start_pos: Loc) -> u32 {
    let dirs = [Dir::E, Dir::S, Dir::W, Dir::N];

    let mut queue: VecDeque<(Loc, u32)> = VecDeque::new();
    queue.push_back((start_pos, 0));

    let mut trail_ends = HashSet::new();
    while !queue.is_empty() {
        let (loc, height) = queue.pop_front().unwrap();
        if height == 9 {
            trail_ends.insert(loc);
            continue;
        }

        for dir in dirs {
            if let Some(next_pos) = grid.step(&loc, dir) {
                if grid[&next_pos] == height + 1 {
                    queue.push_back((next_pos, height + 1));
                }
            }
        }
    }

    trail_ends.len() as u32
}

fn part1() -> u32 {
    let grid = read_input();
    grid.iter_with_loc()
        .filter_map(|(pos, height)| {
            if *height == 0 {
                Some(count_trails(&grid, pos))
            } else {
                None
            }
        })
        .sum()
}


fn count_trails_distinct(grid: &Grid<u32>, start_pos: Loc) -> u32 {
    let dirs = [Dir::E, Dir::S, Dir::W, Dir::N];

    let mut queue: VecDeque<(Loc, u32)> = VecDeque::new();
    queue.push_back((start_pos, 0));

    let mut num_trails = 0;
    while !queue.is_empty() {
        let (loc, height) = queue.pop_front().unwrap();
        if height == 9 {
            num_trails += 1;
            continue;
        }

        for dir in dirs {
            if let Some(next_pos) = grid.step(&loc, dir) {
                if grid[&next_pos] == height + 1 {
                    queue.push_back((next_pos, height + 1));
                }
            }
        }
    }

    num_trails
}


fn part2() -> u32 {
    let grid = read_input();
    grid.iter_with_loc()
        .filter_map(|(pos, height)| {
            if *height == 0 {
                Some(count_trails_distinct(&grid, pos))
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
