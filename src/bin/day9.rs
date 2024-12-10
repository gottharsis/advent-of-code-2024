use std::cmp::min;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn read_input() -> Vec<FileType> {
    read_to_string("input/day9.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).expect("Could not parse digit"))
        .enumerate()
        .map(|(idx, size)| {
            if idx % 2 == 0 {
                FileType::File(File {
                    id: (idx / 2) as u64,
                    size: size.into(),
                })
            } else {
                FileType::Space(size.into())
            }
        })
        .collect()
}

#[derive(Copy, Clone)]
struct File {
    id: u64,
    size: u64,
}

enum FileType {
    Space(u64),
    File(File),
}

fn remove_n_file_blocks(queue: &mut VecDeque<FileType>, n: u64) -> Vec<u64> {
    let mut n = n;
    let mut ids: Vec<u64> = Vec::with_capacity(n as usize);
    while n > 0 {
        if queue.is_empty() {
            break;
        }
        let right = queue.back_mut().unwrap();
        match right {
            FileType::Space(_) => {
                queue.pop_back();
                continue;
            }
            FileType::File(File { id, size }) => {
                let num_blocks = min(*size, n);
                for _ in 0..num_blocks {
                    ids.push(*id);
                    *size -= 1;
                    n -= 1;
                }
                if *size == 0 {
                    queue.pop_back();
                }
            }
        }
    }

    ids
}

fn part1() -> u64 {
    let mut queue = VecDeque::from(read_input());
    let mut checksum = 0u64;
    let mut i = 0u64;
    while !queue.is_empty() {
        let entry = queue.pop_front().unwrap();
        match entry {
            FileType::File(File { id, size }) => {
                for _ in 0..size {
                    checksum += i * id;
                    i += 1;
                }
            }
            FileType::Space(size) => {
                let ids = remove_n_file_blocks(&mut queue, size);
                for id in ids {
                    checksum += i * id;
                    i += 1;
                }
            }
        }
    }
    checksum
}

fn try_move_file_from_end(queue: &mut VecDeque<FileType>, n_blocks: u64) -> Option<File> {
    for item in queue.iter_mut().rev() {
        match item {
            FileType::Space(_) => continue,
            FileType::File(f) => {
                if f.size <= n_blocks {
                    let ret = *f;
                    *item = FileType::Space(f.size);
                    return Some(ret);
                }
            }
        }
    }
    None
}

fn part2() -> u64 {
    let mut queue = VecDeque::from(read_input());
    let mut checksum = 0u64;
    let mut i = 0u64;
    let mut n_consecutive_spaces = 0u64;

    while !queue.is_empty() {
        let entry = queue.pop_front().unwrap();
        match entry {
            FileType::File(File{id, size}) =>{
                i += n_consecutive_spaces;
                n_consecutive_spaces = 0;
            for _ in 0..size {
                    checksum += id * i;
                    i += 1;
                }
        }
            FileType::Space(n_blocks) => {
                n_consecutive_spaces += n_blocks;
                if let Some(file) = try_move_file_from_end(&mut queue, n_consecutive_spaces) {
                    let remaining_space = n_consecutive_spaces - file.size;
                    n_consecutive_spaces = 0;
                    if remaining_space > 0 {
                        queue.push_front(FileType::Space(remaining_space));
                    }
                    queue.push_front(FileType::File(file));
                }             }
        }
    }

    checksum
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
