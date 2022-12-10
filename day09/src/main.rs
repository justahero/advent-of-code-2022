//! Day 09: Rope Bridge

use std::{cmp::Ordering, collections::HashSet};

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

    /// Returns a directional Vector to advance one knot to the other
    pub fn get_dir(&self, target: &Pos) -> Pos {
        let x = match target.x.cmp(&self.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y = match target.y.cmp(&self.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        Pos::new(x, y)
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
    /// The rope consisting of knots
    rope: Vec<Pos>,
    /// The list of all visited position by the last tail element (end of rope)
    trail: HashSet<Pos>,
    /// The number of knots, length of the rope
    num_knots: usize,
}

impl Grid {
    const DIRECTIONS: [Pos; 4] = [
        Pos::new(1, 0),  // right
        Pos::new(0, -1), // up
        Pos::new(-1, 0), // left
        Pos::new(0, 1),  // down
    ];

    pub fn new(num_knots: usize) -> Self {
        // All knots start on position (0, 0)
        let trail = HashSet::from([Pos::new(0, 0)]);
        let rope = std::iter::repeat(Pos::new(0, 0))
            .take(num_knots)
            .collect::<Vec<_>>();

        Self {
            rope,
            trail,
            num_knots,
        }
    }

    /// Apply the given distance in steps, move head and tail rope, keep track of where the tail stepped.
    pub fn step(&mut self, m: &Move) {
        // get the direction to move
        let dir = Self::DIRECTIONS[m.dir as usize];

        for _ in 0..m.steps {
            // update the head
            self.rope[0] += dir;

            // check for all other knots if they need to update their position
            for index in 1..self.num_knots {
                let head = self.rope[index - 1];
                let tail = self.rope[index];

                if !tail.touches(&head) {
                    self.rope[index] += head.get_dir(&tail);
                }
            }

            // Add the last knot (end of rope) to trail
            if let Some(last) = self.rope.last() {
                self.trail.insert(*last);
            }
        }
    }

    pub fn print_trail(&self) {
        let (minx, maxx) = (-2, 7);
        let (miny, maxy) = (-6, 3);

        for y in miny..=maxy {
            for x in minx..=maxx {
                let pos = Pos::new(x, y);

                // in case the pos is part of the rope
                if let Some(index) = self.rope.iter().position(|&p| p == pos) {
                    if index == 0 {
                        print!("H");
                    } else {
                        print!("{}", index);
                    }
                } else if pos == Pos::new(0, 0) {
                    print!("s");
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
    let mut grid = Grid::new(2);
    for m in moves {
        grid.step(m);
    }
    grid.trail.len()
}

/// Returns the number of visible trees
fn part2(moves: &[Move]) -> usize {
    let mut grid = Grid::new(10);
    for m in moves {
        grid.step(m);
    }
    grid.trail.len()
}

fn main() {
    let moves = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&moves));
    println!("Part 2: {}", part2(&moves));
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

    #[test]
    fn check_part2() {
        let grid = parse(INPUT);
        assert_eq!(1, part2(&grid));
    }

    #[test]
    fn check_longer_part2() {
        let input = r#"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "#;
        let grid = parse(input);
        assert_eq!(36, part2(&grid));
    }
}
