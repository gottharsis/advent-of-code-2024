use advent_of_code_2024::{Dir, Grid, Loc};
use std::{collections::HashSet, fs};

fn read_input() -> Grid<char> {
    Grid::from_string(&fs::read_to_string("input/day6.txt").unwrap())
}

fn next_guard_position(grid: &Grid<char>, pos: &Loc, mut facing: Dir) -> Option<(Loc, Dir)> {
    for _ in 0..4 {
        let step_pos = grid.step(pos, facing);
        match step_pos {
            None => return None, // the guard left the map
            Some(np) => {
                if *grid.at(&np).unwrap() != '#' {
                    return Some((np, facing));
                }
            }
        }
        facing = Dir::rotate_clockwise(&facing);
    }
    Some((*pos, facing))
}

fn find_guard(grid: &Grid<char>) -> Option<(Loc, Dir)> {
    for (r, row) in grid.iter_rows().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return Some(((r, c), Dir::N));
            }
        }
    }
    None
}

fn part1() -> u32 {
    let mut grid = read_input();
    // find the guard
    let mut guard_loc: Option<(Loc, Dir)> = find_guard(&grid);

    let mut num_visited = 0;
    while let Some((pos, facing)) = &guard_loc {
        let ch = grid.at_mut(pos).unwrap();
        if *ch != 'X' {
            *ch = 'X';
            num_visited += 1;
        }
        let new_loc = next_guard_position(&grid, pos, *facing);
        if let Some((new_pos, _)) = &new_loc {
            if new_pos == pos {
                break;
            }
        }
        guard_loc = new_loc;
    }

    num_visited
}

fn jump_to_next_obstacle(
    grid: &Grid<char>,
    curr_pos: &Loc,
    curr_facing: Dir,
) -> Option<(Loc, Dir)> {
    let mut curr_pos = *curr_pos;
    while let Some(next_pos) = grid.step(&curr_pos, curr_facing) {
        if *grid.at(&next_pos).unwrap() == '#' {
            return Some((curr_pos, curr_facing));
        }
        curr_pos = next_pos;
    }
    None
}

fn gets_stuck_in_loop(grid: &Grid<char>, start_pos: &Loc, start_dir: Dir) -> bool {
    let mut seen: HashSet<(Loc, Dir)> = HashSet::new();
    let mut curr_pos = Some((*start_pos, start_dir));
    while let Some((loc, dir)) = curr_pos {
        if seen.contains(&(loc, dir)) {
            return true;
        }
        seen.insert((loc, dir));
        curr_pos = next_guard_position(grid, &loc, dir)
            .and_then(|(pos, facing)| jump_to_next_obstacle(grid, &pos, facing));
    }
    false
}

fn part2() -> u32 {
    let mut grid = read_input();
    let (mut curr_pos, mut curr_facing) = find_guard(&grid).unwrap();

    loop {
        let next_pos = next_guard_position(&grid, &curr_pos, curr_facing);
        // the guard is already at the edge, nothing we can do
        if next_pos.is_none() {
            break;
        }
        let (next_pos, next_facing) = next_pos.unwrap();
        if *grid.at(&next_pos).unwrap() == '.'{
            *grid.at_mut(&next_pos).unwrap() = '#'; // add an obstacle where none was before
            if gets_stuck_in_loop(&grid, &curr_pos, curr_facing) {
                *grid.at_mut(&next_pos).unwrap() = 'O'; // mark as a valid obstacle position
            } else {
                *grid.at_mut(&next_pos).unwrap() = ','; // remove the obstacle, but mark as
                // visited
            }
        }
        curr_pos = next_pos;
        curr_facing = next_facing;
    }


    grid.iter_rows()
        .flat_map(|r| r.iter())
        .filter(|c| **c == 'O')
        .count() as u32
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
