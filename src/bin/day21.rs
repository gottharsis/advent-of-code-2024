use advent_of_code_2024::{Dir, Grid, Loc, Vec2};
use itertools::{iproduct, repeat_n, Itertools};
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::sync::OnceLock;

fn read_input() -> Vec<String> {
    read_to_string("input/day21.txt")
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn simulate(keypad: &Grid<char>, start: Loc, sequence: &[char]) -> Option<Vec<char>> {
    let mut output = Vec::new();
    let (mut r, mut c) = start;
    for ch in sequence {
        match *ch {
            '^' => r = r.checked_sub(1)?,
            'v' => r += 1,
            '<' => c = c.checked_sub(1)?,
            '>' => c += 1,
            'A' => (),
            _ => return None,
        }
        match keypad.at(&(r, c)) {
            Some(ch) if *ch != '.' => (),
            _ => return None,
        }
        if *ch == 'A' {
            output.push(*keypad.at(&(r, c))?);
        }
    }
    Some(output)
}

fn numeric_keypad() -> &'static Grid<char> {
    static NUMERIC_KEYPAD_CELL: OnceLock<Grid<char>> = OnceLock::new();
    NUMERIC_KEYPAD_CELL.get_or_init(|| Grid::from_string("789\n456\n123\n.0A"))
}

fn directional_keypad() -> &'static Grid<char> {
    static DIRECTIONAL_KEYPAD_CELL: OnceLock<Grid<char>> = OnceLock::new();
    DIRECTIONAL_KEYPAD_CELL.get_or_init(|| Grid::from_string(".^A\n<v>"))
}

const NUMERIC_START: (usize, usize) = (3, 2);
const DIRECTIONAL_START: (usize, usize) = (0, 2);

fn simulate_all_p1(sequence: &[char]) -> Option<Vec<char>> {
    simulate(directional_keypad(), DIRECTIONAL_START, sequence)
        .and_then(|r1| {
            println!("R1: {:?}", r1.iter().collect::<String>());
            simulate(directional_keypad(), DIRECTIONAL_START, &r1)
        })
        .and_then(|r2| {
            println!("R2: {:?}", r2.iter().collect::<String>());
            simulate(numeric_keypad(), NUMERIC_START, &r2)
        })
}

// get the changes that it would take to print
fn simulate_reverse_single_layer(
    target: &[char],
    keypad: &Grid<char>,
    start: Loc,
) -> Vec<Vec<char>> {
    let mut ptr_location = start;
    let mut results = vec![Vec::new()];
    for ch in target {
        let target_loc = keypad.find_item(ch).unwrap();
        let dr = ptr_location.0.abs_diff(target_loc.0);
        let dir_r = if target_loc.0 < ptr_location.0 {
            '^'
        } else {
            'v'
        };
        let dc = ptr_location.1.abs_diff(target_loc.1);
        let dir_c = if target_loc.1 < ptr_location.1 {
            '<'
        } else {
            '>'
        };
        let path_additions = match (dr, dc) {
            (0, 0) => vec![vec![]],
            (dr, 0) => vec![vec![(dir_r, dr)]],
            (0, dc) => vec![vec![(dir_c, dc)]],
            (dr, dc) => vec![
                vec![(dir_r, dr), (dir_c, dc)],
                vec![(dir_c, dc), (dir_r, dr)],
            ],
        };

        results = iproduct!(results.iter(), path_additions)
            .filter_map(|(original_path, addition)| {
                let mut new_path = original_path.clone();
                let mut intermediate_pos = ptr_location;
                for (ch, cnt) in addition {
                    match ch {
                        '^' => intermediate_pos.0 -= cnt,
                        'v' => intermediate_pos.0 += cnt,
                        '>' => intermediate_pos.1 += cnt,
                        '<' => intermediate_pos.1 -= cnt,
                        _ => unreachable!(),
                    }
                    // if we pass over the blank space this is illegal
                    if keypad[&intermediate_pos] == '.' {
                        return None;
                    }
                    new_path.extend(repeat_n(ch, cnt));
                }
                new_path.push('A');
                Some(new_path)
            })
            .collect();
        ptr_location = target_loc;
    }

    results
}

fn keep_only_shortest_paths(paths: Vec<Vec<char>>) -> Vec<Vec<char>> {
    paths
        .into_iter()
        .fold((vec![], usize::MAX), |(mut paths, best_len), path| {
            let n = path.len();
            match n.cmp(&best_len) {
                std::cmp::Ordering::Greater => (paths, best_len),
                std::cmp::Ordering::Less => (vec![path], n),
                std::cmp::Ordering::Equal => {
                    paths.push(path);
                    (paths, best_len)
                }
            }
        })
        .0
}

fn simulate_all_layers_reverse(target: &[char]) -> Vec<Vec<char>> {
    let r3_input = simulate_reverse_single_layer(target, numeric_keypad(), NUMERIC_START);
    let r3_input = keep_only_shortest_paths(r3_input);

    let r2_input = r3_input
        .into_iter()
        .flat_map(|path| {
            simulate_reverse_single_layer(&path, directional_keypad(), DIRECTIONAL_START)
        })
        .collect();
    let r2_input = keep_only_shortest_paths(r2_input);

    let r1_input = r2_input
        .into_iter()
        .flat_map(|path| {
            simulate_reverse_single_layer(&path, directional_keypad(), DIRECTIONAL_START)
        })
        .collect();
    keep_only_shortest_paths(r1_input)
}

