//! Day 08: Rope Bridge

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    Right = 0,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Checks if this pos is adjacent / touching the other
    pub fn touches(&self, rhs: &Pos) -> bool {
        (self.x - rhs.x).abs() <= 1 && (self.y - rhs.y).abs() <= 1
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

#[derive(Debug)]
struct Grid {
    /// The head position
    head: Pos,
    /// Keeps the position where head touched tail the last time
    last_head: Pos,
    /// The tail position
    tail: Pos,
    /// The list of all visited position by tail
    trail: HashSet<Pos>,
}

impl Grid {
    const DIRECTIONS: [Pos; 4] = [
        Pos::new(1, 0),  // right
        Pos::new(0, -1), // up
        Pos::new(-1, 0), // left
        Pos::new(0, 1),  // down
    ];

    pub fn new() -> Self {
        let trail = HashSet::from([Pos::new(0, 0)]);
        Self {
            head: Pos::new(0, 0),
            last_head: Pos::new(0, 0),
            tail: Pos::new(0, 0),
            trail,
        }
    }

    /// Apply the given step, move head and tail, keep track of where tail stepped.
    pub fn step(&mut self, m: &Move) {
        for _ in 0..m.steps {
            let dir = Self::DIRECTIONS[m.dir as usize];
            self.last_head = self.head;
            self.head += dir;

            // update tail position if head moves away
            if !self.tail.touches(&self.head) {
                self.tail = self.last_head;
                self.trail.insert(self.tail);
            }

            // self.print_trail();
        }
    }

    /*
    pub fn print_trail(&self) {
        let (minx, maxx) = (-2, 7);
        let (miny, maxy) = (-6, 3);

        for y in miny..=maxy {
            for x in minx..=maxx {
                let pos = Pos::new(x, y);
                if self.tail == pos {
                    if self.head != self.tail {
                        print!("T");
                    }
                } else if self.head == pos {
                    print!("H");
                } else {
                    match self.trail.get(&pos) {
                        Some(_) => print!("#"),
                        None => print!("."),
                    }
                }
            }
            println!();
        }
        println!();
    }
    */
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
    // grid.print_trail();
    grid.trail.len()
}

fn main() {
    let moves = parse(include_str!("input.txt"));
    let count = part1(&moves);
    assert!(count > 6262);
    println!("Part 1: {}", part1(&moves));
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
