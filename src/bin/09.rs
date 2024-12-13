#![feature(iter_array_chunks)]

use std::{collections::VecDeque, fs::File, io::Write};

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug)]
struct FileBlock {
    id: usize,
    size: usize,
}

impl FileBlock {
    fn new(id: usize, size: usize) -> FileBlock {
        FileBlock { id, size }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut numbers = input
        .chars()
        .filter_map(|num| num.to_string().parse::<usize>().ok())
        .enumerate()
        .collect::<VecDeque<_>>();
    let mut disk: Vec<FileBlock> = Vec::new();
    while let Some((index, mut number)) = numbers.pop_front() {
        if index % 2 == 1 {
            while number > 0 {
                if let Some((index_o, other)) = numbers.pop_back() {
                    if index_o % 2 == 0 {
                        let id_o = index_o / 2;
                        if other <= number {
                            number -= other;
                            disk.push(FileBlock::new(id_o, other));
                        } else {
                            disk.push(FileBlock::new(id_o, number));
                            let remaining = other - number;
                            number = 0;
                            numbers.push_back((index_o, remaining));
                        }
                    }
                }
            }
        } else {
            let id = index / 2;
            disk.push(FileBlock::new(id, number));
        }
    }
    let mut idx = 0;
    let mut result = 0;
    for block in disk {
        for _ in 0..block.size {
            result += block.id * idx;
            idx += 1;
        }
    }

    Some(result)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    FileBlock { id: usize, size: usize },
    FreeBlock { size: usize },
}

impl Block {
    pub fn free(size: usize) -> Block {
        Block::FreeBlock { size }
    }

    pub fn file(id: usize, size: usize) -> Block {
        Block::FileBlock { id, size }
    }
}

pub fn merge_free(mut disk: Vec<Block>) -> Vec<Block> {
    let mut compacted_disk: Vec<Block> = Vec::new();
    let mut compacting: usize = 0;
    while let Some(block) = disk.pop() {
        match block {
            Block::FileBlock { id: _, size: _ } => {
                if compacting > 0 {
                    compacted_disk.push(Block::free(compacting));
                    compacting = 0;
                }
                compacted_disk.push(block);
            }
            Block::FreeBlock { size } => compacting += size,
        }
    }
    compacted_disk.reverse();
    compacted_disk
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = input
        .chars()
        .filter_map(|num| num.to_string().parse::<usize>().ok())
        .enumerate()
        .collect_vec();
    dbg!(numbers);
    let numbers = input
        .chars()
        .filter_map(|num| num.to_string().parse::<usize>().ok())
        .enumerate();
    let mut disk: Vec<Block> = Vec::new();
    for (index, amount) in numbers {
        if index % 2 == 1 {
            if amount > 0 {
                disk.push(Block::free(amount));
            }
        } else {
            let id = index / 2;
            disk.push(Block::file(id, amount));
        }
    }
    let mut pos = disk.len();
    while pos != 0 {
        pos -= 1;
        if let Some(Block::FileBlock { id: _, size }) = disk.get(pos) {
            let mut swap_pos: Option<usize> = None;
            let mut remaining_opt: Option<usize> = None;
            for (other_pos, other) in disk[0..pos].iter().enumerate() {
                match other {
                    Block::FileBlock { id: _, size: _ } => (),
                    Block::FreeBlock { size: free } => {
                        if size == free {
                            swap_pos = Some(other_pos);
                            break;
                        } else if size < free {
                            remaining_opt = Some(free - size);
                            swap_pos = Some(other_pos);
                            break;
                        }
                    }
                }
            }
            if let Some(other_pos) = swap_pos {
                disk.swap(pos, other_pos);
                if let Some(remaining) = remaining_opt {
                    if let Some(free_block) = disk.get_mut(pos) {
                        match free_block {
                            Block::FileBlock { size: _, id: _ } => {
                                panic!("should not be a file block")
                            }
                            Block::FreeBlock { size } => {
                                *free_block = Block::free(*size - remaining)
                            }
                        };
                    }
                    disk.insert(other_pos + 1, Block::FreeBlock { size: remaining });
                    pos += 1;
                }
            }
        }
    }
    let disk = merge_free(disk);
    let mut idx = 0;
    let mut result = 0;
    for block in disk {
        match block {
            Block::FileBlock { id, size } => {
                for _ in 0..size {
                    result += id * idx;
                    idx += 1;
                }
            }
            Block::FreeBlock { size } => {
                idx += size;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_merge_frees() {
        let result = merge_free(vec![
            Block::file(0, 1),
            Block::free(1),
            Block::free(1),
            Block::file(1, 1),
        ]);
        assert_eq!(
            result,
            vec![Block::file(0, 1), Block::free(2), Block::file(1, 1)]
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
