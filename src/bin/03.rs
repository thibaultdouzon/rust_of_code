use regex::Regex;
advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);
    input.iter().map(|(a, b)| a * b).sum::<u32>().into()
}

fn parse_input_2(input: &str) -> Vec<(u32, u32)> {
    let re_do = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let input = re_do.replace_all(&input, "#");

    let re_last_dont = Regex::new(r"(?s)don't\(\).*$").unwrap();
    let input = re_last_dont.replace(&input, "#");

    let re_numbers = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re_numbers
        .captures_iter(&input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input_2(input);
    input
        .iter()
        .map(|(a, b)| (a * b) as u64)
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(178538786u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102467299u64));
    }
}
