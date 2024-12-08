use std::fs::read_to_string;

fn read_input() -> Vec<(u64, Vec<u64>)> {
    read_to_string("input/day7.txt")
        .expect("Could not read input file")
        .lines()
        .map(|line| {
            let (sum, rest) = line.split_once(": ").unwrap();
            (
                sum.parse().unwrap(),
                rest.split_ascii_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn get_operator_result(nums: &Vec<u64>, bitmask: u32) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let initial_value = nums[0];
    nums.iter()
        .skip(1)
        .fold((initial_value, bitmask), |(curr_val, bitmask), next_val| {
            if bitmask & 1 > 0 {
                (curr_val * next_val, bitmask >> 1)
            } else {
                (curr_val + next_val, bitmask >> 1)
            }
        })
        .0
}

fn part1() -> u64 {
    let cases = read_input();
    cases
        .into_iter()
        .filter_map(|(target, nums)| {
            assert!(nums.len() < 32);
            let num_cases = 1u32 << (nums.len() - 1);
            if (0..num_cases).any(|bitmask| get_operator_result(&nums, bitmask) == target) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

enum Op {
    Add,
    Multiply,
    Concatenate,
}

impl Op {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
            Op::Concatenate => {
                let mut p = 1;
                while p <= rhs {
                    p *= 10;
                }
                p * lhs + rhs
            }
        }
    }
}

// switch to recursive backtracking
fn can_make_target(target: u64, nums: &Vec<u64>) -> bool {
    return can_make_target_helper(target, nums, nums[0], 1);
}

fn can_make_target_helper(target: u64, nums: &Vec<u64>, partial_result: u64, idx: usize) -> bool {
    if idx == nums.len() {
        return partial_result == target;
    }

    for op in [Op::Add, Op::Multiply, Op::Concatenate] {
        let next_partial_result = op.apply(partial_result, nums[idx]);
        if next_partial_result <= target
            && can_make_target_helper(target, nums, next_partial_result, idx + 1)
        {
            return true;
        }
    }
    false
}

fn part2() -> u64 {
    read_input().iter().filter_map(|(target, nums)| {
        if can_make_target(*target, nums) {
            Some(target)
        } else {
            None
        }
    }).sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
