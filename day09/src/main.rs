//! Day 08: Rope Bridge

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl From<&str> for Direction {
    fn from(d: &str) -> Self {
        match d {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => panic!("Unsupported input found"),
        }
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

#[derive(Debug)]
struct Grid {
    head: Pos,
    tail: Pos,
    trail: Vec<Pos>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            head: Pos::new(0, 0),
            tail: Pos::new(0, 0),
            trail: Vec::new(),
        }
    }

    /// Apply the given step, move head and tail, keep track of where tail stepped.
    pub fn step(&mut self, m: &Move) {
        todo!()
    }
}

#[derive(Debug)]
struct Move {
    pub dir: Direction,
    pub steps: u32,
}

impl Move {
    pub fn new(dir: Direction, steps: u32) -> Self {
        Self { dir, steps }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (dir, steps) = line.split_once(" ").expect("Failed to parse line.");
            Move::new(
                dir.into(),
                steps.parse::<u32>().expect("Failed to parse number"),
            )
        })
        .collect::<Vec<_>>()
}

/// Returns the number of visible trees
fn part1(moves: &[Move]) -> usize {
    let mut grid = Grid::new();
    for m in moves {
        grid.step(m);
    }
    grid.trail.len()
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&grid));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "#;

    #[test]
    fn check_part1() {
        let grid = parse(INPUT);
        assert_eq!(13, part1(&grid));
    }
}
