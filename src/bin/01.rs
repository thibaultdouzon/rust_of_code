use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        if let Some((num_left, num_right)) = line
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect_tuple()
        {
            left.push(num_left);
            right.push(num_right);
        }
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    Some(
        left.iter()
            .zip(right.iter())
            .map(|(num_left, num_right)| (*num_left as i32 - *num_right as i32).abs())
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);

    let mut counter = HashMap::<&u32, u32>::new();
    for num in right.iter() {
        counter.insert(num, counter.get(num).unwrap_or(&0) + 1);
    }

    Some(
        left.iter()
            .map(|num| counter.get(num).unwrap_or(&0) * num)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
