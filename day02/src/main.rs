//! Day 02: Rock Paper Scissors

use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown char found"),
        }
    }
}

impl Hand {
    /// Two players show their hands, outcome for right hand is counted.
    pub fn play(left: &Hand, right: &Hand) -> u32 {
        let total = match (left, right) {
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scissors) => 0,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Paper, Hand::Scissors) => 6,
            (Hand::Scissors, Hand::Rock) => 6,
            (Hand::Scissors, Hand::Paper) => 0,
            _ => 3,
        };
        total + right.value()
    }

    pub fn value(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

/// Parses the strategy guide from the input as list of hands to play
fn parse(input: &str) -> Vec<(Hand, Hand)> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|line| line.split(' ').map(Hand::from).collect_tuple())
        .collect_vec()
}

fn part1(hands: &[(Hand, Hand)]) -> u32 {
    hands.iter().map(|(l, r)| Hand::play(l, r)).sum::<u32>()
}

fn main() {
    let total = part1(&parse(include_str!("input.txt")));

    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    const INPUT: &str = r#"
        A Y
        B X
        C Z
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(15, part1(&parse(INPUT)));
    }
}
