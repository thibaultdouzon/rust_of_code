use gcd::Gcd;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> HashMap<char, Vec<Point>> {
    let mut map = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert(Vec::new()).push(Point {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }

    map
}

fn is_inside(p: &Point, size: (i32, i32)) -> bool {
    p.x >= 0 && p.x < size.1 && p.y >= 0 && p.y < size.0
}

fn mirror(p: &Point, p_mirror: &Point, size: (i32, i32)) -> Option<Point> {
    let p = Point {
        x: 2 * p_mirror.x - p.x,
        y: 2 * p_mirror.y - p.y,
    };
    if is_inside(&p, size) {
        Some(p)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let h = input.lines().count() as i32;
    let w = input.lines().next()?.len() as i32;

    let mut antinodes = HashSet::<Point>::new();
    for points in map.values() {
        for (i, p1) in points.iter().enumerate() {
            for p2 in points.iter().skip(i + 1) {
                if let Some(p_mirror) = mirror(&p1, &p2, (h, w)) {
                    antinodes.insert(p_mirror);
                }
                if let Some(p_mirror) = mirror(&p2, &p1, (h, w)) {
                    antinodes.insert(p_mirror);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

fn aligned(p1: &Point, p2: &Point, size: (i32, i32)) -> Vec<Point> {
    let u = (p2.x - p1.x, p2.y - p1.y);
    let gcd = (u.0.abs() as u32).gcd(u.1.abs() as u32);
    let u = (u.0 / gcd as i32, u.1 / gcd as i32);

    let mut aligned_points = Vec::new();
    let mut p = p1.clone();
    while is_inside(&p, size) {
        aligned_points.push(p);
        p = Point {
            x: p.x + u.0,
            y: p.y + u.1,
        };
    }

    let mut p = p1.clone();
    while is_inside(&p, size) {
        aligned_points.push(p);
        p = Point {
            x: p.x - u.0,
            y: p.y - u.1,
        };
    }
    aligned_points
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let h = input.lines().count() as i32;
    let w = input.lines().next()?.len() as i32;

    let mut antinodes = HashSet::<Point>::new();
    for points in map.values() {
        for (i, p1) in points.iter().enumerate() {
            for p2 in points.iter().skip(i + 1) {
                antinodes.extend(aligned(&p1, &p2, (h, w)));
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
