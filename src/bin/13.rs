use num::NumCast;
use num::ToPrimitive;
use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Game {
    a: Button,
    b: Button,
    prize: Button,
}

fn parse_input(input: &str) -> Vec<Game> {
    let re = Regex::new(r"[+=](?<X>\d+).*?[+=](?<Y>\d+)").unwrap();
    let mut games = vec![];
    for block in input.split("\n\n") {
        let matches: Vec<regex::Captures<'_>> = re.captures_iter(block).collect();
        games.push(Game {
            a: Button {
                x: matches[0].name("X").unwrap().as_str().parse().unwrap(),
                y: matches[0].name("Y").unwrap().as_str().parse().unwrap(),
            },
            b: Button {
                x: matches[1].name("X").unwrap().as_str().parse().unwrap(),
                y: matches[1].name("Y").unwrap().as_str().parse().unwrap(),
            },
            prize: Button {
                x: matches[2].name("X").unwrap().as_str().parse().unwrap(),
                y: matches[2].name("Y").unwrap().as_str().parse().unwrap(),
            },
        });
    }
    games
}

fn compute_dp(game: &Game) -> HashMap<(usize, usize), u32> {
    let mut dp_table = HashMap::<(usize, usize), u32>::new();
    dp_table.insert((0, 0), 0);

    for (button, val) in [(&game.a, 3), (&game.b, 1)].iter() {
        let possible_values = dp_table
            .iter()
            .map(|(&(x, y), &v)| ((x, y), v)) // Make a copy here
            .collect::<Vec<((usize, usize), u32)>>();
        for ((xx, yy), vv) in possible_values {
            let mut k = 1;
            while xx + button.x * k <= game.prize.x && yy + button.y * k <= game.prize.y {
                dp_table
                    .entry((xx + button.x * k, yy + button.y * k))
                    .and_modify(|v| *v = *v.min(&mut (vv + val * k as u32)))
                    .or_insert(vv + val * k as u32);

                k += 1;
            }
        }
    }
    dp_table
}

fn solve_game(game: &Game) -> Option<u64> {
    let dp_table = compute_dp(game);
    dp_table
        .get(&(game.prize.x, game.prize.y))
        .map(|v| *v as u64)
        .into()
}

fn solve_game_fast(game: &Game) -> Option<u64> {
    // println!("Game: {:?}", game);
    let det: f64 = ((game.a.x * game.b.y) as i128 - (game.a.y * game.b.x) as i128) as f64;
    if det.abs() < 0.0001 {
        return None;
    }

    let button_a =
        ((game.b.y * game.prize.x) as i128 - (game.b.x * game.prize.y) as i128) as f64 / det;
    let button_b =
        ((game.a.x * game.prize.y) as i128 - (game.a.y * game.prize.x) as i128) as f64 / det;

    if let (Some(a), Some(b)) = (button_a.to_usize(), button_b.to_usize()) {
        if a * game.a.x + b * game.b.x == game.prize.x
            && a * game.a.y + b * game.b.y == game.prize.y
        {
            return Some((a * 3 + b) as u64);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let games = parse_input(input);
    games
        .iter()
        .map(|game| solve_game(game))
        .filter(|res| res.is_some())
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let games = parse_input(input);
    games
        .iter()
        .map(|game| Game {
            a: game.a.clone(),
            b: game.b.clone(),
            prize: Button {
                x: game.prize.x + 10000000000000,
                y: game.prize.y + 10000000000000,
            },
        })
        .map(|game| solve_game_fast(&game))
        .filter(|res| res.is_some())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
