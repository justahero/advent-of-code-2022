//! Day 12: Hill Climbing Algorithm

use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
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
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            self.start = Pos::new(x as i32, self.height as i32);
        };
        if let Some(x) = row.iter().position(|&c| c == 'E') {
            self.end = Pos::new(x as i32, self.height as i32);
        }

        self.cells.append(&mut row);
        self.height += 1;
        self
    }

    /// Breadth depth search
    fn find_path(&self) -> usize {
        let mut visited: Vec<Pos> = vec![self.start];
        let mut positions: VecDeque<Pos> = VecDeque::new();

        positions.push_back(self.start);
        visited.push(self.start);

        while let Some(pos) = positions.pop_front() {
            let current = self.get(pos).expect("Failed to get height.");

            if current == 'E' {
                continue;
            }

            for neighbor in self.neighbors(pos).into_iter() {
                if visited.contains(&neighbor) {
                    continue;
                }

                if let Some(elevation) = self.get(neighbor) {
                    if current == 'z' && elevation == 'E' {
                        break;
                    }

                    // can only pass one step up but lower is always possible
                    // println!("  CMP: {} {} , {} {}", pos, neighbor, current, elevation);
                    if (current as u8) <= (elevation as u8) - 1 {
                        // println!("    visiting");
                        visited.push(neighbor);
                        positions.push_back(neighbor);
                    }
                }
            }
        }

        println!("VISITED: {:?}", visited);

        visited.len()
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
    println!("{}", grid);
    grid.find_path()
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
    println!("Part 1: {}", part1(&grid));
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
