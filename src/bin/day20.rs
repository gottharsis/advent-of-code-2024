use std::{
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use advent_of_code_2024::{manhattan_distance, Dir, Grid, Loc};
use itertools::iproduct;

fn read_input() -> Grid<char> {
    Grid::from_string(&read_to_string("input/day20.txt").unwrap())
}

fn find_path(grid: &Grid<char>, start: Loc, end: Loc) -> Vec<Loc> {
    use Dir::*;
    let mut path = Vec::new();
    let mut curr = start;
    path.push(curr);

    'outer: while curr != end {
        'inner: for d in [N, S, E, W] {
            if let Some(next) = grid.step(&curr, d) {
                if grid[&next] == '#' {
                    continue 'inner;
                }
                if path.contains(&next) {
                    continue 'inner;
                }
                curr = next;
                path.push(next);
                continue 'outer;
            }
        }
        unreachable!();
    }
    println!("Path generated");
    path
}

fn count_shortcuts(path: &[Loc], max_cheat_length: usize, min_cheat_gain: usize) -> u32 {
    let mut count = 0;
    for (i, cheat_start) in path.iter().enumerate() {
        for (j, cheat_end) in path.iter().enumerate().skip(i + 1) {
            let normal_length = j - i;
            let cheat_length = manhattan_distance(cheat_end, cheat_start);
            let gain = normal_length - cheat_length;
            if gain >= min_cheat_gain && cheat_length <= max_cheat_length {
                count += 1;
            }
        }
    }
    count
}

fn part1() -> u32 {
    let grid = read_input();
    let start = grid.find_item(&'S').unwrap();
    let end = grid.find_item(&'E').unwrap();

    let path = find_path(&grid, start, end);
    count_shortcuts(&path, 2, 100)
}


fn part2() -> u32 {
    let grid = read_input();
    let start = grid.find_item(&'S').unwrap();
    let end = grid.find_item(&'E').unwrap();

    let path = find_path(&grid, start, end);

    count_shortcuts(&path, 20, 100)
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
