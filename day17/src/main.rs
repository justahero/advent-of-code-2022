//! Day 17: Pyroclastic Flow

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub const fn new(x: i64, y: i64) -> Self {
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
    positions: Vec<Pos>,
}

impl Shape {
    pub fn new(positions: Vec<Pos>) -> Self {
        Self { positions }
    }

    pub fn move_all(&mut self, dir: Pos) {
        self.positions.iter_mut().for_each(|pos| *pos += dir);
    }

    pub fn pos(&self, x: i64, y: i64) -> Option<&Pos> {
        self.positions.iter().find(|p| p.x == x && p.y == y)
    }
}

#[derive(Debug)]
struct Stack {
    lines: Vec<u8>,
}

impl Stack {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn height(&self) -> i64 {
        self.lines.len() as i64
    }

    pub fn print(&self, shape: &Shape) {
        let length = self.height() as i64 + 6;

        // draw top to bottom
        for y in (0..length).rev() {
            let row = self.lines.get(y as usize);
            for x in 0..7 {
                if shape.pos(x, y).is_some() {
                    print!("@")
                } else if let Some(row) = row {
                    if row & (1 << x) > 0 {
                        print!("#");
                    } else {
                        print!(".")
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("-------\n");
    }

    #[inline]
    pub fn set(&mut self, pos: &Pos) {
        assert!(self.is_free(pos.x, pos.y)); // remove
        self.lines[pos.y as usize] |= 1 << pos.x;
    }

    #[inline]
    pub fn is_free(&self, x: i64, y: i64) -> bool {
        if 0 <= x && x < 7 && y >= 0 {
            if y < self.height() {
                return (self.lines[y as usize] & (1 << x)) == 0;
            } else {
                return true;
            }
        }
        false
    }
}

/// Returns true if rock was moved
fn can_move_rock(stack: &Stack, dir: &Dir, shape: &mut Shape) -> bool {
    // Calculate the next position of the shape.
    let next_positions = shape
        .positions
        .iter()
        .map(|pos| *pos + dir.dir())
        .collect_vec();

    // for each block of the shape check it's actually possible to move there
    next_positions.iter().all(|pos| stack.is_free(pos.x, pos.y))
}

fn merge_stack(stack: &mut Stack, shape: &Shape) {
    for pos in shape.positions.iter() {
        while pos.y >= stack.height() {
            stack.lines.push(0b0000000);
        }
        stack.set(pos);
    }
}

fn part1(jets: Vec<Dir>) -> usize {
    const DOWN: Dir = Dir::Down;

    let shapes: Vec<Shape> = vec![
        Shape::new(vec![
            Pos::new(2, 0),
            Pos::new(3, 0),
            Pos::new(4, 0),
            Pos::new(5, 0),
        ]),
        Shape::new(vec![
            Pos::new(3, 0),
            Pos::new(2, 1),
            Pos::new(3, 1),
            Pos::new(4, 1),
            Pos::new(3, 2),
        ]),
        Shape::new(vec![
            Pos::new(4, 2),
            Pos::new(4, 1),
            Pos::new(2, 0),
            Pos::new(3, 0),
            Pos::new(4, 0),
        ]),
        Shape::new(vec![
            Pos::new(2, 0),
            Pos::new(2, 1),
            Pos::new(2, 2),
            Pos::new(2, 3),
        ]),
        Shape::new(vec![
            Pos::new(2, 0),
            Pos::new(3, 0),
            Pos::new(2, 1),
            Pos::new(3, 1),
        ]),
    ];

    let num_rocks = 2022;
    let mut stack = Stack::new();

    let mut jet_iter = itertools::Itertools::intersperse(jets.iter().cycle(), &DOWN);

    for mut rock in shapes.iter().cycle().take(num_rocks).cloned() {
        rock.move_all(Pos::new(0, stack.height() + 3));

        // first apply jet then move down until the shape cannot move anymore, then repeat for next rock
        for dir in jet_iter.by_ref() {
            if !can_move_rock(&mut stack, dir, &mut rock) {
                if *dir == Dir::Down {
                    merge_stack(&mut stack, &rock);
                    break;
                }
            } else {
                rock.move_all(dir.dir());
            }
        }
    }

    stack.height() as usize
}

fn part2(jets: Vec<Dir>) -> u64 {
    0
}

fn parse(input: &str) -> Vec<Dir> {
    input.chars().map(|c| c.into()).collect()
}

fn main() {
    let jets = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(jets.clone()));
    println!("Part 2: {}", part2(jets));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_stack_is_free_bounds() {
        let stack = Stack::new();
        assert!(stack.is_free(0, 0));
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
