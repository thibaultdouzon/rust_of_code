advent_of_code::solution!(7);

#[derive(Debug)]
struct Line {
    result: u64,
    numbers: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (res, nums) = line.split_once(": ").unwrap();
            let result = res.parse().unwrap();
            let numbers = nums.split(" ").map(|n| n.parse().unwrap()).collect();
            Line { result, numbers }
        })
        .collect()
}

fn solve_line(line: &Line, allow_concat: Option<bool>) -> bool {
    let allow_concat = allow_concat.unwrap_or(false);
    let mut curr_values = vec![line.result];
    for num in line.numbers.iter().skip(1).rev() {
        let mut new_values = Vec::new();
        let mut ten_pow_mod = 10u64.pow((*num as f64).log10().ceil() as u32);
        if *num == ten_pow_mod {
            ten_pow_mod *= 10;
        }

        for val in curr_values {
            if val >= *num {
                new_values.push(val - num);
            }
            if val % num == 0 {
                new_values.push(val / num);
            }

            if allow_concat && val > 0 && val != *num && (val % ten_pow_mod) == *num {
                new_values.push(val / ten_pow_mod);
            }
        }
        curr_values = new_values;
    }
    curr_values.contains(&line.numbers[0])
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    lines
        .iter()
        .filter(|line| solve_line(line, None))
        .map(|line| line.result)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    lines
        .iter()
        .filter(|line| solve_line(line, Some(true)))
        .map(|line| line.result)
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
