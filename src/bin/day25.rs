use itertools::{iproduct, Itertools};
use std::{collections::HashMap, fs::read_to_string};

type Heights = [u32; 5];
enum Schematic {
    Key(Heights),
    Lock(Heights),
}
fn parse_schematic(schematic: Vec<&str>) -> Schematic {
    let is_lock = schematic[0] == "#####";
    let schematic = &schematic[1..6];
    let mut heights = [0; 5];
    for row in schematic {
        for (col, c) in row.char_indices() {
            if c == '#' {
                heights[col] += 1;
            }
        }
    }
    if is_lock {
        Schematic::Lock(heights)
    } else {
        Schematic::Key(heights)
    }
}

fn read_input() -> (Vec<Heights>, Vec<Heights>) {
    read_to_string("input/day25.txt")
        .unwrap()
        .lines()
        .chunks(8)
        .into_iter()
        .map(|chunk| parse_schematic(chunk.collect()))
        .fold((Vec::new(), Vec::new()), |mut acc, schematic| {
            match schematic {
                Schematic::Lock(h) => acc.0.push(h),
                Schematic::Key(h) => acc.1.push(h),
            };
            acc
        })
}

fn fits(lock: &Heights, key: &Heights) -> bool {
    lock.iter().zip(key.iter()).all(|(a, b)| *a + *b <= 5)
}

fn part_1() -> usize {
    let (locks, keys) = read_input();
    iproduct!(locks, keys).filter(|(a, b)| fits(a, b)).count()
}

fn main() {
    println!("Part 1: {}", part_1())
}
