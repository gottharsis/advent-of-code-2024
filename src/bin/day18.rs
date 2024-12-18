use itertools::Itertools;
use std::{
    collections::BinaryHeap,
    fs::read_to_string,
};

use advent_of_code_2024::{euclidean_distance, Dir, Grid, Loc};

fn read_input() -> Vec<Loc> {
    read_to_string("input/day18.txt")
        .expect("Input file not found")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().expect("Could not parse int"))
                .collect_tuple()
                .expect("Unable to pase line")
        })
        .collect()
}

#[derive(PartialEq)]
struct AStarEntry {
    pos: Loc,
    dist: u32,
    heuristic: f64,
}

impl AStarEntry {
    fn g(&self) -> f64 {
        let dist: f64 = self.dist.into();
        dist + self.heuristic
    }
}

impl PartialOrd for AStarEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.pos == other.pos && self.dist == other.dist {
            return std::cmp::Ordering::Equal;
        }
        other
            .g()
            .partial_cmp(&self.g())
            .or_else(|| self.pos.0.partial_cmp(&other.pos.0))
            .or_else(|| self.pos.1.partial_cmp(&other.pos.1))
            .unwrap_or(std::cmp::Ordering::Less)
    }
}

impl Eq for AStarEntry {}

fn a_star_grid(grid: &Grid<char>, start: Loc, end: Loc) -> Option<u32> {
    use Dir::*;
    let mut pq = BinaryHeap::new();
    pq.push(AStarEntry {
        pos: start,
        dist: 0,
        heuristic: euclidean_distance(start, end),
    });

    let mut visited = Grid::new(grid.n_rows(), grid.n_cols(), false);
    let mut best_dist = Grid::new(grid.n_rows(), grid.n_cols(), u32::MAX);
    best_dist[&start] = 0;
    while !pq.is_empty() {
        let entry = pq.pop().unwrap();
        if entry.pos == end {
            return Some(entry.dist);
        }
        if visited[&entry.pos] {
            continue;
        }
        visited[&entry.pos] = true;
        best_dist[&entry.pos] = entry.dist;

        for dir in [N, E, S, W] {
            if let Some(next_pos) = grid.step(&entry.pos, dir) {
                if grid[&next_pos] == '#' {
                    continue;
                }
                if visited[&next_pos] {
                    continue;
                }
                let next_dist = entry.dist + 1;
                if best_dist[&next_pos] <= next_dist {
                    continue;
                }
                best_dist[&next_pos] = next_dist;
                pq.push(AStarEntry {
                    dist: next_dist,
                    pos: next_pos,
                    heuristic: euclidean_distance(next_pos, end),
                })
            }
        }
    }

    None
}

fn part1() -> u32 {
    let bytes = read_input();
    let mut grid = Grid::new(71, 71, '.');
    bytes.iter().take(1024).for_each(|l| grid[l] = '#');
    a_star_grid(&grid, (0, 0), (70, 70)).unwrap_or(u32::MAX)
}

fn apply_changes(points: &[Loc], n: usize) -> Grid<char> {
    let mut grid = Grid::new(71, 71, '.');
    for ch in points.iter().take(n) {
        grid[ch] = '#';
    }
    grid
}

fn is_valid(grid: &Grid<char>) -> bool {
    a_star_grid(grid, (0, 0), (70, 70)).is_some()
}

fn part2() -> String {
    let input = read_input();
    let num_changes_applied = (1..=input.len()).collect_vec().partition_point(|n| {
        let grid = apply_changes(&input, *n);
        is_valid(&grid)
    });
    let (a, b) = input[num_changes_applied];
    format!("{},{}", a, b)
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
