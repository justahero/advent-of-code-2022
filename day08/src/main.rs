//! Day 08: Treetop Tree House

use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct TreeGrid {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

impl Display for TreeGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.trees.chunks(self.width) {
            for tree in row {
                write!(f, "{}", tree)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Steps<'a> {
    grid: &'a TreeGrid,
    start: Pos,
    dir: Pos,
}

impl<'a> Steps<'a> {
    pub fn new(grid: &'a TreeGrid, start: Pos, dir: Pos) -> Self {
        Self { grid, start, dir }
    }
}

impl<'a> Iterator for Steps<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start + self.dir;

        match self.grid.get(start) {
            Some(value) => {
                self.start = start;
                Some(value)
            }
            None => None
        }
    }
}

impl TreeGrid {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            trees: Vec::new(),
        }
    }

    /// Adds a row of trees to the grid
    pub fn add_line(mut self, mut line: Vec<u8>) -> Self {
        self.width = line.len();
        self.height += 1;
        self.trees.append(&mut line);
        self
    }

    /// Checks all trees and counts all visible ones by determine all invisible trees first.
    pub fn visibles(&self) -> u64 {
        self.trees.len() as u64 - self.invisibles()
    }

    pub fn invisibles(&self) -> u64 {
        let mut invisibles = 0;
        const DIRECTIONS: [Pos; 4] = [Pos::new(1, 0), Pos::new(0, 1), Pos::new(-1, 0), Pos::new(0, -1)];

        for y in 1..(self.height - 1) as i32 {
            for x in 1..(self.width - 1) as i32 {
                let tree = self.get(Pos::new(x, y)).expect(format!("No tree found at {}x{}?", x, y).as_str());

                // A tree is invisible when it cannot be seen from any direction
                let invisible = DIRECTIONS.iter().all(|&dir| self.steps(Pos::new(x, y), dir).any(|neighbor| tree <= neighbor));
                if invisible {
                    invisibles += 1;
                }
            }
        }
        invisibles
    }

    fn steps(&self, pos: Pos, dir: Pos) -> Steps {
        Steps::new(self, pos, dir)
    }

    fn get(&self, Pos{ x, y }: Pos) -> Option<u8> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.trees[(y * self.width as i32 + x) as usize])
        } else {
            None
        }
    }
}

fn parse(input: &str) -> TreeGrid {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .fold(TreeGrid::new(), |grid, line: &str| {
            let trees = line.chars().filter_map(|c| c.to_string().parse::<u8>().ok()).collect::<Vec<_>>();
            grid.add_line(trees)
        })
}

/// Returns the number of visible trees
fn part1(grid: &TreeGrid) -> u64 {
    grid.visibles()
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&grid));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
        30373
        25512
        65332
        33549
        35390
    "#;

    #[test]
    fn check_part1() {
        let tree_grid = parse(INPUT);
        assert_eq!(21, part1(&tree_grid));
    }
}
