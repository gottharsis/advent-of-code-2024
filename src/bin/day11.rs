use std::{collections::HashMap, fs};

fn read_input() -> Vec<u64> {
    fs::read_to_string("input/day11.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().expect("could not parse number"))
        .collect()
}

const POWERS_OF_10: [u64; 11] = [
    1,
    10,
    100,
    1000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000,
];

fn count_digits(mut n: u64) -> usize {
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn successor(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }
    let digits = count_digits(n);
    if digits % 2 == 0 {
        let splitmod = POWERS_OF_10[digits / 2];
        vec![n / splitmod, n % splitmod]
    } else {
        vec![2024 * n]
    }
}

fn part1() -> usize {
    let mut values = read_input();
    for _blinks in 0..25 {
        values = values.into_iter().flat_map(successor).collect();
    }
    values.len()
}

// recurrence relation: S[value][num_generations] = sum S[succ][num_generations - 1]

fn recurse(n: u64, generation: usize, dp: &mut HashMap<(u64, usize), u64>) -> u64 {
    // check if we've already computed it
    if let Some(val) = dp.get(&(n, generation)) {
        return *val;
    }

    if generation == 0 {
        return 1;
    }

    let s: u64 = successor(n)
        .into_iter()
        .map(|new_val| recurse(new_val, generation - 1, dp))
        .sum();
    dp.insert((n, generation), s);
    s
}

fn part2() -> u64 {
    // use dfs to iterate through this
    let input = read_input();
    const GENERATION : usize = 75;
    let mut dp = HashMap::new();

    let mut sum = 0;
    for n in input {
        sum += recurse(n, GENERATION, &mut dp);
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
