//! Day 08: Treetop Tree House

use std::fmt::{Display, Formatter};

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

/// A struct to hold an Iterator used with [`TreeGrid::steps`].
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
        self.start = self.start + self.dir;
        self.grid.get(self.start)
    }
}

/// Iterates over all inner trees in the tree grid.
struct TreeIter<'a> {
    grid: &'a TreeGrid,
    pos: Pos,
}

impl<'a> TreeIter<'a> {
    pub fn new(grid: &'a TreeGrid) -> Self {
        Self {
            grid,
            pos: Pos::new(1, 1),
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = (Pos, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos + Pos::new(1, 0);
        let pos = if pos.x >= self.grid.width as i32 - 1 {
            Pos::new(1, pos.y + 1)
        } else {
            pos
        };
        self.pos = pos;
        if pos.y >= self.grid.height as i32 - 1 {
            None
        } else {
            Some((pos, self.grid.get(pos).expect("Failed to get tree")))
        }
    }
}

/// Algorithm to scan a line of trees from a given tree height. As soon as we find a tree of same height or
/// above the remaining trees are hidden and do not count.
///
/// Examples:
/// ```
/// scan_trees(5, [3, 5, 3]) -> 2  //
/// ```
pub fn scan_trees(tree: u8, iter: impl Iterator<Item = u8>) -> usize {
    let mut count = 0;
    for neighbor in iter {
        count += 1;
        if neighbor >= tree {
            break;
        }
    }
    count
}

#[derive(Debug)]
struct TreeGrid {
    pub width: usize,
    pub height: usize,
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

impl TreeGrid {
    const DIRECTIONS: [Pos; 4] = [
        Pos::new(0, -1), // up
        Pos::new(0, 1),  // right
        Pos::new(-1, 0), // left
        Pos::new(1, 0),  // down
    ];

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
    pub fn visibles(&self) -> usize {
        self.trees.len() - self.invisibles()
    }

    /// Returns the score of the best scenic tree in the forest
    pub fn best_scenic(&self) -> Option<usize> {
        self.trees()
            .map(|(pos, tree)| {
                Self::DIRECTIONS.iter().fold(1, |product, &dir| {
                    product * scan_trees(tree, self.steps(pos, dir))
                })
            })
            .max()
    }

    /// Returns the number of invisible trees that cannot be seen from any side.
    pub fn invisibles(&self) -> usize {
        self.trees()
            .filter(|&(pos, tree)| {
                Self::DIRECTIONS
                    .iter()
                    .all(|&dir| self.steps(pos, dir).any(|neighbor| tree <= neighbor))
            })
            .count()
    }

    /// Returns an iterator over all inner trees
    fn trees(&self) -> TreeIter {
        TreeIter::new(self)
    }

    /// Returns a steps iterator from current pos to the given direction (up, left, right, down)
    fn steps(&self, pos: Pos, dir: Pos) -> Steps {
        Steps::new(self, pos, dir)
    }

    /// Get a tree by index
    fn get(&self, Pos { x, y }: Pos) -> Option<u8> {
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
            let trees = line
                .chars()
                .filter_map(|c| c.to_string().parse::<u8>().ok())
                .collect::<Vec<_>>();
            grid.add_line(trees)
        })
}

/// Returns the number of visible trees
fn part1(grid: &TreeGrid) -> usize {
    grid.visibles()
}

/// Returns the score of the best scenic tree
fn part2(grid: &TreeGrid) -> usize {
    grid.best_scenic()
        .expect("Failed to find the most scenic tree")
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
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
    fn evaluate_scenic_iter() {
        assert_eq!(1, scan_trees(5, [5u8, 2].into_iter()));
        assert_eq!(1, scan_trees(5, [3u8].into_iter()));
        assert_eq!(2, scan_trees(5, [1u8, 2].into_iter()));
        assert_eq!(2, scan_trees(5, [3u8, 5, 3].into_iter()));
    }

    #[test]
    fn check_part1() {
        let tree_grid = parse(INPUT);
        assert_eq!(21, part1(&tree_grid));
    }

    #[test]
    fn check_part2() {
        let tree_grid = parse(INPUT);
        assert_eq!(8, part2(&tree_grid));
    }
}
