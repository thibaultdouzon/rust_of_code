use core::panic;
use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Top { x: usize, left: usize, right: usize },
    Right { y: usize, top: usize, bottom: usize },
    Bottom { x: usize, left: usize, right: usize },
    Left { y: usize, top: usize, bottom: usize },
}

fn side_neighbour(s: &Side) -> Vec<Side> {
    let mut nei = vec![];
    match s {
        Side::Top { x, left, right } => {
            if *left > 0 {
                nei.push(Side::Top {
                    x: *x,
                    left: *left - 1,
                    right: *right - 1,
                });
            }
            nei.push(Side::Top {
                x: *x,
                left: *left + 1,
                right: *right + 1,
            });
        }
        Side::Right { y, top, bottom } => {
            if *top > 0 {
                nei.push(Side::Right {
                    y: *y,
                    top: *top - 1,
                    bottom: *bottom - 1,
                });
            }
            nei.push(Side::Right {
                y: *y,
                top: *top + 1,
                bottom: *bottom + 1,
            });
        }
        Side::Bottom { x, left, right } => {
            if *left > 0 {
                nei.push(Side::Bottom {
                    x: *x,
                    left: *left - 1,
                    right: *right - 1,
                });
            }
            nei.push(Side::Bottom {
                x: *x,
                left: *left + 1,
                right: *right + 1,
            });
        }
        Side::Left { y, top, bottom } => {
            if *top > 0 {
                nei.push(Side::Left {
                    y: *y,
                    top: *top - 1,
                    bottom: *bottom - 1,
                });
            }
            nei.push(Side::Left {
                y: *y,
                top: *top + 1,
                bottom: *bottom + 1,
            });
        }
    }
    nei
}

fn neighbour(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x < height - 1 {
        neighbours.push((x + 1, y));
    }
    if y < width - 1 {
        neighbours.push((x, y + 1));
    }
    neighbours
}
trait Sizeable {
    fn area(&self) -> usize;
    fn perimeter(&self) -> usize;
    fn sides(&self) -> usize;
}

#[allow(dead_code)]
#[derive(Debug)]
struct Region {
    name: char,
    positions: Vec<(usize, usize)>,
}

impl Sizeable for Region {
    fn area(&self) -> usize {
        self.positions.len()
    }

    fn perimeter(&self) -> usize {
        let position_set = self.positions.iter().collect::<HashSet<_>>();
        let mut perimeter = 0;
        for (x, y) in self.positions.iter() {
            if *x == 0 || !position_set.contains(&(x - 1, *y)) {
                perimeter += 1;
            }
            if !position_set.contains(&(x + 1, *y)) {
                perimeter += 1;
            }
            if *y == 0 || !position_set.contains(&(*x, y - 1)) {
                perimeter += 1;
            }
            if !position_set.contains(&(*x, y + 1)) {
                perimeter += 1;
            }
        }
        perimeter
    }

    fn sides(&self) -> usize {
        let mut raw_sides = HashSet::<Side>::new();
        let mut sides = vec![];

        for (x, y) in self.positions.iter() {
            if *x == 0 || !self.positions.contains(&(x - 1, *y)) {
                raw_sides.insert(Side::Top {
                    x: *x,
                    left: *y,
                    right: *y,
                });
            }
            if *y == 0 || !self.positions.contains(&(*x, y - 1)) {
                raw_sides.insert(Side::Left {
                    y: *y,
                    top: *x,
                    bottom: *x,
                });
            }
            if !self.positions.contains(&(x + 1, *y)) {
                raw_sides.insert(Side::Bottom {
                    x: *x,
                    left: *y,
                    right: *y,
                });
            }
            if !self.positions.contains(&(*x, y + 1)) {
                raw_sides.insert(Side::Right {
                    y: *y,
                    top: *x,
                    bottom: *x,
                });
            }
        }

        let mut seen_raw_sides = HashSet::<Side>::new();
        let mut to_process = VecDeque::new();
        for rs in raw_sides.iter() {
            if seen_raw_sides.contains(rs) {
                continue;
            }
            let mut side = rs.clone();
            to_process.push_back(rs.clone());
            seen_raw_sides.insert(rs.clone());
            while !to_process.is_empty() {
                let s = to_process.pop_front().unwrap();

                if side != s {
                    side = merge(&side, &s);
                }

                for n in side_neighbour(&s).iter() {
                    if raw_sides.contains(&n) && !seen_raw_sides.contains(&n) {
                        to_process.push_back(n.clone());
                        seen_raw_sides.insert(n.clone());
                    }
                }
            }
            sides.push(side);
        }

        sides.len()
    }
}

