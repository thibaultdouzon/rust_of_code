use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(10);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    value: u32,
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn neighbours(node: &Node, map: &Vec<Vec<u32>>) -> Vec<Node> {
    let mut neighbours = Vec::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let x = node.x as i32 + dx;
        let y = node.y as i32 + dy;

        if x >= 0
            && y >= 0
            && x < map.len() as i32
            && y < map[0].len() as i32
            && map[x as usize][y as usize] == node.value + 1
        {
            neighbours.push(Node {
                x: x as usize,
                y: y as usize,
                value: map[x as usize][y as usize],
            });
        }
    }
    neighbours
}

fn bfs(node: Node, map: &Vec<Vec<u32>>, v: i32) -> u32 {
    let mut queue = VecDeque::new();

    queue.push_back(node);

    let mut trail_count = HashSet::new();
    let mut trail_count2 = 0;
    while !queue.is_empty() {
        let curr_node = queue.pop_front().unwrap();

        if curr_node.value == 9 {
            trail_count.insert(curr_node.clone());
            trail_count2 += 1;
            continue;
        }

        for neighbour in neighbours(&curr_node, map) {
            queue.push_back(neighbour);
        }
    }
    match v {
        1 => trail_count.len() as u32,
        2 => trail_count2,
        _ => panic!("Invalid value"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let height = map.len();
    let width = map[0].len();

    let mut all_trails = 0;
    for x in 0..width {
        for y in 0..height {
            if map[x][y] == 0 {
                let node = Node { x, y, value: 0 };
                let node_score = bfs(node.clone(), &map, 1);
                all_trails += node_score;
            }
        }
    }

    Some(all_trails)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let height = map.len();
    let width = map[0].len();

    let mut all_trails = 0;
    for x in 0..width {
        for y in 0..height {
            if map[x][y] == 0 {
                let node = Node { x, y, value: 0 };
                let node_score = bfs(node.clone(), &map, 2);
                all_trails += node_score;
            }
        }
    }

    Some(all_trails)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
