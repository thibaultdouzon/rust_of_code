use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn parse_input(input: &str) -> ((i32, i32), (i32, i32), Vec<Vec<char>>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut user = (0, 0);
    let mut goal = (0, 0);
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            match c {
                'E' => goal = (x as i32, y as i32),
                'S' => user = (x as i32, y as i32),
                _ => (),
            }
        }
    }
    (user, goal, map)
}

fn dijkstra(user: (i32, i32), goal: (i32, i32), map: Vec<Vec<char>>) -> (usize, usize) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut seen: HashMap<((i32, i32), usize), usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, (i32, i32), usize)>> = BinaryHeap::new();
    heap.push(Reverse((0, user.clone(), 0)));
    seen.entry((user.clone(), 0)).or_insert(0);

    let mut prev: HashMap<(i32, i32, usize), Vec<(i32, i32, usize)>> = HashMap::new();
    let mut solution_score: Option<usize> = None;

    while !heap.is_empty() {
        let Reverse((score, (x, y), dir)) = heap.pop().unwrap();
        if (x, y) == goal {
            solution_score = Some(score);
            continue;
        }
        if score > solution_score.unwrap_or(usize::MAX) {
            break;
        }
        for (d, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= height || ny >= width {
                continue;
            }
            let c = map[nx as usize][ny as usize];
            if c == '#' {
                continue;
            }
            let new_score = match (dir, d) {
                (ddir, dd) if ddir % 2 != dd % 2 => score + 1001,
                (ddir, dd) if ddir == dd => score + 1,
                _ => continue,
            };
            if let Some(old_score) = seen.get(&((nx, ny), d)) {
                if new_score == *old_score {
                    prev.entry((nx, ny, new_score))
                        .or_insert(Vec::new())
                        .push((x, y, score));
                }
                continue;
            }
            seen.entry(((nx, ny), d)).or_insert(new_score);

            heap.push(Reverse((new_score, (nx, ny), d)));
            prev.entry((nx, ny, new_score))
                .or_insert(Vec::new())
                .push((x, y, score));
        }
    }
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    let mut queue: VecDeque<(i32, i32, usize)> = VecDeque::new();
    queue.push_back((goal.0, goal.1, solution_score.unwrap()));
    while !queue.is_empty() {
        let (x, y, score) = queue.pop_front().unwrap();
        positions.insert((x, y));
        if prev.contains_key(&(x, y, score)) {
            for (nx, ny, ns) in prev[&(x, y, score)].iter() {
                queue.push_back((*nx, *ny, *ns));
            }
        }
    }

    (solution_score.unwrap(), positions.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (user, goal, map) = parse_input(input);
    Some(dijkstra(user, goal, map).0)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (user, goal, map) = parse_input(input);
    Some(dijkstra(user, goal, map).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
