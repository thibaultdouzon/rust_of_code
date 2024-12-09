use itertools::{repeat_n, Itertools};

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

fn blocks_to_str(blocks: &[Option<u32>]) -> String {
    blocks
        .iter()
        .map(|b| match b {
            Some(v) => v.to_string(),
            None => ".".to_string(),
        })
        .join("")
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);

    let mut file_id = 0;
    for b in blocks.iter().rev() {
        if let Some(v) = b {
            file_id = *v;
            break;
        }
    }

    loop {
        if file_id == 0 {
            break;
        }
        let count_file_id = blocks.iter().filter(|b| b == &&Some(file_id)).count();
        let (file_position, _) = blocks
            .iter()
            .find_position(|b| b == &&Some(file_id))
            .unwrap();

        let mut p_begin = 0;
        loop {
            if p_begin >= file_position {
                break;
            }
            if blocks[p_begin].is_some() {
                p_begin += 1;
            } else {
                let mut count_none = 0;
                while blocks[p_begin + count_none].is_none() {
                    count_none += 1;
                }
                if count_none >= count_file_id {
                    break;
                }
                p_begin += count_none;
            }
        }
        if p_begin < file_position {
            for i in 0..count_file_id {
                blocks.swap(p_begin + i, file_position + i);
            }
        }

        file_id -= 1;
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
