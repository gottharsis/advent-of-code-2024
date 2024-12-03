use regex::Regex;
use std::fs;

const INPUT: &str = "input/day3.txt";

fn read_data() -> String {
    fs::read_to_string(INPUT).unwrap()
}

fn part1() -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let data = read_data();
    re.captures_iter(&data)
        .map(|m| {
            let arg1 = m
                .get(1)
                .and_then(|s| s.as_str().parse::<i64>().ok())
                .expect("Unable to parse");
            let arg2 = m
                .get(2)
                .and_then(|s| s.as_str().parse::<i64>().ok())
                .expect("Unable to parse");
            arg1 * arg2
        })
        .sum()
}
fn part2() -> i64 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let data = read_data();

    let mut sum = 0;
    let mut enabled = true;
    for cap in re.captures_iter(&data) {
        let func = cap.get(0).unwrap().as_str();
        if func == "do()" {
            enabled = true; 
            continue;
        } else if func == "don't()"{
            enabled = false; 
            continue;
        } else if enabled {
            let arg1 = cap.get(1).and_then(|s|s.as_str().parse::<i64>().ok()).expect("Unable to parse number");
            let arg2 = cap.get(2).and_then(|s|s.as_str().parse::<i64>().ok()).expect("Unable to parse number");
            sum += arg1 * arg2;
        }
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
