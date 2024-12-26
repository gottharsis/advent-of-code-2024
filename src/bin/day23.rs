use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use advent_of_code_2024::{bit_set, bitset::BitSet};
use itertools::Itertools;

fn read_input() -> Vec<(String, String)> {
    read_to_string("input/day23.txt")
        .expect("Could not read input file")
        .lines()
        .map(|x| {
            let (a, b) = x.split_once("-").unwrap();
            (a.to_owned(), b.to_owned())
        })
        .collect()
}

type AdjList = HashMap<String, HashSet<String>>;

fn construct_adj_list<T: IntoIterator<Item = (String, String)>>(edges: T) -> AdjList {
    let mut m: AdjList = HashMap::new();

    for (u, v) in edges {
        m.entry(u.clone()).or_default().insert(v.clone());
        m.entry(v.clone()).or_default().insert(u.clone());
    }
    m
}

fn part1() -> usize {
    let edges = read_input();
    let adj_list = construct_adj_list(edges);

    adj_list
        .keys()
        .combinations(3)
        .filter(|vertices| {
            if !vertices.iter().any(|x| x.starts_with('t')) {
                return false;
            }
            vertices.iter().tuple_combinations().all(|(a, b)| {
                adj_list
                    .get(*a)
                    .is_some_and(|neighbors| neighbors.contains(*b))
            })
        })
        .count()
}

type BitAdjList = HashMap<u64, BitSet>;

fn construct_bit_adj_list<T: IntoIterator<Item = (String, String)>>(
    edges: T,
) -> (HashMap<String, u64>, Vec<String>, BitAdjList) {
    let mut adj = BitAdjList::new();
    let mut vert = HashMap::new();
    let mut vert_reverse = Vec::new();
    let mut cur_idx = 0;

    for (us, vs) in edges {
        let u = *vert.entry(us.clone()).or_insert_with(|| {
            let x = cur_idx;
            cur_idx += 1;
            vert_reverse.push(us.clone());
            x
        });

        let v = *vert.entry(vs.clone()).or_insert_with(|| {
            let x = cur_idx;
            cur_idx += 1;
            vert_reverse.push(vs.clone());
            x
        });
        adj.entry(u).or_default().insert(v);
        adj.entry(v).or_default().insert(u);
    }

    (vert, vert_reverse, adj)
}

/** Implement Bron-Kerbosch algorithm to list maximal bit sets */
fn maximum_clique(adj_list: &BitAdjList) -> BitSet {
    let mut best_len = 0;
    let mut best = bit_set!();
    let r = BitSet::new();
    let p: BitSet = (0..(adj_list.len() as u64)).collect();
    let x = BitSet::new();
    let mut stack = VecDeque::new();
    stack.push_back((r, p, x));

    while !stack.is_empty() {
        let (r, mut p, mut x) = stack.pop_back().unwrap();
        //println!("popped r {:?}, p:  {:?} x size {}", r, p, x.len());
        if p.is_empty() && x.is_empty() {
            if r.len() >= best_len {
                best_len = r.len();
                best = r;
            }
            continue;
        }

        for v in p.clone().iter() {
            let nv = adj_list.get(&v).expect("Not present in adj list");
            //println!("v: {}, nv: {:?}", v, nv);
            let mut new_r = r.clone();
            new_r.insert(v);
            let new_p = p.intersection(nv);
            let new_x = x.intersection(nv);
            stack.push_back((new_r, new_p, new_x));
            assert!(p.contains(v));
            p.remove(v);
            x.insert(v);
        }
    }

    best
}

fn neighborhood_excluding_self(adj_list: &BitAdjList, clique: &BitSet) -> BitSet {
    let mut neighborhood = clique.iter().fold(BitSet::new(), |mut acc, v| {
        if let Some(nv) = adj_list.get(&v) {
            acc.intersection_inplace(nv);
        }
        acc
    });
    neighborhood.difference_inplace(clique);
    neighborhood
}

fn part2() -> String {
    let input = read_input();
    let (vert, vert_reverse, adj_list) = construct_bit_adj_list(input);
    println!("adj list size {}", adj_list.len());
    //println!("{:?}", vert);
    let clique = maximum_clique(&adj_list);
    println!("clique size {}", clique.len());
    let mut computers = clique
        .iter()
        .map(|id| vert_reverse[id as usize].clone())
        .collect_vec();
    computers.sort();
    computers.iter().join(",")
}
fn main() {
    //println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
