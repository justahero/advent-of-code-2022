//! Day 12: Hill Climbing Algorithm

use itertools::Itertools;

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
        self
    }

    pub fn traverse_steps(&self) -> usize {
        0
    }

    fn neighbors(&self, pos: &Pos) -> impl Iterator<Item = &char> + '_ {
        self.cells.iter()
    }
}

fn part1(grid: &Grid) -> usize {
    grid.traverse_steps()
}

fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .fold(Grid::new(), |grid, line| grid.add_line(line.chars()));

    grid
}

fn main() {}

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
