//! Day 03: Rucksack Reorganization

use std::collections::HashSet;

use itertools::Itertools;

/// Parses the contents of the rucksack compartments
///
/// The compartments are filled evenly
/// all: 'vJrwpWtwJgWrhcsFMMfFFhFp'
/// 1st: 'vJrwpWtwJgWr'
/// 2nd:             'hcsFMMfFFhFp'
///
fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect_vec()
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 96,
        'A'..='Z' => c as u32 - 38,
        _ => panic!("Unknown char found"),
    }
}

/// Finds all duplicate items, calculates the total priority of all.
/// Each rucksack is split into two same size compartments.
fn part1(rucksacks: &[&str]) -> u32 {
    rucksacks
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();
            left.intersection(&right).next().unwrap().clone()
        })
        .map(get_priority)
        .sum()
}

/// Find the badge in each group of elves. Each group consists of three elves. All three
/// elves' rucksack contain the same badge.
fn part2(rucksacks: &[&str]) -> u32 {
    rucksacks
        .iter()
        .map(|&s| s.chars().collect::<HashSet<_>>())
        .chunks(3)
        .into_iter()
        .map(|group| {
            group
                .reduce(|result: HashSet<char>, rhs| result.intersection(&rhs).cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone()
        })
        .map(get_priority)
        .sum()
}

fn main() {
    let input = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{parse, part1, part2};

    const INPUT: &str = r#"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(157, part1(&parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(70, part2(&parse(INPUT)));
    }
}
