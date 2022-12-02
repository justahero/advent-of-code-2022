//! Day 02: Rock Paper Scissors

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand(u8);

impl Hand {
    const ROCK: Hand = Hand(0);
    const PAPER: Hand = Hand(1);
    const SCISSORS: Hand = Hand(2);

    /// Pick next hand, it wins
    pub fn win(&self) -> Hand {
        Self((self.0 + 1) % 3)
    }

    /// Pick previous hand, it loses
    pub fn lose(&self) -> Hand {
        Self((self.0 + 2) % 3)
    }
}

impl From<&str> for Hand {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::ROCK,
            "B" | "Y" => Self::PAPER,
            "C" | "Z" => Self::SCISSORS,
            _ => panic!("Unknown char found"),
        }
    }
}

impl Hand {
    /// Two players show their hands, outcome for right hand is counted.
    pub fn play(left: Hand, right: Hand) -> u32 {
        let total = match (left, right) {
            (Hand::ROCK, Hand::PAPER) => 6,
            (Hand::ROCK, Hand::SCISSORS) => 0,
            (Hand::PAPER, Hand::ROCK) => 0,
            (Hand::PAPER, Hand::SCISSORS) => 6,
            (Hand::SCISSORS, Hand::ROCK) => 6,
            (Hand::SCISSORS, Hand::PAPER) => 0,
            _ => 3,
        };
        total + right.0 as u32 + 1
    }

    /// The `right` parameter determines the outcome, map to associated hand.
    pub fn play2(left: &str, right: &str) -> u32 {
        let left = Hand::from(left);
        match right {
            "X" => Self::play(left, left.lose()),
            "Y" => Self::play(left, left),
            "Z" => Self::play(left, left.win()),
            _ => panic!("Unknown input found"),
        }
    }
}

/// Parses the strategy guide from the input as list of hands to play
fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|line| line.split(' ').collect_tuple())
        .collect_vec()
}

fn part1(hands: &[(&str, &str)]) -> u32 {
    hands
        .iter()
        .map(|&(l, r)| Hand::play(Hand::from(l), Hand::from(r)))
        .sum()
}

fn part2(hands: &[(&str, &str)]) -> u32 {
    hands.iter().map(|&(l, r)| Hand::play2(l, r)).sum()
}

fn main() {
    let hands = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&hands));
    println!("Part 2: {}", part2(&hands));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    const INPUT: &str = r#"
        A Y
        B X
        C Z
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(15, part1(&parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(12, part2(&parse(INPUT)));
    }
}
