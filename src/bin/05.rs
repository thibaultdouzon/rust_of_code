use itertools::Itertools;
advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: Vec<Vec<i32>> = Vec::with_capacity(100);

    for _ in 0..100 {
        rules.push(Vec::new());
    }

    let (rule_str, book_str) = input.split_once("\n\n").unwrap();
    for line in rule_str.lines() {
        if let Some((i1, i2)) = line
            .split("|")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect_tuple()
        {
            rules[i1 as usize].push(i2); // DefaultDict here
        } else {
            panic!("Invalid rule: {}", line);
        }
    }

    for rule in rules.iter_mut() {
        rule.sort();
    }

    let mut books = Vec::new();
    for line in book_str.lines() {
        books.push(line.split(",").map(|s| s.parse().unwrap()).collect());
    }

    (rules, books)
}

fn check_rules_book(book: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for (i, &num1) in book.iter().enumerate() {
        let fit = book
            .iter()
            .skip(i + 1)
            .all(|&num2| rules[num1 as usize].binary_search(&num2).is_ok());
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

fn correct_order<'a>(
    book: &'a mut Vec<i32>,
    beg_idx: usize,
    rules: &Vec<Vec<i32>>,
) -> &'a mut Vec<i32> {
    if book.len() == beg_idx + 1 {
        return book;
    }
    // Topological sort
    let mut first_elem_idx = 0usize;
    for (i, &elem1) in book.iter().enumerate().skip(beg_idx) {
        if book
            .iter()
            .skip(i + 1)
            .all(|&elem2| rules[elem1 as usize].contains(&elem2))
        {
            first_elem_idx = i;
            break;
        }
    }
    book.swap(beg_idx, first_elem_idx);

    correct_order(book, beg_idx + 1, rules);

    book
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut books) = parse_input(input);
    books
        .iter_mut()
        .filter(|book| !check_rules_book(book, &rules))
        .map(|book| correct_order(book, 0usize, &rules))
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
