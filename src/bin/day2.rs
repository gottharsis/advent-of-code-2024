use std::fs::{self};

const INPUT: &str = "input/day2.txt";

fn read_data() -> Vec<Vec<i64>> {
    fs::read_to_string(INPUT)
        .expect("Can't read input file")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid_level(prev: i64, next: i64, is_increasing: bool) -> bool {
    let diff = next - prev;
    if is_increasing {
        diff >= 1 && diff <= 3
    } else {
        diff <= -1 && diff >= -3
    }
}

fn is_safe_report(report: &Vec<i64>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let increasing = report[1] - report[0] > 0;
    report
        .windows(2)
        .flat_map(<&[i64; 2]>::try_from)
        .all(|[a, b]| is_valid_level(*a, *b, increasing))
}

fn part1() -> usize {
    read_data().into_iter().filter(is_safe_report).count()
}

fn is_safe_with_deletion(report: &Vec<i64>) -> bool {
    if report.len() < 2 {
        return true;
    }
    if is_safe_report(report) { return true; }
    // if the first element is wrong
    (0..report.len()).any(|excluded_idx| {
        let excluded = report.iter().copied().enumerate()
            .filter_map(|(idx, val)| { if idx ==  excluded_idx { None } else { Some(val) } }).collect();
        is_safe_report(&excluded)
    })
}

fn part2() -> usize {
    read_data()
        .into_iter()
        .filter(is_safe_with_deletion)
        .count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
