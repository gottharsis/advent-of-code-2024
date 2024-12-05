use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

type AdjList = HashMap<u32, HashSet<u32>>;

fn read_input() -> (AdjList, Vec<Vec<u32>>) {
    let input = read_to_string("input/day5.txt").unwrap();
    let mut lines = input.lines();

    // read the adjacency list
    let mut adj_list: AdjList = HashMap::new();
    lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            let (lhs, rhs) = line
                .split_once("|")
                .expect("Line does not match format lhs|rhs");
            let lhs = lhs.parse::<u32>().expect("Unable to parse int");
            let rhs = rhs.parse::<u32>().expect("Unable to parse int");

            adj_list.entry(lhs).or_default().insert(rhs);
        });

    let queries = lines
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().expect("could not parse int"))
                .collect()
        })
        .collect();
    (adj_list, queries)
}

fn validate_top_sort(adj_list: &AdjList, query: &[u32]) -> bool {
    // it is in topological sort if there is no backward edge
    for (idx, u) in query.iter().enumerate() {
        for v in query.iter().skip(idx + 1) {
            if let Some(set) = adj_list.get(v) {
                if set.contains(u) {
                    return false;
                }
            }
        }
    }
    true
}

fn part1() -> u32 {
    let (adj_list, queries) = read_input();
    queries
        .into_iter()
        .map(|query| {
            if validate_top_sort(&adj_list, &query) {
                let idx = query.len() / 2;
                *query.get(idx).unwrap()
            } else {
                0u32
            }
        })
        .sum()
}

fn topsort(adj_list: &AdjList, query: &[u32]) -> Vec<u32> {
    let mut sorted: Vec<u32> = Vec::with_capacity(query.len());
    let mut queue = VecDeque::new();

    let mut in_degree: HashMap<u32, u32> = HashMap::new();

    // compute in-degree for induced subgraph
    for (idx,u )in query.iter().enumerate() {
        for v in query.iter().skip(idx+1) {
            if adj_list.get(u).map(|set| set.contains(v)).unwrap_or(false) {
                *in_degree.entry(*v).or_default() += 1;
            }
            if adj_list.get(v).map(|set| set.contains(u)).unwrap_or(false) {
                *in_degree.entry(*u).or_default() += 1;
            }
        } 
    }

    for key in query.iter() {
        if *in_degree.get(key).unwrap_or(&0) == 0 {
            queue.push_back(*key);
        }
    }

    if queue.is_empty() {
        println!("Queue is empty {:?}, {:?}", in_degree, query)
    }
    while !queue.is_empty() {
        let key = queue.pop_front().unwrap();
        sorted.push(key);
        if let Some(children) = adj_list.get(&key) {
            for child in children.iter() {
                if let Some(deg) = in_degree.get_mut(child) {
                    *deg -= 1; // guaranteed to not overflow since v is a child of u ==> in_degree[v] > 1
                    if *deg == 0 {
                        queue.push_back(*child);
                    }
                }
            }
        }
    }

    if sorted.len() != query.len() {
        println!("Could not top sort, indegree: {:?}", in_degree)
    }
    sorted
}

fn part2() -> u32 {
    let (adj_list, queries) = read_input();
    queries
        .into_iter()
        .map(|query| {
            let sorted = topsort(&adj_list, &query);
            if sorted == query {
                0
            } else {
                *sorted.get(sorted.len() / 2).unwrap()
            }
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
