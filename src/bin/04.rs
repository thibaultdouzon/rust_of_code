advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let vec_input: Vec<Vec<char>> = input
        .lines()
        .map(|line| format!("....{line}....").chars().collect())
        .collect();

    let n = vec_input[0].len();

    let mut padded_input = vec![vec!['.'; n]; 4];
    padded_input.extend(vec_input);
    padded_input.extend(vec![vec!['.'; n]; 4]);
    padded_input
}

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const XMAS_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let m = input.len();
    let n = input[0].len();

    let mut xmas = 0;
    for i in 4..(m - 4) {
        for j in 4..(n - 4) {
            if input[i][j] == XMAS[0] {
                for (dx, dy) in XMAS_DIRECTIONS.iter() {
                    let mut found = true;
                    for k in 1..4 {
                        if input[(i as i32 + dx * k) as usize][(j as i32 + dy * k) as usize]
                            != XMAS[k as usize]
                        {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        xmas += 1;
                    }
                }
            }
        }
    }
    Some(xmas)
}

const X_MAS: [char; 4] = ['M', 'M', 'S', 'S'];
const X_MAS_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input);

    let m = input.len();
    let n = input[0].len();

    let mut xmas = 0;
    for i in 4..(m - 4) {
        for j in 4..(n - 4) {
            if input[i][j] == 'A' {
                for k in 0..4 {
                    let mut found = true;
                    for (letter, (dx, dy)) in X_MAS
                        .iter()
                        .cycle()
                        .skip(k)
                        .take(4)
                        .zip(X_MAS_DIRECTIONS.iter())
                    {
                        if input[(i as i32 + dx) as usize][(j as i32 + dy) as usize] != *letter {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        xmas += 1;
                    }
                }
            }
        }
    }
    Some(xmas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2454u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1858u32));
    }
}
