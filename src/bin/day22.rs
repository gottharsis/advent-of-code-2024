use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

const MODULO: u64 = 16777216;

fn read_input() -> Vec<u64> {
    read_to_string("input/day22.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn next_secret_number(mut secret: u64) -> u64 {
    secret ^= secret * 64;
    secret %= MODULO;

    secret ^= secret / 32;
    secret %= MODULO;

    secret ^= secret * 2048;
    secret % MODULO
}

fn nth_secret_number(start: u64, n: usize) -> u64 {
    let mut s = start;
    for _ in 0..n {
        s = next_secret_number(s);
    }
    s
}

fn part1() -> u64 {
    let input = read_input();
    input.into_iter().map(|x| nth_secret_number(x, 2000)).sum()
}

// returns vector of length n + 1, including the start
fn get_next_secret_numbers(start: u64, n: usize) -> Vec<u64> {
    let mut secret = start;
    let mut result = Vec::new();
    result.push(start);
    for _ in 0..n {
        secret = next_secret_number(secret);
        result.push(secret);
    }
    result
}

fn get_diffs(prices: &Vec<u64>) -> Vec<i32> {
    prices
        .iter()
        .tuple_windows()
        .map(|(prev, next)| ((next % 10) as i32) - ((prev % 10) as i32))
        .collect()
}

fn get_4_windows_with_price(start: u64) -> HashMap<[i32; 4], u64> {
    // 2001 elements
    let prices = get_next_secret_numbers(start, 2000)
        .into_iter()
        .map(|n| n % 10)
        .collect_vec();
    // 2000 elements
    let diffs = get_diffs(&prices);

    // 1st window corresponds to 5th price
    diffs
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .map(|(a, b, c, d)| [*a, *b, *c, *d])
        .zip(prices.iter().skip(4))
        .fold(HashMap::new(), |mut acc, (window, price)| {
            acc.entry(window).or_insert(*price);
            acc
        })
}

// merges b into a and returns the result. Mutates a inplace
fn merge_with<K, V, F>(a: HashMap<K, V>, b: HashMap<K, V>, f: F) -> HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(V, V) -> V,
    V: Clone,
{
    let mut result = a;
    for (key, value) in b {
        result
            .entry(key)
            .and_modify(|existing| *existing = f(existing.clone(), value.clone()))
            .or_insert(value);
    }
    result
}
fn part2() -> u64 {
    let input = read_input();
    *input
        .into_iter()
        .map(get_4_windows_with_price)
        .reduce(|a, b| merge_with(a, b, |v1, v2| v1 + v2))
        .expect("No hashmap present")
        .values()
        .max()
        .expect("Empty max")
}

fn main() {
    println!("Part1 : {}", part1());
    println!("Part2 : {}", part2());
}
