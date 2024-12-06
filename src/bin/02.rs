advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(report: &Vec<i32>) -> bool {
    let direction = (report[1] - report[0]).signum();
    if direction == 0 {
        return false;
    }

    for (x, y) in report.iter().zip(report.iter().skip(1)) {
        if (y - x).signum() != direction {
            return false;
        }
        if (y - x).abs() > 3 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    Some(reports.iter().filter(|&report| is_valid(report)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    Some(
        reports
            .iter()
            .filter(|&report| {
                if is_valid(report) {
                    return true;
                }
                (0..report.len()).any(|i| {
                    let mut report_small = report.clone();
                    report_small.remove(i);
                    is_valid(&report_small)
                })
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(660u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(689u32));
    }
}
