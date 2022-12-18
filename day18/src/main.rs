//! Day 18: Boiling Boulders

use std::fmt::{Display, Formatter};

use anyhow::anyhow;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cube([i32; 3]);

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0[0], self.0[1], self.0[2])
    }
}

impl TryFrom<&str> for Cube {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (x, y, z) = line
            .splitn(3, ",")
            .collect_tuple()
            .ok_or(anyhow!("Failed to parse"))?;

        Ok(Self([
            x.parse::<i32>()?,
            y.parse::<i32>()?,
            z.parse::<i32>()?,
        ]))
    }
}

fn part1(cubes: Vec<Cube>) -> usize {
    // for each fixed plane

    // 1,1,1 & 2,1,1
    let total_adjacent = 0;

    // might be very inefficient, use some sorting first?
    cubes.iter().combinations(2).filter(|cubes| {
        let (lhs, rhs) = (cubes[0], cubes[1]);
        true
    }).inspect(|cubes| println!("Neighbors: {} {}", cubes[0], cubes[1])).count();

    0
}

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| Cube::try_from(line).ok())
        .collect_vec()
}

fn main() {
    let cubes = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(cubes.clone()));
    // println!("Part 2: {}", part2(jets));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    ";

    #[test]
    fn check_part1() {
        assert_eq!(64, part1(parse(INPUT)));
    }
}
