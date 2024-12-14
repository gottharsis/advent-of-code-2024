use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string};

use advent_of_code_2024::{Dir, Grid, Loc};

fn read_input() -> Grid<char> {
    Grid::from_string(&read_to_string("input/day12.txt").unwrap())
}

struct GardenRegion {
    area: u64,
    perimeter: u64,
}

fn bfs(start: Loc, grid: &Grid<char>, visited: &mut Grid<bool>) -> GardenRegion {
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;
    let ch = grid[&start];

    queue.push_back(start);
    visited[&start] = true;

    while let Some(l) = queue.pop_front() {
        area += 1;
        for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
            match grid.step(&l, dir) {
                None => perimeter += 1, // we have reached the edge of the grid
                Some(next) => {
                    if grid[&next] == ch {
                        if visited[&next] {
                            continue;
                        }
                        queue.push_back(next);
                        visited[&next] = true;
                    } else {
                        perimeter += 1;
                    }
                }
            }
        }
    }

    GardenRegion { area, perimeter }
}

fn part1() -> u64 {
    let grid = read_input();
    let mut visited = Grid::new(grid.n_rows(), grid.n_cols(), false);

    let mut score = 0u64;

    for r in 0..grid.n_rows() {
        for c in 0..grid.n_cols() {
            if !visited[&(r, c)] {
                let region = bfs((r, c), &grid, &mut visited);
                score += region.area * region.perimeter;
            }
        }
    }
    score
}

// different, same, same
const INTERIOR_CORNER_PATTERNS: [(Dir, Dir, Dir); 4] = [
    (Dir::NE, Dir::N, Dir::E),
    (Dir::SE, Dir::S, Dir::E),
    (Dir::NW, Dir::N, Dir::W),
    (Dir::SW, Dir::S, Dir::W),
];

fn is_different_in_direction(grid: &Grid<char>, loc: Loc, dir: Dir) -> bool {
    !is_same_in_direction(grid, loc, dir)
}

fn is_same_in_direction(grid: &Grid<char>, loc: Loc, dir: Dir) -> bool {
    grid.step(&loc, dir)
        .is_some_and(|new_loc| grid[&new_loc] == grid[&loc])
}

// # sides = # corners
fn exterior_corners(loc: Loc, grid: &Grid<char>) -> u64 {
    use Dir::*;
    let mut corners = 0;
    for (d1, d2) in [N, S].iter().cartesian_product([E, W].iter()) {
        let d1_diff = is_different_in_direction(grid, loc, *d1);
        let d2_diff = is_different_in_direction(grid, loc, *d2);
        if d1_diff && d2_diff {
            corners += 1;
        }
    }
    corners
}

fn interior_corners(loc: Loc, grid: &Grid<char>) -> u64 {
    let mut corners = 0;

    for (diff, same1, same2) in INTERIOR_CORNER_PATTERNS {
        if is_different_in_direction(grid, loc, diff)
            && is_same_in_direction(grid, loc, same1)
            && is_same_in_direction(grid, loc, same2)
        {
            corners += 1;
        }
    }

    corners
}

fn bfs2(start: Loc, grid: &Grid<char>, visited: &mut Grid<bool>) -> u64 {
    let mut stack = Vec::new();

    stack.push(start);
    let mut area = 0;
    let mut num_sides = 0;
    visited[&start] = true;

    while let Some(l) = stack.pop() {
        area += 1;
        num_sides += exterior_corners(l, grid);
        num_sides += interior_corners(l, grid);

        for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if is_same_in_direction(grid, l, dir) {
                let next = grid.step(&l, dir).unwrap();
                if !visited[&next] {
                stack.push(next);
                visited[&next] = true;
                }
            }
        }
    }

    area * num_sides
}

fn part2() -> u64 {
    let grid = read_input();
    let mut visited = Grid::new(grid.n_rows(), grid.n_cols(), false);

    let mut score = 0u64;

    for r in 0..grid.n_rows() {
        for c in 0..grid.n_cols() {
            if !visited[&(r, c)] {
                score += bfs2((r, c), &grid, &mut visited);
            }
        }
    }
    score
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
