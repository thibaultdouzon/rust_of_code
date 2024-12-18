use std::cmp::Reverse;

use itertools::Itertools;

advent_of_code::solution!(18);

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn dijkstra(start: (i32, i32), target: (i32, i32), map: &Vec<Vec<bool>>) -> Option<usize> {
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = std::collections::BinaryHeap::new();

    queue.push(Reverse((0, start)));
    while !queue.is_empty() {
        let Reverse((d, (x, y))) = queue.pop().unwrap();
        if (x, y) == target {
            return Some(d);
        }

        if dist[y as usize][x as usize] <= d {
            continue;
        }
        dist[y as usize][x as usize] = d;
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= map[0].len() as i32 || ny < 0 || ny >= map.len() as i32 {
                continue;
            }
            if map[ny as usize][nx as usize] {
                continue;
            }
            queue.push(Reverse((d + 1, (nx, ny))));
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let pixels = parse_input(input);
    let (map_len, take) = if pixels.len() > 1024 {
        (71, 1024)
    } else {
        (7, 12)
    };
    let mut map = vec![vec![false; map_len]; map_len];
    for (x, y) in pixels.into_iter().take(take) {
        map[y as usize][x as usize] = true;
    }

    dijkstra((0, 0), ((map_len - 1) as i32, (map_len - 1) as i32), &map)
}

pub fn part_two(input: &str) -> Option<String> {
    let pixels = parse_input(input);
    let map_len = if pixels.len() > 1024 { 71 } else { 7 };
    let mut map = vec![vec![false; map_len]; map_len];
    for (x, y) in pixels.into_iter() {
        map[y as usize][x as usize] = true;
        if dijkstra((0, 0), ((map_len - 1) as i32, (map_len - 1) as i32), &map).is_none() {
            return Some(format!("{},{}", x, y));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
