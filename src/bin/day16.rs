use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

use advent_of_code_2024::{Dir, Grid, Loc};

fn read_input() -> Grid<char> {
    let path = "input/day16.txt";
    Grid::from_string(&read_to_string(path).unwrap())
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Loc,
    heading: Dir,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Grid<char>, start: Loc, end: Loc) -> Option<u32> {
    use Dir::*;
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(State {
        cost: 0,
        position: start,
        heading: E,
    });
    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if curr.position == end {
            return Some(curr.cost);
        }
        visited.insert(curr.position);

        // add turns
        for next_dir in [
            curr.heading.rotate_clockwise(),
            curr.heading.rotate_counterclockwise(),
        ] {
            if let Some(next_pos) = grid.step(&curr.position, next_dir) {
                if visited.contains(&next_pos) {
                    continue;
                }
                if grid[&next_pos] != '#' {
                    pq.push(State {
                        cost: curr.cost + 1001,
                        position: next_pos,
                        heading: next_dir,
                    });
                }
            }
        }

        // add step forward
        if let Some(next_pos) = grid.step(&curr.position, curr.heading) {
            if visited.contains(&next_pos) {
                continue;
            }
            if grid[&next_pos] != '#' {
                pq.push(State {
                    cost: curr.cost + 1,
                    position: next_pos,
                    heading: curr.heading,
                });
            }
        }
    }

    None
}

fn part1() -> u32 {
    let grid = read_input();
    let start = grid.find_item(&'S').expect("No start found");
    let end = grid.find_item(&'E').expect("No end found");

    dijkstra(&grid, start, end).expect("No solution found!")
}

#[derive(Debug)]
struct Parent {
    cost: u32,
    parents: Vec<(Loc, Dir)>,
}

impl Default for Parent {
    fn default() -> Self {
        Parent {
            cost: u32::MAX,
            parents: Vec::new(),
        }
    }
}

impl Parent {
    // returns true if the proposed path is better or equal to existing paths
    fn update(&mut self, from: (Loc, Dir), cost: u32) -> bool {
        use std::cmp::Ordering::*;
        match self.cost.cmp(&cost) {
            Less => return false,
            Equal => {
                self.parents.push(from)
            }
            Greater => {
                self.cost = cost;
                self.parents = vec![from];
            }
        };
        true
    }
}

fn dijkstra2(
    grid: &Grid<char>,
    start: Loc,
    end: Loc,
    min_path_cost: u32,
) -> HashMap<(Loc, Dir), Parent> {
    let mut pq = BinaryHeap::new();
    let mut parents: HashMap<(Loc, Dir), Parent> = HashMap::new();
    let mut visited = HashSet::new();

    pq.push(State {
        cost: 0,
        position: start,
        heading: Dir::E,
    });
    parents
        .entry((start, Dir::E))
        .or_default()
        .update((start, Dir::E), 0);
    while !pq.is_empty() {
        let curr = pq.pop().unwrap();
        if curr.cost > min_path_cost {
            break;
        }
        if visited.contains(&(curr.position, curr.heading)) {
            continue;
        }
        visited.insert((curr.position, curr.heading));

        if curr.position == end {
            continue;
        }

        // add turns
        for next_dir in [
            curr.heading.rotate_clockwise(),
            curr.heading.rotate_counterclockwise(),
        ] {
            let next_cost = curr.cost + 1000;
            let is_valid_cost = parents
                .entry((curr.position, next_dir))
                .or_default()
                .update((curr.position, curr.heading), next_cost);
            if is_valid_cost && !visited.contains(&(curr.position, next_dir)) {
                pq.push(State {
                    position: curr.position,
                    heading: next_dir,
                    cost: next_cost,
                });
            }
        }
        // add forward
        if let Some(next_pos) = grid.step(&curr.position, curr.heading) {
            if grid[&next_pos] != '#' {
                let next_cost = curr.cost + 1;
                let is_valid_cost = parents
                    .entry((next_pos, curr.heading))
                    .or_default()
                    .update((curr.position, curr.heading), next_cost);

                if is_valid_cost && !visited.contains(&(next_pos, curr.heading)) {
                    pq.push(State {
                        position: next_pos,
                        heading: curr.heading,
                        cost: next_cost,
                    });
                }
            }
        }
    }
    parents
}

fn reconstruct_all_paths(
    visited: &mut HashSet<(Loc, Dir)>,
    parents: &HashMap<(Loc, Dir), Parent>,
    curr: (Loc, Dir),
) {
    if visited.contains(&curr) {
        return;
    }
    visited.insert(curr);
    let values = parents.get(&curr).unwrap();
    if values.cost == 0 {
        return; // we are at the start
    }
    for p in values.parents.iter() {
        if !visited.contains(p) {
            reconstruct_all_paths(visited, parents, *p);
        }
    }
}

fn part2(min_path_cost: u32) -> usize {
    let grid = read_input();
    let start = grid.find_item(&'S').unwrap();
    let end = grid.find_item(&'E').unwrap();
    let parents = dijkstra2(&grid, start, end, min_path_cost);
    let mut shortest_path_items = HashSet::new();
    for dir in [Dir::N, Dir::E, Dir::W, Dir::S] {
        if parents.contains_key(&(end, dir)) {
            reconstruct_all_paths(&mut shortest_path_items, &parents, (end, dir));
        }
    }
    let shortest_path_items = shortest_path_items
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();

    shortest_path_items.len()
}

fn main() {
    let min_path_cost = part1();
    println!("Part 1: {}", min_path_cost);
    println!("Part 2: {}", part2(min_path_cost));
}
