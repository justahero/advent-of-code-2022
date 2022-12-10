//! Day 10: Cathode-Ray Tube

use anyhow::anyhow;

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
    /// The start cycle counter that is increased for each instructions
    cycle_count: i32,
    /// The cycle interval to measure signal strength of 'X'
    interval: u32,
    /// The accumulated register 'X'
    x: i32,
}

impl VideoSystem {
    pub fn new(start_count: i32, interval: u32) -> Self {
        Self {
            cycle_count: start_count,
            interval,
            x: 1,
        }
    }

    /// Run all given instructions on the video system, take care of cycles and measure X at certain intervals.
    pub fn run(&self, instructions: &[Instruction]) -> u64 {
        0
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, instructions) = separated_list1(newline, Instruction::parse)(input)
        .map_err(|e| anyhow!("Failed to parse input: {}", e))?;
    Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> u64 {
    let video_system = VideoSystem::new(-20, 40);
    video_system.run(instructions)
}

fn main() -> anyhow::Result<()> {
    let instructions = parse(include_str!("input.txt"))?;
    println!("Part 1: {}", part1(&instructions));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_part1() {
        let instructions = parse(INPUT).expect("Failed to parse input.");
        assert_eq!(13140, part1(&instructions));
    }
}
