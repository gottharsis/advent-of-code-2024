use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    mem::swap,
};

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        use Operation::*;
        match self {
            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Gate {
    inputs: [String; 2],
    output: String,
    op: Operation,
}

fn topsort_eval(gates: Vec<Gate>, mut values: HashMap<String, u64>) -> HashMap<String, u64> {
    let gates_by_output = gates
        .into_iter()
        .map(|g| (g.output.clone(), g))
        .collect::<HashMap<_, _>>();
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for g in gates_by_output.values() {
        for p in g.inputs.iter() {
            adj_list
                .entry(p.clone())
                .or_default()
                .push(g.output.clone());
        }
        *in_degree.entry(g.output.clone()).or_default() += 2;
    }

    let mut queue = VecDeque::new();

    for u in values.keys() {
        if let Some(deps) = adj_list.get(u) {
            for v in deps.iter() {
                let in_deg = in_degree.get_mut(v).expect("Sould have in_degree");
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push_back(v.clone());
                }
            }
        }
    }

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        let gate = gates_by_output.get(&next).expect("Nonexsitent gate");
        let (v1, v2) = gate
            .inputs
            .iter()
            .map(|v| *values.get(v).expect("Should have already evaluated"))
            .collect_tuple()
            .unwrap();
        let output = gate.op.apply(v1, v2);
        if let Some(deps) = adj_list.get(&next) {
            for d in deps.iter() {
                let in_deg = in_degree.get_mut(d).expect("Should have in degree");
                *in_deg -= 1;
                if *in_deg == 0 {
                    queue.push_back(d.clone());
                }
            }
        }
        values.insert(next, output);
    }

    values
}

fn read_input() -> (HashMap<String, u64>, Vec<Gate>) {
    let input = read_to_string("input/day24.txt").unwrap();
    let mut iter = input.lines().map(|line| line.trim());
    let mut initial_values = HashMap::new();
    iter.by_ref().take_while(|x| !x.is_empty()).for_each(|l| {
        let (a, b) = l.split_once(": ").unwrap();
        initial_values.insert(a.to_owned(), b.parse().unwrap());
    });

    let gates = iter
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            let (lhs, op, rhs) = input.split_whitespace().collect_tuple().unwrap();
            let op = match op {
                "XOR" => Operation::Xor,
                "AND" => Operation::And,
                "OR" => Operation::Or,
                _ => panic!("Invalid operand"),
            };
            let mut inputs = [lhs.to_string(), rhs.to_string()];
            inputs.sort();
            Gate {
                inputs,
                output: output.to_string(),
                op,
            }
        })
        .collect();

    (initial_values, gates)
}

fn part1() -> u64 {
    let (values, gates) = read_input();

    let final_values = topsort_eval(gates, values);
    let mut final_values_z: Vec<(String, u64)> = final_values
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();

    final_values_z.sort_by_key(|f| f.0.clone());
    final_values_z.reverse();

    //println!("Z values: {:?}", final_values_z);

    final_values_z
        .into_iter()
        .fold(0, |acc, (_, i)| (acc << 1) | i)
}

struct GateMap(HashMap<(String, String, Operation), String>);

impl GateMap {
    fn get(&self, a: &str, b: &str, c: Operation) -> Option<&str> {
        let key = if a <= b {
            (a.to_string(), b.to_string(), c)
        } else {
            (b.to_string(), a.to_string(), c)
        };
        self.0.get(&key).map(|x| x.as_str())
    }

    fn insert(&mut self, mut g: Gate) -> Option<String> {
        let [a, b] = {
            g.inputs.sort();
            g.inputs
        };
        self.0.insert((a, b, g.op), g.output)
    }
}

impl FromIterator<Gate> for GateMap {
    fn from_iter<T: IntoIterator<Item = Gate>>(iter: T) -> Self {
        iter.into_iter().fold(Self(HashMap::new()), |mut acc, g| {
            acc.insert(g);
            acc
        })
    }
}

fn check_wire(
    gates: &GateMap,
    correct: &mut Vec<String>,
    i: u32,
    last_carry: String,
    prev_intermediates: [String; 2],
) -> Option<(String, [String; 2])> {
    use Operation::*;

    let x_wire = format!("x{:02}", i);
    let y_wire = format!("y{:02}", i);
    let z_wire = format!("z{:02}", i);

    let pre_digit = gates.get(&x_wire, &y_wire, Xor).unwrap();
    let pre_carry_1 = gates.get(&x_wire, &y_wire, And).unwrap();
    let digit = gates.get(pre_digit, &last_carry, Xor);

    // current digit does not lead to z -> we are wrong
    if digit != Some(&z_wire) {
        return None;
    }

    // compute carry out
    let pre_carry_2 = gates.get(&last_carry, pre_digit, And)?;
    let carry_out = gates.get(pre_carry_1, pre_carry_2, Or)?;

    // everything that inputs into z_i is correct
    // namely, inputs into carry_in
    correct.extend(prev_intermediates);
    correct.push(last_carry);
    // also the result of x_i ^ y_i
    correct.push(pre_digit.to_string());

    Some((
        carry_out.to_string(),
        [pre_carry_1.to_string(), pre_carry_2.to_string()],
    ))
}

fn latest_correct_wire(gates: &GateMap) -> (u32, Vec<String>) {
    use Operation::*;
    let mut correct = Vec::new();
    let mut prev_intermediates = ["".to_string(), "".to_string()];

    // handle 0th digit, since this has no carry input
    let out_0 = gates.get("x00", "y00", Xor);
    let carry_0 = gates.get("x00", "y00", And);
    assert_eq!(out_0, Some("z00"));
    let mut last_carry = carry_0.expect("No carry output from 0").to_string();

    for i in 1..45 {
        if let Some((carry, intermediates)) =
            check_wire(gates, &mut correct, i, last_carry, prev_intermediates)
        {
            last_carry = carry;
            prev_intermediates = intermediates;
        } else {
            return (i - 1, correct);
        }
    }

    (45, correct)
}

fn part2() -> String {
    let (_, gates) = read_input();
    let mut gates = gates.into_iter().collect();

    let mut swaps = Vec::new();
    let (mut best_wire, mut best_correct) = latest_correct_wire(&gates);

    println!("Initially correct up to wire z{:02}", best_wire);
    // at most 4 swaps
    for _ in 0..4 {
        let keys = gates.0.keys().map(|x| x.to_owned()).collect_vec();
        for (i, j) in keys.iter().tuple_combinations() {
            let res_i = gates.0.get(i).unwrap().clone();
            let res_j = gates.0.get(j).unwrap().clone();

            // don't mess with 0th output
            if res_i == "z00" || res_j == "z00" {
                continue;
            }

            // don't switch if these are already part of the best
            if best_correct.contains(&res_i) || best_correct.contains(&res_j) {
                continue;
            }

            // try swap
            *gates.0.get_mut(i).unwrap() = res_j.clone();
            *gates.0.get_mut(j).unwrap() = res_i.clone();

            let (attempt, attempt_used) = latest_correct_wire(&gates);
            if attempt > best_wire {
                println!(
                    "Found better: swapped {} and {}, {} -> {}",
                    res_i, res_j, best_wire, attempt
                );
                (best_wire, best_correct) = (attempt, attempt_used);
                swaps.push([res_i.clone(), res_j.clone()]);
                break;
            }

            // undo swap
            *gates.0.get_mut(i).unwrap() = res_i.clone();
            *gates.0.get_mut(j).unwrap() = res_j.clone();
        }
    }

    swaps.into_iter().flatten().sorted().join(",")
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
