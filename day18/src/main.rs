//! Day 18: Boiling Boulders

use std::{collections::HashSet, ops::Index};

use anyhow::anyhow;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec3([i32; 3]);

impl Default for Vec3 {
    fn default() -> Self {
        Self([0, 0, 0])
    }
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self([x, y, z])
    }

    fn in_bounds(&self, bounds: &Bounds) -> bool {
        let (mins, maxs) = (bounds[0], bounds[1]);
        self[0] >= mins[0] - 1
            && self[0] <= maxs[0] + 1
            && self[1] >= mins[1] - 1
            && self[1] <= maxs[1] + 1
            && self[2] >= mins[2] - 1
            && self[2] <= maxs[2] + 1
    }
}

impl From<&(i32, i32, i32)> for Vec3 {
    fn from(&(x, y, z): &(i32, i32, i32)) -> Self {
        Self([x, y, z])
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs[0];
        self.0[1] += rhs[1];
        self.0[2] += rhs[2];
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl Vec3 {
    const DIRECTIONS: [(i32, i32, i32); 6] = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    /// Returns the list of neighbors as iterator
    pub fn neighbors(&self) -> Vec<Vec3> {
        Self::DIRECTIONS
            .iter()
            .map(|&(x, y, z)| *self + Vec3::new(x, y, z))
            .collect_vec()
    }
}

impl Index<usize> for Vec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl TryFrom<&str> for Vec3 {
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

#[derive(Debug)]
struct Bounds(Vec3, Vec3);

impl Bounds {
    pub fn bounds<'a>(cubes: impl Iterator<Item = &'a Vec3>) -> Self {
        let min = Vec3::new(i32::MAX, i32::MAX, i32::MAX);
        let max = Vec3::new(i32::MIN, i32::MIN, i32::MIN);

        let (min, max) = cubes.fold((min, max), |(mut min, mut max), cube| {
            min.0[0] = i32::min(min[0], cube[0]);
            min.0[1] = i32::min(min[1], cube[1]);
            min.0[2] = i32::min(min[2], cube[2]);
            max.0[0] = i32::max(max[0], cube[0]);
            max.0[1] = i32::max(max[1], cube[1]);
            max.0[2] = i32::max(max[2], cube[2]);
            (min, max)
        });
        Bounds(min, max)
    }
}

impl Index<usize> for Bounds {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// Using floodfill algorithm: https://en.wikipedia.org/wiki/Flood_fill
fn find_exposed(cubes: &[Vec3]) -> HashSet<Vec3> {
    let bounds = Bounds::bounds(cubes.iter());
    let start = Vec3::default();

    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut exposed = HashSet::new();

    visited.insert(start);

    while let Some(pos) = stack.pop() {
        for neighbor in pos.neighbors() {
            if cubes.contains(&neighbor) || !neighbor.in_bounds(&bounds) {
                continue;
            }

            if visited.insert(neighbor) {
                stack.push(neighbor);
                exposed.insert(neighbor);
            }
        }
    }

    exposed
}

/// Returns the number of sides that are exposed
fn part1(cubes: Vec<Vec3>) -> usize {
    cubes
        .iter()
        .flat_map(|cube| cube.neighbors())
        .filter(|cube| !cubes.contains(&cube))
        .count()
}

/// Returns the number of faces exposed, excluding air pockets
fn part2(cubes: Vec<Vec3>) -> usize {
    let exposed_cubes = find_exposed(&cubes);
    cubes
        .iter()
        .flat_map(|cube| cube.neighbors())
        .filter(|neighbor| exposed_cubes.contains(neighbor))
        .count()
}

fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| Vec3::try_from(line).ok())
        .collect_vec()
}

fn main() {
    let cubes = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(cubes.clone()));
    println!("Part 2: {}", part2(cubes));
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

    #[test]
    fn check_part2() {
        assert_eq!(58, part2(parse(INPUT)));
    }
}
