use core::panic;
use std::cmp::min;
use std::{fs::read_to_string, ops::BitXor, ops::BitXorAssign};

use itertools::Itertools;

#[derive(Clone)]
struct ProgramState {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_ptr: usize,
}

impl ProgramState {
    fn combo_operand(&self, val: u64) -> u64 {
        match val {
            n if (0..=3).contains(&n) => n,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Reserved opcode 7"),
            _ => unreachable!(),
        }
    }
}

fn read_input() -> (ProgramState, Vec<u64>) {
    let input_lines = read_to_string("input/day17.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();

    let register_a = input_lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let register_b = input_lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let register_c = input_lines[2].split_once(": ").unwrap().1.parse().unwrap();

    let instructions = input_lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    (
        ProgramState {
            register_a,
            register_b,
            register_c,
            instruction_ptr: 0,
        },
        instructions,
    )
}

fn simulate(ps: &mut ProgramState, instructions: &[u64]) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    while ps.instruction_ptr < instructions.len() - 1 {
        let opcode = instructions[ps.instruction_ptr];
        let operand = instructions[ps.instruction_ptr + 1];
        let mut did_jump = false;
        match opcode {
            0 => {
                ps.register_a >>= ps.combo_operand(operand);
            }
            1 => {
                ps.register_b.bitxor_assign(operand);
            }
            2 => ps.register_b = ps.combo_operand(operand) % 8,
            3 => {
                if ps.register_a != 0 {
                    ps.instruction_ptr = operand as usize;
                    did_jump = true;
                }
            }
            4 => {
                ps.register_b.bitxor_assign(ps.register_c);
            }
            5 => {
                output.push(ps.combo_operand(operand) % 8);
            }
            6 => {
                ps.register_b = ps.register_a >> ps.combo_operand(operand);
            }
            7 => {
                ps.register_c = ps.register_a >> ps.combo_operand(operand);
            }
            _ => (),
        };
        if !did_jump {
            ps.instruction_ptr += 2;
        }
    }
    output
}

fn part1() -> String {
    let (mut ps, instructions) = read_input();
    simulate(&mut ps, &instructions).iter().join(",")
}

/*
* 2,4,  1,1,  7,5,  1,5,  4,3,  0,3,  5,5,  3,0
*
* B1 = A1 % 8
* B2 = B1 xor 1 = (A1 % 8) xor 1
* C1 = A1 >> B2 = A1 >> (A1 % 8 xor 1)
* B3 = B2 xor 5 = (A1 % 8 xor 1) xor 5 = (A1 % 8) xor 4
* B4 = B3 xor C1 = ((A1 % 8) xor 4) xor (A1 >> (A1 % 8 xor 1))
* A2 = A1 >> 3
* print B4 % 8 = ((A1 % 8) xor 4) xor (A1 >> (A1 % 8 xor 1)) % 8
* loop
*/

fn simulate_fast(value: u64) -> u64 {
    (value % 8)
        .bitxor(4)
        .bitxor((value >> (value % 8).bitxor(1)) % 8)
}

fn backtrack(target: &Vec<u64>, cur: u64, num_correct_outputs: usize, best: &mut u64) {
    if num_correct_outputs == target.len() {
        *best = min(*best, cur);
        return;
    }
    'outer: for i in 0..8 {
        let num = (cur << 3) + i;
        let mut val = num;
        // simulate and try to get the last few digits
        for v in target.iter().skip(target.len() - num_correct_outputs - 1) {
            let output = simulate_fast(val);
            if output != *v {
                continue 'outer;
            }
            val >>= 3;
        }
        backtrack(target, num, num_correct_outputs + 1, best);
    }
}

fn part2() -> u64 {
    let (_, target) = read_input();
    let mut best = u64::MAX;
    backtrack(&target, 0, 0, &mut best);
    if best == u64::MAX {
        println!("Not found");
    }
    best
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
