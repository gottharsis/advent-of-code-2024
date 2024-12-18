use itertools::Itertools;
use std::collections::HashMap;
use std::{collections::HashSet, fs::read_to_string};

use advent_of_code_2024::{Grid, Loc};

fn read_input() -> Grid<char> {
    Grid::from_string(&read_to_string("input/day8.txt").unwrap())
}

fn get_antenna_locations(grid: &Grid<char>) -> HashMap<char, Vec<Loc>> {
    let mut antenna_locations: HashMap<char, Vec<Loc>> = HashMap::new();
    for (r, row) in grid.iter_rows().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '.' {
                continue;
            }
            antenna_locations.entry(*ch).or_default().push((r, c));
        }
    }
    antenna_locations
}

fn part1() -> usize {
    let grid = read_input();
    let antenna_locations = get_antenna_locations(&grid);
    let mut antinode_locations = HashSet::new();

    for (_, locs) in antenna_locations {
        for (pos1, pos2) in locs.iter().tuple_combinations() {
            let d_r = (pos2.0 as i32) - (pos1.0 as i32);
            let d_c = (pos2.1 as i32) - (pos1.1 as i32);

            // pos1 -> pos2 -> antinode
            let r = (pos2.0 as i32) + d_r;
            let c = (pos2.1 as i32) + d_c;
            if r >= 0 && c >= 0 {
                let r = r as usize;
                let c = c as usize;
                if grid.in_bounds(&(r, c)) {
                    antinode_locations.insert((r, c));
                }
            }

            // antinode -> pos1 -> pos2
            let r = (pos1.0 as i32) - d_r;
            let c = (pos1.1 as i32) - d_c;
            if r >= 0 && c >= 0 {
                let r = r as usize;
                let c = c as usize;
                if grid.in_bounds(&(r, c)) {
                    antinode_locations.insert((r, c));
                }
            }
        }
    }

    antinode_locations.len()
}

fn part2() -> u32 {
    let grid = read_input();
    let antenna_locations = get_antenna_locations(&grid);

    let mut antinodes = Grid::new(grid.n_rows(), grid.n_cols(), false);
    let n_rows = grid.n_rows() as i32;
    let n_cols = grid.n_cols() as i32;

    for (_, locs) in antenna_locations {
        for (pos1, pos2) in locs.iter().tuple_combinations() {
            let dr = (pos2.0 as i32) - (pos1.0 as i32);
            let dc = (pos2.1 as i32) - (pos1.1 as i32);
            //let (dr, dc) = normalize(d_r, d_c);

            // in pos1 -> pos2 direction
            let (mut r, mut c) = (pos1.0 as i32, pos1.1 as i32);
            while (0..n_rows).contains(&r) && (0..n_cols).contains(&c) {
                antinodes[&(r as usize, c as usize)] = true;

                r += dr;
                c += dc;
            }

            // in pos2 -> pos1 direction
            (r, c) = (pos1.0 as i32 - dr, pos1.1 as i32 - dc);
            while (0..n_rows).contains(&r) && (0..n_cols).contains(&c) {
                antinodes[&(r as usize, c as usize)] = true;

                r -= dr;
                c -= dc;
            }
        }
    }

    antinodes.iter_rows().flatten().filter(|x| **x).count() as u32
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
