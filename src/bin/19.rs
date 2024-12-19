use std::collections::HashMap;
advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut recepies: Vec<String> = blocks[0].split(", ").map(|word| word.to_string()).collect();
    recepies.sort();

    let designs = blocks[1].lines().map(|word| word.to_string()).collect();

    (recepies, designs)
}

fn tokenizable<'a>(
    word: &'a str,
    recepies: &Vec<String>,
    memory: &mut HashMap<&'a str, usize>,
) -> usize {
    if memory.contains_key(word) {
        return *memory.get(word).unwrap();
    }
    if word.len() == 0 {
        return 1;
    }

    let mut found = 0;
    for recepy in recepies.iter() {
        if word.starts_with(recepy.as_str()) {
            let add = tokenizable(&word[recepy.len()..], recepies, memory);
            found += add;
        }
    }
    memory.entry(word).insert_entry(found);
    found
}

pub fn part_one(input: &str) -> Option<usize> {
    let (recepies, designs) = parse_input(input);
    let mut memory: HashMap<&str, usize> = HashMap::new();
    designs
        .iter()
        .filter(|word| tokenizable(word, &recepies, &mut memory) > 0)
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (recepies, designs) = parse_input(input);
    let mut memory: HashMap<&str, usize> = HashMap::new();
    designs
        .iter()
        .map(|word| tokenizable(word, &recepies, &mut memory))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
