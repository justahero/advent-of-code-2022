//! Day 22: Monkey Map

use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveInput {
    Left,
    Right,
    Steps(u32),
}

peg::parser! {
    /// Parses line of movement Instructions
    grammar line_parser() for str {
        rule steps() -> MoveInput
            = n:$(['0'..='9']+) { MoveInput::Steps(n.parse::<u32>().unwrap()) }

        rule right() -> MoveInput
            = "R" { MoveInput::Right }

        rule left() -> MoveInput
            = "L" { MoveInput::Left }

        pub(crate) rule line() -> Vec<MoveInput>
            = items:((left() / right() / steps()) ** "") { items }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Tile {
    Empty,
    Floor,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Empty => " ",
            Tile::Floor => ".",
            Tile::Wall => "#",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Dir {
    Right = 0,
    Down,
    Left,
    Up,
}

impl From<u8> for Dir {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("Unsupported direction"),
        }
    }
}

impl Dir {
    pub fn left(&self) -> Self {
        ((*self as u8 + 3) % 4).into()
    }

    pub fn right(&self) -> Self {
        ((*self as u8 + 1) % 4).into()
    }

    pub fn direction(&self) -> Pos {
        match self {
            Dir::Right => Pos::new(1, 0),
            Dir::Down => Pos::new(0, 1),
            Dir::Left => Pos::new(-1, 0),
            Dir::Up => Pos::new(0, -1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// A trait to wrap moves in different ways
pub trait BoardWrapper {
    fn advance(&self, board: &Board, pos: Pos, dir: Dir) -> (Pos, Dir);
}

/// Wraps around on a 2-dimensional plane.
struct Planar {}

impl BoardWrapper for Planar {
    fn advance(&self, board: &Board, pos: Pos, dir: Dir) -> (Pos, Dir) {
        let pos = pos + dir.direction();
        (
            Pos::new(
                pos.x.rem_euclid(board.width as i32),
                pos.y.rem_euclid(board.height as i32),
            ),
            dir,
        )
    }
}

/// Wraps around on a cuboid
struct Cuboid {}

/// This cuboid wrapper is very specific to the given input file and may not work
/// for other cube side layouts in the input file.
///
/// The layout in the input file is:
///
/// ```text
///        ┌─────┬─────┐   : Consider the following sides of the cube
///        │     │     │   :             (x, y)
///        │  1  │  2  │   : 1 - back    (1, 0)
///        │     │     │   : 2 - right   (2, 0)
///        ├─────┼─────┘   : 3 - top     (1, 1)
///        │     │         : 4 - left    (0, 2)
///        │  3  │         : 5 - front   (1, 2)
///        │     │         : 6 - bottom  (0, 3)
///  ┌─────┼─────┤
///  │     │     │
///  │  4  │  5  │
///  │     │     │
///  ├─────┼─────┘
///  │     │
///  │  6  │
///  │     │
///  └─────┘
/// ```
///
fn wrap_cube_side(pos: Pos, side: i32, dir: Dir) -> (i32, i32, Dir) {
    let (cube_col, cube_row, new_dir) = match (pos.x / side, pos.y / side, dir) {
        (1, 0, Dir::Up) => (0, 3, Dir::Right),
        (1, 0, Dir::Right) => (2, 0, Dir::Right),
        (1, 0, Dir::Down) => (1, 1, Dir::Down),
        (1, 0, Dir::Left) => (0, 2, Dir::Right),
        (2, 0, Dir::Up) => (0, 3, Dir::Up),
        (2, 0, Dir::Right) => (1, 2, Dir::Left),
        (2, 0, Dir::Down) => (1, 1, Dir::Left),
        (2, 0, Dir::Left) => (1, 0, Dir::Left),
        (1, 1, Dir::Up) => (1, 0, Dir::Up),
        (1, 1, Dir::Right) => (2, 0, Dir::Up),
        (1, 1, Dir::Down) => (1, 2, Dir::Down),
        (1, 1, Dir::Left) => (0, 2, Dir::Down),
        (0, 2, Dir::Up) => (1, 1, Dir::Right),
        (0, 2, Dir::Right) => (1, 2, Dir::Right),
        (0, 2, Dir::Down) => (0, 3, Dir::Down),
        (0, 2, Dir::Left) => (1, 0, Dir::Right),
        (1, 2, Dir::Up) => (1, 1, Dir::Up),
        (1, 2, Dir::Right) => (2, 0, Dir::Left),
        (1, 2, Dir::Down) => (0, 3, Dir::Left),
        (1, 2, Dir::Left) => (0, 2, Dir::Left),
        (0, 3, Dir::Up) => (0, 2, Dir::Up),
        (0, 3, Dir::Right) => (1, 2, Dir::Up),
        (0, 3, Dir::Down) => (2, 0, Dir::Down),
        (0, 3, Dir::Left) => (1, 0, Dir::Down),
        _ => unreachable!("Unsupported movement"),
    };
    (cube_col, cube_row, new_dir)
}

/// Compare with: https://github.com/wilkotom/AoC2022/blob/main/day22/src/main.rs
/// & https://nickymeuleman.netlify.app/garden/aoc2022-day22
impl BoardWrapper for Cuboid {
    fn advance(&self, board: &Board, current_pos: Pos, dir: Dir) -> (Pos, Dir) {
        // println!("----->> Advance - pos: {:?}, dir: {:?}", current_pos, dir);

        let side = board.side;

        // Get the current cube side ranges to check of current pos
        let (current_side_x, current_side_y) = (current_pos.x / side, current_pos.y / side);
        let current_cube_x = current_side_x * side..current_side_x * side + side;
        let current_cube_y = current_side_y * side..current_side_y * side + side;

        // Calculate which cube side (& dir) to transition to in case.
        // Advance one step, then wrap around if the new position is not within the same cube side
        let next_pos = current_pos + dir.direction();

        if current_cube_x.contains(&next_pos.x) && current_cube_y.contains(&next_pos.y) {
            // println!("---->> Within same cube ({:?}, {:?}): {:?}", current_cube_x, current_cube_y, next_pos);
            (next_pos, dir)
        } else {
            println!(
                "---->> Wrap, current: {:?} ({:?}) at ({},{}), next: {:?}",
                current_pos, dir, current_side_x, current_side_y, next_pos
            );

            // Determine next cube and the offset within the cube
            let (cube_col, cube_row, new_dir) = wrap_cube_side(current_pos, side, dir);
            let (cube_x, cube_y) = (next_pos.x % side, next_pos.y % side);
            println!(
                "       New cube: {},{}, cube offset: {},{}, dir: {:?}, new_dir: {:?}",
                cube_col, cube_row, cube_x, cube_y, dir, new_dir
            );

            let i = match dir {
                Dir::Left => 49 - cube_y,
                Dir::Right => cube_y,
                Dir::Up => cube_x,
                Dir::Down => 49 - cube_x,
            };

            // find new idxes within the cube
            let new_row = match new_dir {
                Dir::Left => 49 - i,
                Dir::Right => i,
                Dir::Up => 49,
                Dir::Down => 0,
            };
            let new_col = match new_dir {
                Dir::Left => 49,
                Dir::Right => 0,
                Dir::Up => i,
                Dir::Down => 49 - i,
            };

            let new_pos = Pos::new(cube_col * side + new_col, cube_row * side + new_row);
            println!(
                "       New pos: {:?}, new_col: {}, new_row: {}",
                new_pos, new_col, new_row
            );
            (new_pos, new_dir)
        }
    }
}

/// Two dimensional board with wall / floor tiles
pub struct Board {
    height: i32,
    width: i32,
    side: i32,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(width: i32, side: i32) -> Self {
        Board {
            width,
            height: 0,
            side,
            tiles: Vec::new(),
        }
    }

    /// Apply the given moves on the board, starting from most top left floor tile
    ///
    /// Returns the final position.
    fn apply(&self, moves: &[MoveInput], wrapper: &impl BoardWrapper) -> (Pos, Dir) {
        moves
            .iter()
            .fold((self.start_pos(), Dir::Right), |(pos, dir), m| {
                self.next_move(pos, dir, &m, wrapper)
            })
    }

    /// Advance next move, either turn left or right or move forward a number of steps until
    /// a free spot is reached or a wall is hit.
    fn next_move(
        &self,
        start: Pos,
        dir: Dir,
        move_input: &MoveInput,
        wrapper: &impl BoardWrapper,
    ) -> (Pos, Dir) {
        println!(
            "<<< Next Move: {:?} ({:?}), Move: {:?}",
            start, dir, move_input,
        );

        let mut pos = start;
        let mut dir = dir;

        match move_input {
            MoveInput::Left => dir = dir.left(),
            MoveInput::Right => dir = dir.right(),
            MoveInput::Steps(num_steps) => {
                let mut next_pos = start;
                let mut steps = 0;

                loop {
                    // advance to next pos, wrap around if necessary
                    (next_pos, dir) = wrapper.advance(self, next_pos, dir);
                    let tile = self.get_tile(next_pos);

                    // get to the next tile
                    match tile {
                        Tile::Empty => {
                            continue;
                        }
                        Tile::Floor => {
                            pos = next_pos;
                            steps += 1;
                            if steps == *num_steps {
                                break;
                            }
                        }
                        Tile::Wall => {
                            break;
                        }
                    }
                }
            }
        }

        (pos, dir)
    }

    /// Adds a new line of tiles to the board
    pub fn add_line(mut self, tiles: Vec<Tile>) -> Self {
        let mut new_tiles = vec![Tile::Empty; self.width as usize];
        new_tiles[0..tiles.len()].clone_from_slice(&tiles);
        self.tiles.append(&mut new_tiles);
        self.height += 1;
        self
    }

    /// Returns the start pos, most top left floor tile.
    fn start_pos(&self) -> Pos {
        let x = self.tiles.iter().position(|t| *t == Tile::Floor).unwrap() as i32;
        Pos::new(x, 0)
    }

    /// Returns the tile with the given position.
    fn get_tile(&self, pos: Pos) -> Tile {
        if 0 <= pos.x && pos.x < self.width as i32 && 0 <= pos.y && pos.y < self.height as i32 {
            return self.tiles[(pos.y * self.width as i32 + pos.x) as usize];
        }
        Tile::Empty
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                write!(f, "{}", self.tiles[index])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn part1(board: &Board, moves: &[MoveInput]) -> i64 {
    let (pos, dir) = board.apply(moves, &Planar {});
    ((pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir as i32) as i64
}

fn part2(board: &Board, moves: &[MoveInput]) -> i64 {
    println!("START: {:?}", board.start_pos());

    let (pos, dir) = board.apply(moves, &Cuboid {});
    ((pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir as i32) as i64
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str, side: i32) -> (Board, Vec<MoveInput>) {
    let (board, instructions) = input.split_once("\n\n").expect("Failed to split.");

    let max_width = board
        .lines()
        .map(|line| line.len())
        .max()
        .expect("Failed to get width");

    let board = board
        .lines()
        .fold(Board::new(max_width as i32, side), |board, line| {
            board.add_line(line.chars().map(Tile::from).collect_vec())
        });

    let moves = line_parser::line(instructions).into_iter().next().unwrap();

    (board, moves)
}

fn main() {
    let (board, moves) = parse(include_str!("input.txt"), 50);
    println!("Part 1: {}", part1(&board, &moves));

    let result = part2(&board, &moves);
    assert!(110400 == result);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_parse_moves() {
        let (_board, moves) = parse(INPUT, 4);
        assert_eq!(
            vec![
                MoveInput::Steps(10),
                MoveInput::Right,
                MoveInput::Steps(5),
                MoveInput::Left,
                MoveInput::Steps(5),
                MoveInput::Right,
                MoveInput::Steps(10),
                MoveInput::Left,
                MoveInput::Steps(4),
                MoveInput::Right,
                MoveInput::Steps(5),
                MoveInput::Left,
                MoveInput::Steps(5),
            ],
            moves,
        )
    }

    #[test]
    fn check_part1() {
        let (board, moves) = parse(INPUT, 4);
        assert_eq!(6032, part1(&board, &moves));
    }

    #[test]
    fn check_part2() {
        let (board, moves) = parse(INPUT, 4);
        assert_eq!(5031, part2(&board, &moves));
    }
}
