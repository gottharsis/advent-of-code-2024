use advent_of_code_2024::{Dir, Grid, Loc};
use std::fs;

fn read_input() -> (Grid<char>, Vec<char>) {
    let s = fs::read_to_string("input/day15.txt").expect("Could not read input");
    let (grid_s, moves_s) = s
        .split_once("\n\n")
        .expect("Input did not conform to expected format");
    let grid_s = grid_s.trim();
    let move_s = moves_s.trim();

    (
        Grid::from_string(grid_s),
        move_s.chars().filter(|ch| *ch != '\n').collect(),
    )
}

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';

// tries to shift the object at initial_pos in the given direction
// returns true if the move happened
fn try_move(grid: &mut Grid<char>, initial_pos: Loc, dir: Dir) -> bool {
    if grid[&initial_pos] == WALL {
        return false;
    }
    if grid[&initial_pos] == EMPTY {
        return true; // nothing to do here, but return true to upstream callers
    }
    let next_pos = grid.step(&initial_pos, dir);
    if next_pos.is_none() {
        return false;
    }
    let next_pos = next_pos.unwrap();
    if !try_move(grid, next_pos, dir) {
        return false;
    }
    grid[&next_pos] = grid[&initial_pos];
    grid[&initial_pos] = EMPTY;
    true
}

fn part1() -> u32 {
    let (mut grid, moves) = read_input();
    let mut robot_pos = grid.find_item(&ROBOT).expect("No robot found in input");

    for mv in moves {
        let dir = match mv {
            '^' => Dir::N,
            '>' => Dir::E,
            '<' => Dir::W,
            'v' => Dir::S,
            _ => unreachable!(),
        };
        let did_move = try_move(&mut grid, robot_pos, dir);
        if did_move {
            robot_pos = grid.step(&robot_pos, dir).unwrap();
        }
    }

    let s: usize = grid
        .iter_with_loc()
        .map(|((r, c), ch)| if *ch == BOX { 100 * r + c } else { 0 })
        .sum();
    s as u32
}

// checks if we can move north or south only, use try_move for E and W
fn can_move2(grid: &Grid<char>, initial_pos: Loc, dir: Dir, did_check_partner: bool) -> bool {
    use Dir::*;
    let ch = grid[&initial_pos];
    match ch {
        WALL => return false,
        EMPTY => return true,
        ROBOT => {
            return can_move2(
                grid,
                grid.step(&initial_pos, dir).expect("Assuming no OOB"),
                dir,
                false,
            )
        }
        _ => (),
    };

    if !did_check_partner {
        let partner_loc = match ch {
            BOX_LEFT => grid.step(&initial_pos, E).expect("BOX undefined"),
            BOX_RIGHT => grid.step(&initial_pos, W).expect("BOX undefined"),
            _ => unreachable!(),
        };
        if !can_move2(grid, partner_loc, dir, true) {
            return false;
        }
    }

    can_move2(
        grid,
        grid.step(&initial_pos, dir).expect("No OOB"),
        dir,
        false,
    )
}

// use this after can_move2 returns true
fn do_move2(grid: &mut Grid<char>, initial_pos: Loc, dir: Dir, did_move_partner: bool) {
    use Dir::*;
    let ch = grid[&initial_pos];
    // base case
    if ch == EMPTY {
        return;
    }
    let next_pos = grid.step(&initial_pos, dir).expect("Invalid move step");
    do_move2(grid, next_pos, dir, false);
    if !did_move_partner && (ch == BOX_LEFT || ch == BOX_RIGHT) {
        let partner_loc = match ch {
            BOX_LEFT => grid.step(&initial_pos, E).expect("BOX undefined"),
            BOX_RIGHT => grid.step(&initial_pos, W).expect("BOX undefined"),
            _ => unreachable!(),
        };
        do_move2(grid, partner_loc, dir, true)
    }
    grid[&next_pos] = grid[&initial_pos];
    grid[&initial_pos] = EMPTY;
}

fn try_move2(grid: &mut Grid<char>, initial_pos: Loc, dir: Dir) -> bool {
    use Dir::*;
    match dir {
        E | W => try_move(grid, initial_pos, dir),
        N | S => {
            let can_move = can_move2(grid, initial_pos, dir, false);
            if can_move {
                do_move2(grid, initial_pos, dir, false);
            }
            can_move
        }
        _ => unreachable!(),
    }
}

fn part2() -> usize {
    let (grid, moves) = read_input();
    // widen the grid
    let mut grid = Grid(
        grid.iter_rows()
            .map(|row| {
                row.iter()
                    .flat_map(|ch| match *ch {
                        WALL => ['#', '#'],
                        BOX => [BOX_LEFT, BOX_RIGHT],
                        EMPTY => [EMPTY, EMPTY],
                        ROBOT => [ROBOT, EMPTY],
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    );

    let mut robot_pos = grid.find_item(&ROBOT).expect("No robot found in input");

    for mv in moves {
        let dir = match mv {
            '^' => Dir::N,
            '>' => Dir::E,
            '<' => Dir::W,
            'v' => Dir::S,
            _ => unreachable!(),
        };
        let did_move = try_move2(&mut grid, robot_pos, dir);
        if did_move {
            robot_pos = grid.step(&robot_pos, dir).unwrap();
        }
    }

    grid.iter_with_loc()
        .map(|((r, c), ch)| if *ch == BOX_LEFT { 100 * r + c } else { 0 })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
