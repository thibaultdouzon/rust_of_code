use itertools::Itertools;

advent_of_code::solution!(15);
fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>, Vec<char>) {
    let input_blocks = input.split("\n\n").collect::<Vec<&str>>();
    let grid: Vec<Vec<char>> = input_blocks[0]
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut user = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                user = (x, y);
            }
        }
    }
    let moves = input_blocks[1].lines().join("").chars().collect();
    (user, grid, moves)
}

fn parse_input_2(input: &str) -> ((usize, usize), Vec<Vec<char>>, Vec<char>) {
    let (mut user, grid, moves) = parse_input(input);

    let mut new_grid = vec![vec!['.'; grid[0].len() * 2]; grid.len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            match c {
                '.' => {
                    new_grid[y][2 * x] = '.';
                    new_grid[y][2 * x + 1] = '.';
                }
                '#' => {
                    new_grid[y][2 * x] = '#';
                    new_grid[y][2 * x + 1] = '#';
                }
                'O' => {
                    new_grid[y][2 * x] = '[';
                    new_grid[y][2 * x + 1] = ']';
                }
                '@' => {
                    new_grid[y][2 * x] = '@';
                    new_grid[y][2 * x + 1] = '.';
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    user = (user.0 * 2, user.1);
    (user, new_grid, moves)
}

fn move_once(user: (usize, usize), grid: &mut Vec<Vec<char>>, direction: char) -> (usize, usize) {
    let (x, y) = user;
    let (x, y) = (x as i32, y as i32);
    let (dx, dy) = match direction {
        '^' => (0, -1i32),
        'v' => (0, 1),
        '<' => (-1i32, 0),
        '>' => (1, 0),
        _ => panic!("Invalid direction"),
    };

    match grid[(y + dy) as usize][(x + dx) as usize] {
        '.' => {
            grid[(y + dy) as usize][(x + dx) as usize] = '@';
            grid[(y) as usize][(x) as usize] = '.';
            return ((x + dx) as usize, (y + dy) as usize);
        }
        '#' => return (x as usize, y as usize),
        'O' => {
            let mut k = 1;
            while grid[(y + dy * k) as usize][(x + dx * k) as usize] == 'O' {
                k += 1;
            }
            match grid[(y + dy * k) as usize][(x + dx * k) as usize] {
                '.' => {
                    grid[(y + dy * k) as usize][(x + dx * k) as usize] = 'O';
                    grid[(y + dy) as usize][(x + dx) as usize] = '@';
                    grid[(y) as usize][(x) as usize] = '.';
                    return ((x + dx) as usize, (y + dy) as usize);
                }
                '#' => return (x as usize, y as usize),
                _ => panic!("Invalid character"),
            }
        }
        _ => panic!("Invalid character"),
    }
}

fn print_grid(grid: &Vec<Vec<char>>) -> () {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
    println!("\n\n");
}

fn score(grid: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'O' {
                score += 100 * i + j;
            } else if *c == '[' {
                score += 100 * i + j;
            }
        }
    }
    score
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut user, mut grid, moves) = parse_input(input);
    for c in moves.iter() {
        user = move_once(user, &mut grid, *c);
        // print_grid(&grid);
    }

    Some(score(&grid))
}

fn can_move(pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let (x, y) = pos;
    let (dx, dy) = dir;

    match grid[y as usize][x as usize] {
        '@' => {
            return can_move((x + dx, y + dy), dir, grid);
        }
        '.' => {
            return true;
        }
        '#' => {
            return false;
        }
        '[' => {
            return can_move((x + dx, y + dy), dir, grid)
                && (dy == 0 || can_move((x + dx + 1, y + dy), dir, grid));
        }
        ']' => {
            return can_move((x + dx, y + dy), dir, grid)
                && (dy == 0 || can_move((x + dx - 1, y + dy), dir, grid));
        }
        _ => panic!("Invalid character"),
    };
}

fn move_once_2(user: (usize, usize), grid: &mut Vec<Vec<char>>, direction: char) -> (usize, usize) {
    let (x, y) = user;
    let (x, y) = (x as i32, y as i32);
    let (dx, dy) = match direction {
        '^' => (0, -1i32),
        'v' => (0, 1),
        '<' => (-1i32, 0),
        '>' => (1, 0),
        _ => panic!("Invalid direction"),
    };
    if can_move((x, y), (dx, dy), grid) {
        apply_move((x, y), grid, (dx, dy));
        return ((x + dx) as usize, (y + dy) as usize);
    }
    return (x as usize, y as usize);
}

fn apply_move(pos: (i32, i32), grid: &mut Vec<Vec<char>>, dir: (i32, i32)) -> () {
    let (x, y) = pos;
    let (dx, dy) = dir;

    match grid[y as usize][x as usize] {
        '@' => {
            apply_move((x + dx, y + dy), grid, dir);
            grid[y as usize][x as usize] = '.';
            grid[(y + dy) as usize][(x + dx) as usize] = '@';
        }
        '[' => {
            apply_move((x + dx, y + dy), grid, dir);
            grid[y as usize][x as usize] = '.';
            grid[(y + dy) as usize][(x + dx) as usize] = '[';

            if dy != 0 {
                apply_move((x + dx + 1, y + dy), grid, dir);
                grid[(y + dy) as usize][(x + dx + 1) as usize] = ']';
                grid[y as usize][(x + 1) as usize] = '.';
            }
        }
        ']' => {
            apply_move((x + dx, y + dy), grid, dir);
            grid[y as usize][x as usize] = '.';
            grid[(y + dy) as usize][(x + dx) as usize] = ']';

            if dy != 0 {
                apply_move((x + dx - 1, y + dy), grid, dir);
                grid[(y + dy) as usize][(x + dx - 1) as usize] = '[';
                grid[y as usize][(x - 1) as usize] = '.';
            }
        }
        _ => (),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut user, mut grid, moves) = parse_input_2(input);
    for c in moves.iter() {
        user = move_once_2(user, &mut grid, *c);
        // print_grid(&grid);
    }

    Some(score(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
