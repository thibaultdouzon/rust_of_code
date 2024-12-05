use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    vec,
};
advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules = HashMap::new();

    let (rule_str, book_str) = input.split_once("\n\n").unwrap();
    for line in rule_str.lines() {
        if let Some((i1, i2)) = line
            .split("|")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect_tuple()
        {
            rules.entry(i1).or_insert(HashSet::new()).insert(i2); // DefaultDict here
        } else {
            panic!("Invalid rule: {}", line);
        }
    }

    let mut books = Vec::new();
    for line in book_str.lines() {
        books.push(line.split(",").map(|s| s.parse().unwrap()).collect());
    }

    (rules, books)
}

fn check_rules_book(book: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, &num1) in book.iter().enumerate() {
        let fit = book.iter().skip(i + 1).all(|&num2| {
            rules.get(&num1).unwrap().contains(&num2) && !rules.get(&num2).unwrap().contains(&num1)
        });
        if !fit {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, books) = parse_input(input);
    books
        .iter()
        .filter(|&book| check_rules_book(book, &rules))
        .map(|book| book[book.len() / 2])
        .sum::<i32>()
        .try_into()
        .ok()
}

fn correct_order(book: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    if book.len() == 1 {
        return book.clone();
    }
    // Topological sort
    let mut first_elem = 0;
    for (i, &elem1) in book.iter().enumerate() {
        if book
            .iter()
            .skip(i + 1)
            .all(|&elem2| rules.get(&elem1).unwrap().contains(&elem2))
        {
            first_elem = elem1;
            break;
        }
    }
    let remaining = book
        .into_iter()
        .filter(|&elem| *elem != first_elem)
        .cloned()
        .collect();
    let mut solution = vec![first_elem];
    solution.extend(correct_order(&remaining, rules));
    solution
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, books) = parse_input(input);
    books
        .iter()
        .filter(|&book| !check_rules_book(book, &rules))
        .map(|book| correct_order(book, &rules))
        .map(|book| book[book.len() / 2])
        .sum::<i32>()
        .try_into()
        .ok()
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
