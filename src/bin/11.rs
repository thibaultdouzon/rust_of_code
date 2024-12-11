use std::collections::HashMap;
advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn blink(numbers: &Vec<u64>) -> Vec<u64> {
    let mut new_numbers = Vec::new();
    for n in numbers {
        match *n {
            0 => new_numbers.push(1),
            x if format!("{x}").len() % 2 == 0 => {
                let x_str = format!("{x}");
                new_numbers.push(x_str[..x_str.len() / 2].parse().unwrap());
                new_numbers.push(x_str[x_str.len() / 2..].parse().unwrap());
            }
            x => new_numbers.push(x * 2024),
        }
    }
    new_numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = parse_input(input);
    for _ in 0..25 {
        numbers = blink(&numbers);
    }
    Some(numbers.len() as u32)
}

fn blink_dp_top_down(number: u64, n: u32, mem: &mut HashMap<(u64, u32), u64>) -> u64 {
    if n == 0 {
        return 1;
    } else {
        if let Some(&ret) = mem.get(&(number, n)) {
            return ret;
        }
        let ret = match number {
            0 => blink_dp_top_down(1, n - 1, mem),
            x if format!("{x}").len() % 2 == 0 => {
                let x_str = format!("{x}");
                let (left, right) = (
                    x_str[..x_str.len() / 2].parse().unwrap(),
                    x_str[x_str.len() / 2..].parse().unwrap(),
                );
                blink_dp_top_down(left, n - 1, mem) + blink_dp_top_down(right, n - 1, mem)
            }
            x => blink_dp_top_down(x * 2024, n - 1, mem),
        };
        mem.insert((number, n), ret);
        return ret;
    }
}

fn blink_fast_top_down(numbers: &Vec<u64>, n: u32) -> u64 {
    let mut mem = HashMap::new();
    numbers
        .iter()
        .map(|x| blink_dp_top_down(*x, n, &mut mem))
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = parse_input(input);
    Some(blink_fast_top_down(&numbers, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
