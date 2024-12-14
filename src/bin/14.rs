use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Robot {
                pos: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                vel: (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            }
        })
        .collect()
}

fn step(robot: &mut Robot, height: i32, width: i32) -> () {
    robot.pos.0 = (robot.pos.0 + robot.vel.0) % width;
    robot.pos.1 = (robot.pos.1 + robot.vel.1) % height;

    if robot.pos.0 < 0 {
        robot.pos.0 += width;
    }
    if robot.pos.1 < 0 {
        robot.pos.1 += height;
    }
}

fn quadrant_count(robots: &Vec<Robot>, height: i32, width: i32) -> (usize, usize, usize, usize) {
    let mut count = vec![0usize; 4];

    for r in robots.iter() {
        if r.pos.0 < width / 2 && r.pos.1 < height / 2 {
            count[0] += 1;
        } else if r.pos.0 > width / 2 && r.pos.1 < height / 2 {
            count[1] += 1;
        } else if r.pos.0 < width / 2 && r.pos.1 > height / 2 {
            count[2] += 1;
        } else if r.pos.0 > width / 2 && r.pos.1 > height / 2 {
            count[3] += 1;
        }
    }

    count.into_iter().collect_tuple().unwrap()
}

fn print_grid(robots: &Vec<Robot>, height: i32, width: i32) -> () {
    let mut grid = vec![vec!['0'; width as usize]; height as usize];
    for r in robots.iter() {
        grid[r.pos.1 as usize][r.pos.0 as usize] =
            (grid[r.pos.1 as usize][r.pos.0 as usize] as u8 + 1) as char;
    }

    for row in grid.iter() {
        let srow = row.iter().collect::<String>();

        println!("{:?}", srow.replace("0", " "));
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // let width = 101;
    // let height = 103;
    let width = 11;
    let height = 7;

    let mut robots = parse_input(input);
    for _ in 0..100 {
        for r in robots.iter_mut() {
            step(r, height, width);
        }
    }
    // print_grid(&robots, height, width);
    let count = quadrant_count(&robots, height, width);
    println!("{:?}", count);
    Some(count.0 * count.1 * count.2 * count.3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = 101;
    let height = 103;
    let mut robots = parse_input(input);
    for i in 0..10000 {
        for r in robots.iter_mut() {
            step(r, height, width);
        }
        if i % 101 == 778 % 101 {
            println!("{i} :");
            print_grid(&robots, height, width);
            println!("\n\n");
        }
    }
    Some(7344);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