fn merge(side: &Side, raw_side: &Side) -> Side {
    match (side, raw_side) {
        (
            &Side::Top {
                x: sx,
                left: sleft,
                right: sright,
            },
            &Side::Top {
                x: rsx,
                left: rsleft,
                right: rsright,
            },
        ) if sx == rsx && (sright + 1 == rsleft || sleft - 1 == rsright) => Side::Top {
            x: sx,
            left: sleft.min(rsleft),
            right: sright.max(rsright),
        },
        (
            &Side::Left {
                y: sy,
                top: stop,
                bottom: sbottom,
            },
            &Side::Left {
                y: rsy,
                top: rstop,
                bottom: rsbottom,
            },
        ) if sy == rsy && (sbottom + 1 == rsbottom || stop - 1 == rstop) => Side::Left {
            y: sy,
            top: stop.min(rstop),
            bottom: sbottom.max(rsbottom),
        },
        (
            &Side::Bottom {
                x: sx,
                left: sleft,
                right: sright,
            },
            &Side::Bottom {
                x: rsx,
                left: rsleft,
                right: rsright,
            },
        ) if sx == rsx && (sright + 1 == rsleft || sleft - 1 == rsright) => Side::Bottom {
            x: sx,
            left: sleft.min(rsleft),
            right: sright.max(rsright),
        },
        (
            &Side::Right {
                y: sy,
                top: stop,
                bottom: sbottom,
            },
            &Side::Right {
                y: rsy,
                top: rstop,
                bottom: rsbottom,
            },
        ) if sy == rsy && (sbottom + 1 == rsbottom || stop - 1 == rstop) => Side::Right {
            y: sy,
            top: stop.min(rstop),
            bottom: sbottom.max(rsbottom),
        },
        _ => panic!("side {:?} | raw_side {:?}", side, raw_side),
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn grow_region(x: usize, y: usize, map: &Vec<Vec<char>>) -> Region {
    let mut positions = vec![];
    let mut to_process = VecDeque::new();
    let mut seen_positions = HashSet::<(usize, usize)>::new();
    to_process.push_back((x, y));
    seen_positions.insert((x, y));
    while !to_process.is_empty() {
        let (x, y) = to_process.pop_front().unwrap();
        positions.push((x, y));

        for (i, j) in neighbour(x, y, map.len(), map[0].len()) {
            if map[i][j] == map[x][y] && !seen_positions.contains(&(i, j)) {
                to_process.push_back((i, j));
                seen_positions.insert((i, j));
            }
        }
    }
    Region {
        name: map[x][y],
        positions: positions,
    }
}

fn get_regions(map: &Vec<Vec<char>>) -> Vec<Region> {
    let mut seen_positions = HashSet::<(usize, usize)>::new();
    let height = map.len();
    let width = map[0].len();
    let mut regions = vec![];

    for i in 0..height {
        for j in 0..width {
            if seen_positions.contains(&(i, j)) {
                continue;
            }
            let region = grow_region(i, j, map);
            seen_positions.extend(region.positions.iter());
            regions.push(region);
        }
    }
    regions
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let regions = get_regions(&map);

    regions
        .iter()
        .map(|r| r.area() * r.perimeter())
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_input(input);
    let regions = get_regions(&map);

    regions
        .iter()
        .map(|r| r.area() * r.sides())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
