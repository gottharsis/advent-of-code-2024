use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

type Vector2 = (i64, i64);

struct Machine {
    a: Vector2,
    b: Vector2,
    prize: Vector2,
}

fn read_input() -> Vec<Machine> {
    let button_re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    read_to_string("input/day13.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let (l1, l2, l3) = chunk.collect_tuple().unwrap();
            let a_match = button_re.captures(l1).expect("Can't parse line");
            let a_x = a_match
                .get(1)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();
            let a_y = a_match
                .get(2)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();

            let b_match = button_re.captures(l2).expect("Can't parse line");
            let b_x = b_match
                .get(1)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();
            let b_y = b_match
                .get(2)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();

            let prize_match = prize_re.captures(l3).expect("Can't parse line");
            let prize_x = prize_match
                .get(1)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();
            let prize_y = prize_match
                .get(2)
                .expect("Can't parse captured value")
                .as_str()
                .parse()
                .unwrap();

            Machine {
                a: (a_x, a_y),
                b: (b_x, b_y),
                prize: (prize_x, prize_y),
            }
        })
        .collect()
}

// Approach: use cramer's rule to solve the 2x2 linear equation

fn determinant(col1: Vector2, col2: Vector2) -> i64 {
    col1.0 * col2.1 - col2.0 * col1.1
}

fn test_soln(row1: Vector2, row2: Vector2, row_weights: Vector2, target: Vector2) -> bool {
    row1.0 * row_weights.0 + row2.0 * row_weights.1 == target.0
        && row1.1 * row_weights.0 + row2.1 * row_weights.1 == target.1
}

fn find_soln(m: &Machine) -> Option<i64> {
    let det = determinant(m.a, m.b);
    // special case: they are the same line
    if det == 0 {
        println!("Encountered 0 determinant");
        return None;
    }

    let row_weights = (
        determinant(m.prize, m.b) / det,
        determinant(m.a, m.prize) / det,
    );
    if test_soln(m.a, m.b, row_weights, m.prize) {
        Some(3 * row_weights.0 + row_weights.1)
    } else {
        None
    }
}

fn part1() -> i64 {
    read_input().into_iter().filter_map(|m| find_soln(&m)).sum()
}

fn part2() -> i64 {
    read_input().into_iter().filter_map(|mut m| {
        m.prize.0 += 10000000000000;
        m.prize.1 += 10000000000000;
        find_soln(&m)
    }).sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
