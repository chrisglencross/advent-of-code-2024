use itertools::Itertools;

const DAY: u8 = 9;

#[derive(Clone, Copy)]
enum Blocks {
    Free { length: u32 },
    File { id: usize, length: u32 },
}

fn main() {
    let input = aocutil::load_input(DAY);
    let fs = parse_input(&input);

    println!("Part 1: {}", checksum(&compact_part1(&fs)));
    println!("Part 2: {}", checksum(&compact_part2(&fs)));
}

fn compact_part1(fs: &Vec<Blocks>) -> Vec<Blocks> {
    let mut compacted = fs.clone();
    let mut i = 0;
    while i < compacted.len() {
        if let Blocks::Free { length: free_length } = compacted[i] {
            if let Some(Blocks::File { id: file_id, length: file_length }) = compacted.pop() {
                if file_length >= free_length {
                    // move some of the file into free space; keep remainder at the end of the file system
                    compacted[i] = Blocks::File { id: file_id, length: free_length };
                    if file_length != free_length {
                        compacted.push(Blocks::File { id: file_id, length: file_length - free_length })
                    }
                } else if file_length > 0 {
                    // move whole file into free space and reduce free space
                    compacted[i] = Blocks::File { id: file_id, length: file_length };
                    compacted.insert(i + 1, Blocks::Free { length: free_length - file_length })
                }
            }
        } else {
            i += 1;
        }
    }
    compacted
}

fn compact_part2(fs: &[Blocks]) -> Vec<Blocks> {
    let mut compacted = fs.iter().cloned().collect_vec();
    for &file in fs.iter().rev() {
        if let Blocks::File { id: file_id, length: file_length } = file {
            if let Some((free_index, free_length)) = find_first_free(&compacted, file_length) {
                let file_index = find_file_index_by_id(&compacted, file_id);
                if free_index < file_index {
                    compacted[file_index] = Blocks::Free { length: file_length };
                    compacted[free_index] = Blocks::File { id: file_id, length: file_length };
                    if free_length > file_length {
                        compacted.insert(free_index + 1, Blocks::Free { length: free_length - file_length });
                    }
                }
            }
        }
    }
    compacted
}

fn find_file_index_by_id(compacted: &[Blocks], file_id: usize) -> usize {
    compacted.iter().find_position(|f| match f {
        Blocks::Free { .. } => false,
        Blocks::File { id, .. } => *id == file_id
    }).unwrap().0
}

fn find_first_free(fs: &[Blocks], min_length: u32) -> Option<(usize, u32)> {
    fs.iter().enumerate().find_map(|(i, b)| match b {
        Blocks::Free { length } => if *length >= min_length { Some((i, *length)) } else { None },
        Blocks::File { .. } => None,
    })
}

fn checksum(blocks: &Vec<Blocks>) -> usize {
    let mut total: usize = 0;
    let mut i = 0;
    for block in blocks {
        i += match block {
            Blocks::Free { length } => length,
            Blocks::File { length, id } => {
                for j in i..length + i {
                    total += j as usize * id;
                }
                length
            }
        };
    }
    total
}

fn parse_input(input: &str) -> Vec<Blocks> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate().map(|(i, length)|
            if i % 2 == 0 {
                Blocks::File { id: i / 2, length }
            } else {
                Blocks::Free { length }
            })
        .collect()
}