//! Day 17: Pyroclastic Flow

use std::ops::Shl;

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

#[derive(Debug)]
enum Dir {
    Left = 0,
    Right,
}

impl Dir {
    pub fn dir(&self) -> Pos {
        match self {
            Dir::Left => Pos::new(-1, 0),
            Dir::Right => Pos::new(1, 0),
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

struct Shape {
    rows: Vec<u8>,
    height: i32,
    width: i32,
}

impl Shape {
    pub fn new(bits: [u8; 4]) -> Self {
        let width = bits.iter().map(|row| row.count_ones()).max().unwrap() as i32;
        let height = bits.iter().filter(|&row| row.count_ones() > 1).sum::<u8>() as i32;

        Self {
            rows: bits.to_vec(),
            height,
            width,
        }
    }

    /// Returns the indexes of bits where it's one as iterator
    pub fn ones(&self, row_index: i32) -> impl Iterator<Item = u8> + '_ {
        assert!(0 <= row_index && row_index < self.height);
        (0..self.width)
            .filter(move |index| (self.rows[row_index as usize] & 1u8.shl(index) > 0))
            .map(|index| index as u8)
    }
}

fn can_move(stack: &[u8], next_pos: Pos, shape: &Shape) -> bool {
    false
}

fn merge_stack(stack: &[u8], current_pos: Pos, shape: &Shape) {
    // todo!()
}

fn print(stack: &[u8], current_pos: &Pos, shape: &Shape) {
    let length = stack.len() as i32 + 4;

    // scan each row
    for y in (0..length).rev() {
        // draw each horizontal cell in row
        let diff = current_pos.y - y..current_pos.y - y + shape.height;

        for x in 0..7 {
            if diff.contains(&y)
                && shape
                    .ones(current_pos.y - y)
                    .find(|index| current_pos.x + (*index as i32) == x)
                    .is_some()
            {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("-------");
}

fn part1(directions: Vec<Dir>) -> usize {
    const DOWN: Pos = Pos::new(0, -1);
    let jets = directions.iter().map(|dir| dir.dir()).collect_vec();

    // X bits are given in reverse order
    //
    // `0b111100` -> represents '@@@@' 2 units from left side
    //    543210  <- bits indices
    let shapes: Vec<Shape> = vec![
        Shape::new([0b0000, 0b0000, 0b0000, 0b1111]),
        Shape::new([0b0000, 0b0010, 0b0111, 0b0010]),
        Shape::new([0b0000, 0b0001, 0b0001, 0b0111]),
        Shape::new([0b0001, 0b0001, 0b0001, 0b0001]),
        Shape::new([0b0000, 0b0000, 0b0011, 0b0011]),
    ];

    let num_rocks = 1;
    let mut stack: Vec<u8> = Vec::new();

    for rock in shapes.iter().cycle().take(num_rocks) {
        let mut current_pos = Pos::new(2, stack.len() as i32 + 3);

        // first apply jet then move down until the shape cannot move anymore.
        for dir in itertools::Itertools::intersperse(jets.iter(), &DOWN) {
            if can_move(&mut stack, current_pos + *dir, &rock) {
                current_pos += *dir;
            } else {
                merge_stack(&mut stack, current_pos, &rock);
            }
        }

        print(&stack, &current_pos, &rock);
    }

    0
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| c.into()).collect()
}

fn main() {
    //    let pipes = parse(include_str!("input.txt"));
    //    println!("Part 1: {}", part1(pipes.clone()));
    //    println!("Part 2: {}", part2(pipes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_part1() {
        assert_eq!(3068, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        // assert_eq!(1707, part2(parse(INPUT)));
    }
}
