//! Day 17: Pyroclastic Flow

use std::collections::HashMap;

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

const CAVE_WIDTH: usize = 7;

#[derive(Debug)]
struct Stack {
    /// Stack of all lines, each entry represents an empty field (`0`) or a rock (`#`).
    lines: Vec<[u8; CAVE_WIDTH]>,
}

impl Stack {
    pub fn new() -> Self {
        // 10_000 rows are enough to repeat the same patterns over & over.
        Self {
            lines: std::iter::repeat([0; CAVE_WIDTH])
                .take(10_000)
                .collect_vec(),
        }
    }

    pub fn height(&self) -> usize {
        self.lines
            .iter()
            .position(|row| row == &[0; CAVE_WIDTH])
            .unwrap()
    }

    pub fn column_heights(&self) -> [usize; CAVE_WIDTH] {
        let mut heights = [0; CAVE_WIDTH];
        let y = self.height();
        for x in 0..CAVE_WIDTH {
            heights[x] = (0..y)
                .find(|&y| self.lines[y][x] == 0)
                .unwrap_or(usize::MAX);
        }
        heights
    }

    #[inline]
    pub fn is_free(&self, x: i64, y: i64) -> bool {
        if 0 <= x && x < 7 && y >= 0 {
            if y < self.height() as i64 {
                return self.lines[y as usize][x as usize] == 0;
            } else {
                return true;
            }
        }
        false
    }
}

/// Returns true if rock was moved
fn can_move_rock(stack: &Stack, dir: &Dir, shape: &mut Shape) -> bool {
    shape
        .positions
        .iter()
        .map(|pos| *pos + dir.dir())
        .all(|Pos { x, y }| stack.is_free(x, y))
}

fn merge_stack(stack: &mut Stack, shape: &Shape) {
    for pos in shape.positions.iter() {
        stack.lines[pos.y as usize][pos.x as usize] = b'#';
    }
}

fn fill_rocks(mut stack: Stack, target_num_rocks: usize, jets: &[Dir]) -> usize {
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

    let mut total_height = 0;
    let mut rocks_count = 0;

    let mut cache = HashMap::new();
    let mut shapes_iter = shapes.into_iter().enumerate().cycle();
    let mut jet_iter = jets.iter().enumerate().cycle();

    while rocks_count < target_num_rocks {
        let (rock_index, mut rock) = shapes_iter.next().expect("Failed to get next shape");
        rock.move_all(Pos::new(0, stack.height() as i64 + 3));

        let jet_index = loop {
            // First apply jet force
            let (jet_index, dir) = jet_iter.next().expect("Failed to get dir");
            if can_move_rock(&mut stack, dir, &mut rock) {
                rock.move_all(dir.dir());
            }

            // Second move down
            let dir = &Dir::Down;
            if can_move_rock(&mut stack, dir, &mut rock) {
                rock.move_all(dir.dir());
            } else {
                merge_stack(&mut stack, &rock);
                break jet_index;
            }
        };

        // check if a cycle repeats
        let key = (rock_index, jet_index % jets.len(), stack.column_heights());
        if let Some((index, height)) = cache.get(&key) {
            let repeats = (target_num_rocks - index) / (rocks_count - index) - 1;
            rocks_count += (rocks_count - index) * repeats;
            total_height += (stack.height() - height) * repeats;
        } else {
            cache.insert(key, (rocks_count, stack.height()));
        }

        rocks_count += 1;
    }

    total_height + stack.height()
}

fn part1(jets: Vec<Dir>) -> usize {
    let num_rocks = 2022;
    let stack = Stack::new();
    fill_rocks(stack, num_rocks, &jets)
}

fn part2(jets: Vec<Dir>) -> usize {
    let num_rocks = 1_000_000_000_000;
    let stack = Stack::new();
    fill_rocks(stack, num_rocks, &jets)
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
        assert_eq!(1_514_285_714_288, part2(parse(INPUT)));
    }
}