fn part1() -> usize {
    //let input = vec!["319A".to_string()];
    let input = read_input();
    input
        .into_iter()
        .map(|code| {
            let value = code[0..code.len() - 1].parse::<usize>().unwrap();
            let path_length = simulate_all_layers_reverse(&code.chars().collect_vec())
                .first()
                .map(|x| x.len())
                .expect("No paths found!");
            println!("Code {} cost {}", code, path_length);
            value * path_length
        })
        .sum()
}

#[derive(Clone)]
struct State {
    pos: Loc,
    sequence: String,
}

impl State {
    fn up(&self) -> Self {
        let (r, c) = self.pos;
        let mut new_sequence = self.sequence.clone();
        new_sequence.push('^');
        Self {
            pos: (r.checked_sub(1).expect("Called up() on row 0"), c),
            sequence: new_sequence,
        }
    }

    fn down(&self) -> Self {
        let (r, c) = self.pos;
        let mut new_sequence = self.sequence.clone();
        new_sequence.push('v');
        Self {
            pos: (r + 1, c),
            sequence: new_sequence,
        }
    }
    fn left(&self) -> Self {
        let (r, c) = self.pos;
        let mut new_sequence = self.sequence.clone();
        new_sequence.push('<');
        Self {
            pos: (r, c.checked_sub(1).expect("called left() on col 0")),
            sequence: new_sequence,
        }
    }

    fn right(&self) -> Self {
        let (r, c) = self.pos;
        let mut new_sequence = self.sequence.clone();
        new_sequence.push('>');
        Self {
            pos: (r, c + 1),
            sequence: new_sequence,
        }
    }
}

type StartEndLayerMap = HashMap<(Loc, Loc, usize), usize>;

fn get_transition_cost(
    memo: &mut StartEndLayerMap,
    start: Loc,
    end: Loc,
    is_directional: bool,
    layer: usize,
) -> usize {
    if is_directional {
        if let Some(cost) = memo.get(&(start, end, layer)) {
            return *cost;
        }
    }

    let keypad = if is_directional {
        directional_keypad()
    } else {
        numeric_keypad()
    };

    let mut result = usize::MAX;
    let mut queue = VecDeque::new();

    queue.push_back(State {
        pos: start,
        sequence: String::new(),
    });

    while !queue.is_empty() {
        let state = queue.pop_front().unwrap();

        match keypad.at(&state.pos) {
            Some(ch) if *ch == '.' => continue,
            _ => (),
        }

        if state.pos == end {
            let next_layer = if is_directional { layer - 1 } else { layer };
            let mut sequence = state.sequence;
            sequence.push('A');
            result = min(result, get_cost_in_layer(memo, sequence, true, next_layer));
            continue;
        }

        match end.0.cmp(&state.pos.0) {
            std::cmp::Ordering::Less => queue.push_back(state.up()),
            std::cmp::Ordering::Greater => queue.push_back(state.down()),
            _ => (),
        }
        match end.1.cmp(&state.pos.1) {
            std::cmp::Ordering::Less => queue.push_back(state.left()),
            std::cmp::Ordering::Greater => queue.push_back(state.right()),
            _ => (),
        }
    }

    if is_directional {
        memo.insert((start, end, layer), result);
    }
    result
}

fn get_cost_in_layer(
    memo: &mut StartEndLayerMap,
    sequence: String,
    is_directional: bool,
    layer: usize,
) -> usize {
    if layer == 0 {
        return sequence.len();
    }

    let mut start = if is_directional {
        DIRECTIONAL_START
    } else {
        NUMERIC_START
    };
    let keypad = if is_directional {
        directional_keypad()
    } else {
        numeric_keypad()
    };
    let mut total = 0;
    for ch in sequence.chars() {
        let end = keypad.find_item(&ch).unwrap();
        total += get_transition_cost(memo, start, end, is_directional, layer);
        start = end;
    }
    total
}

fn part2() -> usize {
    let input = read_input();
    //let input = vec!["029A".to_string()];
    let mut memo = HashMap::new();
    input
        .into_iter()
        .map(|code| {
            let cost = get_cost_in_layer(&mut memo, code.clone(), false, 25);
            let value = code[0..3].parse::<usize>().expect("Not numeric");
            println!("Code: {} Cost: {}", code, cost);
            cost * value
        })
        .sum()
}

fn main() {
    //let input: Vec<char> =
    //    "<vA<AA>>A^A<A>vA<A<AA>>^AAvAA<^A>A<vA>^AA<Av<A>A<A>>^AvA<^A>A<vA>A^A<A>A"
    //        .chars()
    //        .collect();
    //let output_1 = simulate_all_p1(&input).map(|x| x.iter().collect::<String>());
    //println!("{:?}", output_1);
    //
    println!("Part 1: {}", part1());
    //println!("Part 2: {}", part2());

    //let path_length = simulate_all_layers_reverse(&"964A".chars().collect_vec())
    //    .first()
    //    .map(|x| x.len())
    //    .expect("No paths found!");
    //println!("{}", path_length);

    println!("Part 2: {}", part2());
}
