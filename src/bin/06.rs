use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;
advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn in_bound(position: &Position, width: usize, height: usize) -> bool {
    position.x >= 0 && position.x < width as i32 && position.y >= 0 && position.y < height as i32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        match other {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn parse_input(input: &str) -> (Position, Direction, HashSet<Position>) {
    let mut guard = Position { x: 0, y: 0 };
    let mut blocks = HashSet::<Position>::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => blocks
                    .insert(Position {
                        x: j as i32,
                        y: i as i32,
                    })
                    .into(),
                '^' => {
                    guard = Position {
                        x: j as i32,
                        y: i as i32,
                    };
                    None
                }
                _ => None,
            };
        }
    }

    (guard, Direction::Up, blocks)
}

fn simulate(
    mut guard: Position,
    mut direction: Direction,
    blocks: &HashSet<Position>,
    width: usize,
    height: usize,
) -> HashSet<Position> {
    let mut visited = HashSet::<Position>::new();
    loop {
        visited.insert(guard);
        let next_guard = guard + direction;
        if blocks.contains(&next_guard) {
            direction = turn_right(&direction);
            continue;
        }

        if in_bound(&next_guard, width, height) {
            guard = next_guard;
        } else {
            break;
        }
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let (guard, direction, blocks) = parse_input(input);
    let width = input.lines().next()?.len();
    let height = input.lines().count();

    let visited_positions = simulate(guard, direction, &blocks, width, height);
    Some(visited_positions.len() as u32)
}

fn is_looping(
    mut guard: Position,
    mut direction: Direction,
    blocks: &HashSet<Position>,
    width: usize,
    height: usize,
) -> bool {
    let mut visited = HashSet::<(Position, Direction)>::new();
    loop {
        if visited.contains(&(guard, direction)) {
            break true;
        }
        visited.insert((guard, direction));
        let next_guard = guard + direction;
        if blocks.contains(&next_guard) {
            direction = turn_right(&direction);
            continue;
        }

        if in_bound(&next_guard, width, height) {
            guard = next_guard;
        } else {
            break false;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (guard, direction, mut blocks) = parse_input(input);
    let width = input.lines().next()?.len();
    let height = input.lines().count();

    let mut loop_count = 0;
    let visited_positions = simulate(guard, direction, &blocks, width, height);
    for new_block in visited_positions {
        if blocks.contains(&new_block) || new_block == guard {
            continue;
        }

        blocks.insert(new_block);
        if is_looping(guard, direction, &blocks, width, height) {
            loop_count += 1;
        }
        blocks.remove(&new_block);
    }

    Some(loop_count)
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
