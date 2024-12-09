use std::cmp::min;

use itertools::Itertools;

use crate::BlockRun::{File, Free};

const DAY: u8 = 9;

#[derive(Clone, Copy, Debug)]
struct FileBlockRun {
    id: usize,
    length: u32,
}

#[derive(Debug, Clone, Copy)]
enum BlockRun {
    Free { length: u32 },
    File { id: usize, length: u32 },
}

/// Consumes and returns blocks from the end of a run-length encoded list of blocks.
struct TailConsumer {
    current: FileBlockRun,
    current_remaining: u32,
    rest: Vec<FileBlockRun>,
}

impl TailConsumer {
    fn read(&mut self, mut consume_count: u32) -> Vec<FileBlockRun> {
        let mut result: Vec<FileBlockRun> = vec![];
        while consume_count >= self.current_remaining {
            if self.current_remaining > 0 {
                result.push(FileBlockRun { id: self.current.id, length: self.current_remaining });
                consume_count -= self.current_remaining;
            }
            self.current = self.rest.pop().unwrap();
            self.current_remaining = self.current.length;
        }
        if consume_count > 0 {
            result.push(FileBlockRun { id: self.current.id, length: consume_count });
            self.current_remaining -= consume_count;
        }
        result
    }
}

fn main() {
    let input = aocutil::load_input(DAY);
    let fs = parse_input(&input);

    println!("Part 1: {}", checksum(&compact_part1(&fs)));
    println!("Part 2: {}", checksum(&compact_part2(&fs)));
}

fn compact_part1(fs: &Vec<BlockRun>) -> Vec<BlockRun> {
    let mut head_consumer = fs.iter();
    let mut tail_consumer = TailConsumer {
        current: FileBlockRun { id: 0, length: 0 },
        current_remaining: 0,
        rest: to_file_block_run(fs),
    };
    let mut blocks_remaining = total_file_blocks(&fs);
    let mut compacted: Vec<BlockRun> = vec![];
    while blocks_remaining > 0 {
        let blocks_to_append = match head_consumer.next().unwrap() {
            File { id, length } => vec![FileBlockRun { id: *id, length: *length }],
            Free { length } => tail_consumer.read(*length),
        };
        for block in blocks_to_append {
            compacted.push(File { id: block.id, length: min(block.length, blocks_remaining) });
            blocks_remaining -= min(block.length, blocks_remaining);
        }
    }
    compacted
}

fn to_file_block_run(fs: &Vec<BlockRun>) -> Vec<FileBlockRun> {
    fs.iter()
        .filter_map(|b| match b {
            Free { length: _ } => None,
            File { id, length } => Some(FileBlockRun { id: *id, length: *length })
        })
        .collect()
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

fn total_file_blocks(fs: &Vec<BlockRun>) -> u32 {
    fs.iter()
        .map(|b| match b {
            Free { .. } => 0,
            File { length, .. } => *length
        }).sum()
}

fn compact_part2(fs: &Vec<BlockRun>) -> Vec<BlockRun> {
    let files: Vec<FileBlockRun> = to_file_block_run(fs);

    let mut compacted = fs.clone();
    for file in files.iter().rev() {
        if let Some((to_index, length)) = find_first_free(&compacted, file.length) {
            let (from_index, _) = compacted.iter().find_position(|f| match f {
                Free { .. } => false,
                File { id, .. } => *id == file.id
            }).unwrap();

            if to_index < from_index {
                compacted[from_index] = Free { length: file.length };
                compacted[to_index] = File { id: file.id, length: file.length };
                if length > file.length {
                    compacted.insert(to_index + 1, Free { length: length - file.length });
                }
            }
        }
    }
    compacted
}

fn find_first_free(fs: &Vec<BlockRun>, min_length: u32) -> Option<(usize, u32)> {
    fs.iter().enumerate()
        .find_map(|(i, b)| match b {
            Free { length } => if *length >= min_length { Some((i, *length)) } else { None },
            File { .. } => None,
        })
}

fn parse_input(input: &str) -> Vec<BlockRun> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, length)| if i % 2 == 0 { BlockRun::File { id: i / 2, length } } else { BlockRun::Free { length } })
        .collect()
}