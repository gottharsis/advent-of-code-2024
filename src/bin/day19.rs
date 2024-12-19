use std::{collections::HashMap, fs::read_to_string};

fn read_input() -> (Vec<String>, Vec<String>) {
    let input = read_to_string("input/day19.txt").unwrap();
    let mut it = input.lines();
    let patterns = it
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim().to_owned())
        .collect::<Vec<_>>();
    it.next();
    (patterns, it.map(|x| x.to_owned()).collect())
}

fn is_possible_backtrack<T: AsRef<str>>(designs: &[T], target: &str) -> bool {
    if target.is_empty() {
        return true;
    }
    designs.iter().any(|prefix| {
        target.starts_with(prefix.as_ref())
            && is_possible_backtrack(designs, &target[prefix.as_ref().len()..])
    })
}

fn part1() -> usize {
    let (designs, towels) = read_input();
    towels
        .into_iter()
        .filter(|target| is_possible_backtrack(&designs, target))
        .count()
}

fn cnt_backtrack<T: AsRef<str>>(
    cache: &mut HashMap<String, u64>,
    designs: &[T],
    target: &str,
) -> u64 {
    if target.is_empty() {
        return 1;
    }
    if cache.contains_key(target) {
        return *cache.get(target).unwrap();
    }
    let ans = designs
        .iter()
        .map(|prefix| {
            if target.starts_with(prefix.as_ref()) {
                cnt_backtrack(cache, designs, &target[prefix.as_ref().len()..])
            } else {
                0
            }
        })
        .sum();
    cache.insert(target.to_owned(), ans);
    ans
}

fn part2() -> u64 {
    let (designs, towels) = read_input();
    let mut cache = HashMap::new();
    towels
        .into_iter()
        .map(|target| cnt_backtrack(&mut cache, &designs, &target))
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_possible_backtrack() {
        let designs = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert!(is_possible_backtrack(&designs, "brwrr"));
        assert!(is_possible_backtrack(&designs, "bggr"));
    }

    #[test]
    fn test_is_possible_backtrack_filter() {
        let designs = ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let targets = ["gbbr", "ubwu", "brgr", "bbrgwb"];
        let valid_cnt = targets
            .into_iter()
            .filter(|x| is_possible_backtrack(&designs, x))
            .count();
        assert_eq!(valid_cnt, 2);
    }
}
