use itertools::repeat_n;
use std::collections::BinaryHeap;

advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut blocks = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if !c.is_digit(10) {
            break;
        }
        if i % 2 == 0 {
            blocks.extend(repeat_n(
                Some((i / 2) as u32),
                c.to_string().parse().unwrap(),
            ))
        } else {
            blocks.extend(repeat_n(None, c.to_string().parse().unwrap()));
        }
    }
    blocks
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Block {
    position: usize,
    size: usize,
    file_id: Option<u32>,
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position
            .partial_cmp(&other.position)
            .map(|o| o.reverse())
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn get_blocks(blocks: &Vec<Option<u32>>) -> (Vec<Block>, Vec<BinaryHeap<Block>>) {
    let mut empty_blocks = Vec::new();
    let mut file_blocks = Vec::new();
    for _ in 0..10 {
        empty_blocks.push(BinaryHeap::new());
    }

    let mut i = 0;
    while i < blocks.len() {
        let mut size = 0;

        let initial_block = blocks[i];

        while i < blocks.len() && initial_block == blocks[i] {
            size += 1;
            i += 1;
        }

        if initial_block.is_none() {
            empty_blocks[size].push(Block {
                position: i - size as usize,
                size,
                file_id: None,
            });
        } else {
            file_blocks.push(Block {
                position: i - size as usize,
                size,
                file_id: initial_block,
            });
        }
    }

    (file_blocks, empty_blocks)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);
    let mut p_begin = 0;
    let mut p_end = blocks.len() - 1;
    loop {
        while blocks[p_begin].is_some() {
            p_begin += 1;
        }
        while blocks[p_end].is_none() {
            p_end -= 1;
        }
        if p_begin >= p_end {
            break;
        }
        blocks.swap(p_begin, p_end);
    }

    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            i as u64
                * match b {
                    Some(v) => *v,
                    None => 0,
                } as u64
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let blocks = parse_input(input);
    let (file_blocks, mut empty_blocks) = get_blocks(&blocks);
    let mut final_file_blocks = Vec::new();

    for file_block in file_blocks.iter().rev() {
        if let Some(b_heap) = empty_blocks
            .iter()
            .skip(file_block.size)
            .filter(|b_heap| !b_heap.is_empty())
            .min_by_key(|b_heap| b_heap.peek().unwrap().position)
        {
            let first_empty_block = b_heap.peek().unwrap().clone();
            if first_empty_block.position > file_block.position {
                final_file_blocks.push(file_block.clone());
            } else {
                let first_empty_block = empty_blocks[first_empty_block.size].pop().unwrap();
                final_file_blocks.push(Block {
                    position: first_empty_block.position,
                    size: file_block.size,
                    file_id: file_block.file_id,
                });

                if file_block.size < first_empty_block.size {
                    empty_blocks[first_empty_block.size - file_block.size].push(Block {
                        position: first_empty_block.position + file_block.size,
                        size: first_empty_block.size - file_block.size,
                        file_id: None,
                    });
                }
            }
        } else {
            final_file_blocks.push(file_block.clone());
        }
    }

    final_file_blocks
        .iter()
        .map(|b| {
            ((b.position..b.position + b.size).sum::<usize>() as u64 * b.file_id.unwrap() as u64)
                as u64
        })
        .sum::<u64>()
        .into()
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
