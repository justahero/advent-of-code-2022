//! Day 23: Unstable Diffusion

use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
#[repr(u8)]
enum Dir {
    NW = 0,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    elves: HashSet<Pos>,
}

impl Grid {
    pub fn new(elves: HashSet<Pos>) -> Self {
        Self { elves }
    }

    pub fn neighbors(&self, pos: &Pos) -> impl Iterator<Item = Pos> + '_ {
        const neighbors: [(i32, i32); 8] = [
            (-1, -1), // NW
            (0, -1),  // N
            (1, -1),  // NE
            (1, 0),   // E
            (1, 1),   // SE
            (0, 1),   // S
            (-1, 1),  // SW
            (-1, 0),  // W
        ];

        neighbors.iter().map(|&(x, y)| Pos::new(x, y))
    }

    pub fn get(&self, pos: Pos) -> Option<&Pos> {
        self.elves.get(&pos)
    }
}

fn part1(grid: Grid) -> i64 {
    for _ in 0..10 {
        // first half of the move
        for elf in &grid.elves {
            let neighbors = grid.neighbors(elf).collect_vec();

        }
    }

    0
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> Grid {
    let elves = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some(Pos::new(col as i32, row as i32)),
                    _ => None,
                })
        })
        .collect();

    Grid::new(elves)
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    ";

    #[test]
    fn check_part1() {
        assert_eq!(110, part1(parse(INPUT)));
    }
}
