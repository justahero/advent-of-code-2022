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
enum Instruction {
    Noop,
    Add(i32),
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
            map(parse_add, |n| Instruction::Add(n)),
        ))(input)
    }
}

#[derive(Debug)]
struct VideoSystem {
    /// The start cycle counter that is increased for each instructions
    start_interval: u32,
    /// The cycle interval to measure signal strength of 'X'
    interval: u32,
}

impl VideoSystem {
    pub fn new(start_interval: u32, interval: u32) -> Self {
        Self {
            start_interval,
            interval,
        }
    }

    /// Run all given instructions on the video system, take care of cycles and measure X at certain intervals.
    pub fn run(&self, instructions: &[Instruction]) -> Vec<(i32, i32)> {
        let mut signal_strengths = Vec::new();
        let mut cycle_count = 0;
        let mut interval = self.start_interval;
        let mut x = 1;

        for instruction in instructions {
            for _ in 0..instruction.cycles() {
                cycle_count += 1;
                interval += 1;

                // in case an interval is hit store current signal / cycle pair
                if interval >= self.interval {
                    interval = 0;
                    signal_strengths.push((cycle_count, x));
                }
            }
            // apply operation
            if let Instruction::Add(value) = instruction {
                x += *value;
            }
        }

        signal_strengths
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<Instruction>> {
    let (_, instructions) = separated_list1(newline, Instruction::parse)(input)
        .map_err(|e| anyhow!("Failed to parse input: {}", e))?;
    Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> i64 {
    let video_system = VideoSystem::new(20, 40);
    let strengths = video_system.run(instructions);

    strengths
        .iter()
        .map(|(cycle, value)| *cycle as i64 * *value as i64)
        .sum::<i64>()
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
        let system = VideoSystem::new(20, 40);
        let signal_strengths = system.run(&instructions);

        assert_eq!(
            vec![
                (20, 21),
                (60, 19),
                (100, 18),
                (140, 21),
                (180, 16),
                (220, 18)
            ],
            signal_strengths
        );
        assert_eq!(13140, part1(&instructions));
    }
}
