//! Day 12: Hill Climbing Algorithm

use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

use itertools::Itertools;
use pathfinding::directed::dijkstra;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd)]
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

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Grid {
    start: Pos,
    end: Pos,
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    const DIRECTIONS: [Pos; 4] = [
        Pos::new(1, 0),  // right
        Pos::new(0, 1),  // down
        Pos::new(-1, 0), // left
        Pos::new(0, -1), // up
    ];

    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            start: Pos::new(0, 0),
            end: Pos::new(0, 0),
            width: 0,
            height: 0,
        }
    }

    pub fn add_line(mut self, row: impl Iterator<Item = char>) -> Self {
        let mut row = row.collect_vec();
        self.width = row.len();

        if let Some(index) = row.iter().position(|&c| c == 'S') {
            self.start = Pos::new(index as i32, self.height as i32);
            row[index] = 'a';
        };
        if let Some(index) = row.iter().position(|&c| c == 'E') {
            self.end = Pos::new(index as i32, self.height as i32);
            row[index] = 'z';
        }

        self.cells.append(&mut row);
        self.height += 1;
        self
    }

    pub fn find_path(&self) -> Vec<Pos> {
        let result = dijkstra::dijkstra(
            &self.start,
            |&pos| {
                let height = self.get(pos).expect("Failed to get height") as i32;
                let mut positions = Vec::new();

                for neighbor in self.neighbors(pos).into_iter() {
                    let dest = self.get(pos).expect("Failed to get dest") as i32;
                    // println!("cmp {} = {}", height as char, dest as char);

                    // b != d
                    // b -> c
                    // b -> b
                    // b -> a
                    if dest - 1 <= height {
                        positions.push((neighbor, 1));
                    }
                }
                positions
            },
            |&p| p == self.end,
        );

        println!("RESULT: {:?}", result);

        result.unwrap().0
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let mut neighbors = Vec::new();
        for &dir in Self::DIRECTIONS.iter() {
            let neighbor = pos + dir;
            if let Some(_) = self.get(neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    fn get(&self, Pos { x, y }: Pos) -> Option<char> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.cells[(y * self.width as i32 + x) as usize])
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.chunks(self.width) {
            for c in row {
                write!(f, "{}", *c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(grid: &Grid) -> usize {
    println!("{}({}x{})", grid, grid.width, grid.height);
    let path = grid.find_path();
    println!("PATH: {:?}", path);
    path.len()
}

fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .fold(Grid::new(), |grid, line| grid.add_line(line.chars()));

    grid
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    let result = part1(&grid);
    println!("RESULT: {}", result);
    assert!(result < 3795);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "#;

    #[test]
    fn check_part1() {
        assert_eq!(31, part1(&parse(INPUT)));
    }
}
