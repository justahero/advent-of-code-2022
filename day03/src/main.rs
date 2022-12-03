//! Day 03: Rucksack Reorganization

use itertools::Itertools;

/// Parses the contents of the rucksack compartments
///
/// The compartments are filled evenly
/// all: 'vJrwpWtwJgWrhcsFMMfFFhFp'
/// 1st: 'vJrwpWtwJgWr'
/// 2nd:             'hcsFMMfFFhFp'
///
fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| line.split_at(line.len() / 2))
        .collect_vec()
}

/// Finds the duplicate item of both compartments
/// Assumes that there is exactly one.
fn find_duplicate(left: &str, right: &str) -> char {
    let left = left.chars().collect_vec();
    let right = right.chars().collect_vec();
    *left.iter().find(|c| right.contains(c)).unwrap()
}

/// Finds all duplicate items, calculates the total priority of all
fn part1(rucksacks: &[(&str, &str)]) -> u32 {
    rucksacks
        .iter()
        .map(|(l, r)| find_duplicate(l, r))
        .map(|item| match item {
            'a'..='z' => item as u32 - 96,
            'A'..='Z' => item as u32 - 38,
            _ => panic!("Unknown char found"),
        })
        .sum()
}

fn main() {
    let input = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

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
    fn check_part2() {}
}
