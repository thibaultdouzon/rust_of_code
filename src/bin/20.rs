use std::cmp::Reverse;
advent_of_code::solution!(20);

fn parse_input(input: &str) -> ((i32, i32), (i32, i32), Vec<Vec<char>>) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (x, line) in map.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = (x as i32, y as i32);
            }
            if *char == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }
    (start, end, map)
}

fn dijkstra(
    start: (i32, i32),
    target: (i32, i32),
    map: &Vec<Vec<char>>,
) -> Vec<Vec<Option<usize>>> {
    let mut dist = vec![vec![None; map[0].len()]; map.len()];
    let mut queue = std::collections::BinaryHeap::new();

    queue.push(Reverse((0, start)));
    while !queue.is_empty() {
        let Reverse((d, (x, y))) = queue.pop().unwrap();
        if (x, y) == target {
            dist[x as usize][y as usize] = Some(d);
            break;
        }

        if dist[x as usize][y as usize].unwrap_or(usize::MAX) <= d {
            continue;
        }
        dist[x as usize][y as usize] = Some(d);
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= map.len() as i32 || ny < 0 || ny >= map[0].len() as i32 {
                continue;
            }
            if map[nx as usize][ny as usize] == '#' {
                continue;
            }
            queue.push(Reverse((d + 1, (nx, ny))));
        }
    }

    dist
}

fn cheat_once(dists: &Vec<Vec<Option<usize>>>) -> Vec<((i32, i32), (i32, i32), Option<usize>)> {
    let mut cheats = vec![];
    for (x, line) in dists.iter().enumerate() {
        for (y, dist) in line.iter().enumerate() {
            if let Some(d) = dist {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nei1 = (x as i32 + dx, y as i32 + dy);
                    let nei2 = (x as i32 + 2 * dx, y as i32 + 2 * dy);

                    if nei2.0 < 0
                        || nei2.0 >= dists.len() as i32
                        || nei2.1 < 0
                        || nei2.1 >= dists[0].len() as i32
                    {
                        continue;
                    }
                    if dists[nei1.0 as usize][nei1.1 as usize].is_none()
                        && dists[nei2.0 as usize][nei2.1 as usize].is_some()
                    {
                        let d_nei = dists[nei2.0 as usize][nei2.1 as usize].unwrap();
                        if d_nei > *d {
                            cheats.push((
                                (x as i32, y as i32),
                                nei2,
                                Some((d_nei - d - 2) as usize),
                            ));
                        }
                    }
                }
            };
        }
    }
    cheats
}

fn manhattan_dist(from: (i32, i32), to: (i32, i32)) -> usize {
    ((from.0 - to.0).abs() + (from.1 - to.1).abs()) as usize
}
fn cheat_n_times(
    dists: &Vec<Vec<Option<usize>>>,
    n: i32,
) -> Vec<((i32, i32), (i32, i32), Option<usize>)> {
    let mut cheats = vec![];
    for (x, line) in dists.iter().enumerate() {
        for (y, dist) in line.iter().enumerate() {
            if let Some(d) = dist {
                for xx in (x as i32 - n)..=(x as i32 + n) {
                    for yy in (y as i32 - n)..=(y as i32 + n) {
                        if xx < 0
                            || xx as usize >= dists.len()
                            || yy < 0
                            || yy as usize >= dists[0].len()
                        {
                            continue;
                        }
                        let md = manhattan_dist((x as i32, y as i32), (xx as i32, yy as i32));
                        if md <= n as usize {
                            let d_nei = dists[xx as usize][yy as usize].unwrap_or(0);
                            if d_nei > *d + md {
                                cheats.push((
                                    (x as i32, y as i32),
                                    (xx as i32, yy as i32),
                                    Some(d_nei - d - md),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    cheats
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, map) = parse_input(input);
    let dists = dijkstra(start, end, &map);

    let cheats = cheat_once(&dists);
    cheats
        .iter()
        .map(|(_a, _b, c)| c.unwrap())
        .filter(|&c| c >= 100)
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, end, map) = parse_input(input);
    let dists = dijkstra(start, end, &map);
    let cheats = cheat_n_times(&dists, 20);

    cheats
        .iter()
        .map(|(_a, _b, c)| c.unwrap())
        .filter(|&c| c >= 100)
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
