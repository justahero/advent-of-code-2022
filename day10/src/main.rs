//! Day 10: Cathode-Ray Tube

use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, combinator::map,
    multi::separated_list1, sequence::preceded, IResult,
};

fn parse_add(input: &str) -> IResult<&str, i32> {
    preceded(tag("addx "), parse_number)(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    nom::character::complete::i32(input)
}

#[derive(Debug)]
struct Add(i32);

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(Add),
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("noop"), |_| Instruction::Noop),
            map(parse_add, |n| Instruction::Add(Add(n))),
        ))(input)
    }
}

#[derive(Debug)]
struct VideoSystem {
    /// The number of cycles
    cycle_count: u32,
    /// The accumulated register 'X'
    x: i32,
}

impl VideoSystem {
    pub fn new() -> Self {
        Self {
            cycle_count: 0,
            x: 1,
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, instructions) = separated_list1(newline, Instruction::parse)(input).unwrap();
    Ok(instructions)
}

fn part1() -> usize {
    0
}

fn main() {
    let _ = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_part1() {
        let _ = parse(INPUT);
        assert_eq!(0, part1());
    }
}
