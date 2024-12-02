use std::collections::HashMap;
use std::fs::{self};
use std::iter::zip;

const INPUT: &str = "input/day1.txt";

fn read_data() -> [Vec<i64>; 2] {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let data = fs::read_to_string(INPUT).expect("Unable to read file!");
    for line in data.lines() {
        if let Some((lhs, rhs)) = line.trim().split_once("   ") {
            let left_num = lhs.parse::<i64>().unwrap();
            let right_num = rhs.parse::<i64>().unwrap();
            left.push(left_num);
            right.push(right_num);
        }
    }
    return [left, right];
}

fn part1() -> i64 {
    let [mut left, mut right] = read_data();
    left.sort();
    right.sort();
    zip(left.into_iter(), right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2() -> i64 {
    let [left, right] = read_data();
    let frequencies: HashMap<i64, i64> = right.into_iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val)
            .and_modify(|frq| *frq += 1)
            .or_insert(1);
        map
    });
    left.into_iter().map(|v| if let Some(f) = frequencies.get(&v) { v * f } else {0}).sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
