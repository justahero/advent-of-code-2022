//! Day 17: Pyroclastic Flow

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum Dir {
    Left = 0,
    Right,
    Down,
}

impl Dir {
    pub fn dir(&self) -> Pos {
        match self {
            Dir::Left => Pos::new(-1, 0),
            Dir::Right => Pos::new(1, 0),
            Dir::Down => Pos::new(0, -1),
        }
    }
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Unknown char found"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    rows: Vec<u8>,
    height: i32,
    width: i32,
}

impl Shape {
    pub fn new(bits: [u8; 4]) -> Self {
        let width = bits.iter().map(|row| row.count_ones()).max().unwrap() as i32;
        let height = bits.iter().filter(|&row| row.count_ones() > 0).count() as i32;

        Self {
            rows: bits.into_iter().rev().collect_vec(),
            height,
            width,
        }
    }

    pub fn row(&self, index: i32) -> u8 {
        assert!(0 <= index && index < self.rows.len() as i32);
        self.rows[index as usize]
    }

    pub fn intersects(&self, index: i32, rhs: u8) -> bool {
        assert!(0 <= index && index < self.height);
        (self.rows[index as usize] & rhs) > 0
    }
}

/// Returns true if rock was moved
fn move_rock(_stack: &[u8], _y: i32, _dir: &Dir, _shape: &mut Shape) -> bool {
    false
}

fn merge_stack(_stack: &[u8], _current_y: i32, _shape: &Shape) {
    // todo!()
}

fn print(stack: &[u8], current_y: i32, shape: &Shape) {
    println!("Stack: {}, y: {}", stack.len(), current_y);

    let length = stack.len() as i32 + 5;
    let y_range = current_y..current_y + shape.height;

    // draw top to bottom
    for y in (0..length).rev() {
        // draw each horizontal cell in row
        if y_range.contains(&y) {
            let row = shape.row(current_y - y);
            for x in 0..7 {
                if row & (1 << x) > 0 {
                    print!("@");
                } else {
                    print!(".");
                }
            }
        } else {
            for x in 0..7 {
                print!(".");
            }
        }

        println!();
    }
    println!("-------");
}

fn part1(jets: Vec<Dir>) -> usize {
    const DOWN: Dir = Dir::Down;

    // X bits are given in reverse order
    //
    // `0b111100` -> represents '@@@@' 2 units from left side
    //    543210  <- bits indices
    let shapes: Vec<Shape> = vec![
        Shape::new([0b000000, 0b000000, 0b000000, 0b111100]),
        Shape::new([0b000000, 0b001000, 0b011100, 0b001000]),
        Shape::new([0b000000, 0b000100, 0b000100, 0b011100]),
        Shape::new([0b000100, 0b000100, 0b000100, 0b000100]),
        Shape::new([0b000000, 0b000000, 0b001100, 0b001100]),
    ];

    let num_rocks = 1;
    let mut stack: Vec<u8> = Vec::new();

    for mut rock in shapes.iter().cycle().take(num_rocks).cloned() {
        let mut current_y = stack.len() as i32 + 3;

        // first apply jet then move down until the shape cannot move anymore.
        for dir in itertools::Itertools::intersperse(jets.iter(), &DOWN).cycle() {
            if !move_rock(&mut stack, current_y, dir, &mut rock) {
                merge_stack(&mut stack, current_y, &rock);
                break;
            } else {
                if *dir == Dir::Down {
                    current_y += -1;
                }
            }
        }

        print(&stack, current_y, &rock);
    }

    0
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| c.into()).collect()
}

fn main() {
    let jets = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(jets));
    //    println!("Part 2: {}", part2(pipes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_shape_creation() {
        let bar = Shape::new([0b000000, 0b000000, 0b000000, 0b111100]);
        assert_eq!(4, bar.width);
        assert_eq!(1, bar.height);
    }

    #[test]
    fn check_part1() {
        assert_eq!(3068, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        // assert_eq!(1707, part2(parse(INPUT)));
    }
}
