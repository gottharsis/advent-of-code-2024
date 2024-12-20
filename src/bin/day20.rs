use std::{
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use advent_of_code_2024::{Dir, Grid, Loc};
use itertools::iproduct;

fn read_input() -> Grid<char> {
    Grid::from_string(&read_to_string("input/day20_sample.txt").unwrap())
}

// all possible places reachable from current location BY CHEATING
fn successors_cheat(grid: &Grid<char>, pos: &Loc) -> HashSet<Loc> {
    use Dir::*;
    let mut successors = HashSet::new();
    for d1 in [N, S, E, W] {
        // ensure the first step is into a wall
        if let Some(next_pos) = grid.step(pos, d1) {
            // it's not cheating unless we go through a wall
            if grid[&next_pos] != '#' {
                continue;
            }

            for d2 in [N, S, E, W] {
                // no sense going back and forth
                if d2 == d1.opposite() {
                    continue;
                }
                if let Some(successor_loc) = grid.step(&next_pos, d2) {
                    if grid[&successor_loc] == '.' {
                        successors.insert(successor_loc);
                    }
                }
            }
        }
    }

    successors
}

// run bfs from end to get the valid distance, stop when we reach the start
// returns a hashmap of location -> distance to end, next step on the path
fn get_distances_to_end(grid: &Grid<char>, start: Loc, end: Loc) -> HashMap<Loc, u32> {
    let mut queue = VecDeque::new();
    let mut parent_dist = HashMap::new();

    parent_dist.insert(end, 0);
    queue.push_back(end);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if curr == start {
            break;
        }
        let curr_dist = parent_dist
            .get(&curr)
            .expect("Should have already populated")
            .to_owned();

        for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
            let next_pos = match grid.step(&curr, dir) {
                Some(pos) if grid[&pos] != '#' => pos,
                _ => continue,
            };
            if parent_dist.contains_key(&next_pos) {
                continue;
            }
            parent_dist.insert(next_pos, curr_dist + 1);
            queue.push_back(next_pos);
        }
    }

    parent_dist
}

struct Entry {
    pos: Loc,
    distance_from_start: u32,
}

fn part1() -> u32 {
    let grid = read_input();
    let start = grid.find_item(&'S').unwrap();
    let end = grid.find_item(&'E').unwrap();

    // contains d(i, end) for all i where d(i, end) <= d(start, end)
    let distances = get_distances_to_end(&grid, start, end);
    let distance_no_cheating = distances.get(&start).unwrap().to_owned();

    const SHORTCUT_THRESHOLD: u32 = 100;
    let mut shortcut_count = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(Entry {
        pos: start,
        distance_from_start: 0,
    });

    // do bfs from start to end, until d(start, curr) + 2 >= d(start, end)
    // because at that point there is no reason to cheat
    while !queue.is_empty() {
        let entry = queue.pop_front().unwrap();
        if entry.distance_from_start + 2 >= distance_no_cheating {
            break;
        }

        // check how many valid cheats we have from here
        for cheat_succ in successors_cheat(&grid, &entry.pos) {
            if let Some(d_next_end) = distances.get(&cheat_succ) {
                let total_path_length = entry.distance_from_start + 2 + d_next_end;
                if total_path_length + SHORTCUT_THRESHOLD <= distance_no_cheating {
                    shortcut_count += 1;
                }
            }
        }

        // continue bfs
        for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(next_pos) = grid.step(&entry.pos, d) {
                if grid[&next_pos] == '#' {
                    continue;
                }
                if visited.contains(&next_pos) {
                    continue;
                }
                visited.insert(next_pos);
                queue.push_back(Entry {
                    pos: next_pos,
                    distance_from_start: entry.distance_from_start + 1,
                });
            }
        }
    }

    shortcut_count
}

fn longer_cheat_succ(grid: &Grid<char>, start: Loc, path_length: usize) -> Vec<(Loc, usize)> {
    use Dir::*;

    iproduct!(
        (0..path_length),
        (0..path_length),
        [N, S].into_iter(),
        [E, W].into_iter(),
    )
    .filter(|(dr, dc, _, _)| (1..=path_length).contains(&(dr + dc)))
    .filter_map(|(delta_r, delta_c, dr, dc)| {
        grid.step_n(&start, dr, delta_r)
            .and_then(|pos| grid.step_n(&pos, dc, delta_c))
            .map(|pos| (pos, delta_r + delta_c))
    })
    .filter(|(loc, _)| grid[loc] == '.')
    .collect()
}

fn part2() -> u32 {
    let grid = read_input();
    let start = grid.find_item(&'S').unwrap();
    let end = grid.find_item(&'E').unwrap();

    // contains d(i, end) for all i where d(i, end) <= d(start, end)
    let distances = get_distances_to_end(&grid, start, end);
    let distance_no_cheating = distances.get(&start).unwrap().to_owned();

    const SHORTCUT_THRESHOLD: u32 = 76;
    const CHEAT_LENGTH: usize = 20;
    let mut shortcut_count = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(Entry {
        pos: start,
        distance_from_start: 0,
    });

    // do bfs from start to end, until d(start, curr) + 2 >= d(start, end)
    // because at that point there is no reason to cheat
    while !queue.is_empty() {
        let entry = queue.pop_front().unwrap();
        if entry.distance_from_start >= distance_no_cheating {
            break;
        }

        // check how many valid cheats we have from here
        for (cheat_succ, cheat_length) in longer_cheat_succ(&grid, entry.pos, CHEAT_LENGTH) {
            if let Some(d_next_end) = distances.get(&cheat_succ) {
                let total_path_length =
                    entry.distance_from_start + (cheat_length as u32) + d_next_end;
                if total_path_length + SHORTCUT_THRESHOLD <= distance_no_cheating {
                    shortcut_count += 1;
                }
            }
        }

        // continue bfs
        for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if let Some(next_pos) = grid.step(&entry.pos, d) {
                if grid[&next_pos] == '#' {
                    continue;
                }
                if visited.contains(&next_pos) {
                    continue;
                }
                visited.insert(next_pos);
                queue.push_back(Entry {
                    pos: next_pos,
                    distance_from_start: entry.distance_from_start + 1,
                });
            }
        }
    }

    shortcut_count
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
