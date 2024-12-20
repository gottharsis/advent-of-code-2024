use std::fs::{self};
use advent_of_code_2024::{Grid,Loc, Dir};

fn read_input() -> Grid<char> {
    let input = "input/day4.txt";
    let input = fs::read_to_string(input).unwrap();
    Grid(input.lines().map(|line| line.chars().collect()).collect())
}
const DIRS: [Dir; 8] = [
    Dir::N,
    Dir::NE,
    Dir::E,
    Dir::SE,
    Dir::S,
    Dir::SW,
    Dir::W,
    Dir::NW,
];


fn check_add(loc: Loc, delta: (i32, i32)) -> Option<Loc> {
    let (r, c) = loc;
    let (dr, dc) = delta;

    if (r == 0 && dr < 0)  || (c == 0 && dc < 0){
        None
    } else {
        let nr = ((r as i32) + dr) as usize;
        let nc = ((c as i32) + dc) as usize;
        Some((nr, nc))
    }
}

fn check_word_in_dir(grid: &Grid<char>, dir: Dir, target: &str, start_pos: Loc) -> bool {
    let mut curr = start_pos;
    let mut word = String::new();

    for _ in 0..target.len() {
        match grid.at(&curr) {
            None => break,
            Some(c) => word.push(*c),
        }
        if let Some(new_pos) = check_add(curr, dir.delta()) {
            curr = new_pos
        } else {
            break;
        }
    }

    word == target
}

fn part1() -> i32 {
    let grid = read_input();
    let mut result = 0;
    for r in 0..grid.n_rows() {
        for c in 0..grid.n_cols() {
            for dir in DIRS {
                if check_word_in_dir(&grid, dir, "XMAS", (r, c)) {
                    result += 1
                }
            }
        }
    }
    result
}

fn part2() -> i32 {
    let grid = read_input();
    let mut result = 0;

    // top left corner of pattern
    for r in 0..grid.n_rows() - 2 {
        for c in 0..grid.n_cols() - 2 {
            if (check_word_in_dir(&grid, Dir::SE, "MAS", (r, c))
                || check_word_in_dir(&grid, Dir::SE, "SAM", (r, c)))
                && (check_word_in_dir(&grid, Dir::SW, "MAS", (r, c + 2))
                    || check_word_in_dir(&grid, Dir::SW, "SAM", (r, c + 2)))
            {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
