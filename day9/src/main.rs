use itertools::Itertools;

use crate::BlockRun::{File, Free};

const DAY: u8 = 9;

#[derive(Clone, Copy)]
enum BlockRun {
    Free { length: u32 },
    File { id: usize, length: u32 },
}

fn main() {
    let input = aocutil::load_input(DAY);
    let fs = parse_input(&input);

    println!("Part 1: {}", checksum(&compact_part1(&fs)));
    println!("Part 2: {}", checksum(&compact_part2(&fs)));
}

fn compact_part1(fs: &Vec<BlockRun>) -> Vec<BlockRun> {
    let mut compacted = fs.clone();
    let mut i = 0;
    while i < compacted.len() {
        if let Free { length: free_length } = compacted[i] {
            let last = compacted.pop();
            if let Some(File { id, length }) = last {
                if length >= free_length {
                    compacted[i] = File { id: id, length: free_length };
                    if length - free_length > 0 {
                        compacted.push(File { id: id, length: length - free_length })
                    }
                } else if length > 0 {
                    compacted[i] = File { id: id, length: length };
                    if free_length - length > 0 {
                        compacted.insert(i + 1, Free { length: free_length - length })
                    }
                }
            }
        } else {
            i += 1;
        }
    }
    compacted
}

fn compact_part2(fs: &Vec<BlockRun>) -> Vec<BlockRun> {
    let mut compacted = fs.clone();
    for &file in fs.iter().rev() {
        if let File{id: file_id, length: file_length} = file {
            if let Some((free_index, free_length)) = find_first_free(&compacted, file_length) {
                let file_index = find_file_index_by_id(&compacted, file_id);
                if free_index < file_index {
                    compacted[file_index] = Free { length: file_length };
                    compacted[free_index] = File { id: file_id, length: file_length };
                    if free_length > file_length {
                        compacted.insert(free_index + 1, Free { length: free_length - file_length });
                    }
                }
            }
        }
    }
    compacted
}

fn find_file_index_by_id(compacted: &Vec<BlockRun>, file_id: usize) -> usize {
    compacted.iter().find_position(|f| match f {
        Free { .. } => false,
        File { id, .. } => *id == file_id
    }).unwrap().0
}

fn find_first_free(fs: &Vec<BlockRun>, min_length: u32) -> Option<(usize, u32)> {
    fs.iter().enumerate()
        .find_map(|(i, b)| match b {
            Free { length } => if *length >= min_length { Some((i, *length)) } else { None },
            File { .. } => None,
        })
}


fn checksum(blocks: &Vec<BlockRun>) -> usize {
    let mut total: usize = 0;
    let mut i = 0;
    for block in blocks {
        i += match block {
            Free { length } => length,
            File { length, id } => {
                for j in i..length + i {
                    total += usize::try_from(j).unwrap() * id;
                }
                length
            }
        };
    }
    total
}

fn parse_input(input: &str) -> Vec<BlockRun> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, length)| if i % 2 == 0 { BlockRun::File { id: i / 2, length } } else { BlockRun::Free { length } })
        .collect()
}