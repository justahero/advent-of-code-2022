//! Day 24: Unstable Diffusion

use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
enum Direction {
    North = 0,
    East,
    South,
    West,
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "^" => Direction::North,
            ">" => Direction::East,
            "<" => Direction::West,
            "v" => Direction::South,
            _ => panic!("Unknown direction found"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Ground = 0,
    Wall,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Blizzard {
    pos: Pos,
    dir: Direction,
}

impl Blizzard {
    pub fn new(pos: Pos, dir: Direction) -> Self {
        Self { pos, dir }
    }
}

#[derive(Debug)]
struct Maze {
    /// The width of the maze
    width: i32,
    /// The height of the maze
    height: i32,
    /// All tiles of the maze, outline is walls
    tiles: Vec<Tile>,
    /// The list of current blizzards
    blizzards: Vec<Blizzard>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    blizzards: Vec<Blizzard>,
    time: u32,
}

impl State {
    pub fn new(pos: Pos, blizzards: Vec<Blizzard>, time: u32) -> Self {
        Self {
            pos,
            blizzards,
            time,
        }
    }
}

impl Maze {
    const DIRECTIONS: [Pos; 4] = [
        Pos::new(0, -1),
        Pos::new(1, 0),
        Pos::new(0, 1),
        Pos::new(-1, 0),
    ];

    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            blizzards: Vec::new(),
        }
    }

    /// Search the shortest path
    pub fn shortest(&self) -> u32 {
        let state = State {
            pos: self.start(),
            blizzards: self.blizzards.clone(),
            time: 0,
        };

        let mut stack = VecDeque::new();

        // keep all configurations of blizzards around
        let mut cache: HashMap<u32, Vec<Blizzard>> = HashMap::new();

        stack.push_back(state);

        while let Some(current) = stack.pop_front() {
            if current.pos == self.end() {
                return current.time;
            }

            // advance all blizzards by 1 minute, check if it was calculated before
            let next_blizzards = if let Some(blizzards) = cache.get(&(current.time + 1)) {
                blizzards.clone()
            } else {
                let blizzards = self.advance(&current.blizzards);
                cache.insert(current.time + 1, blizzards.clone());
                blizzards
            };

            // if possible wait on the current position
            if next_blizzards
                .iter()
                .find(|&blizzard| current.pos == blizzard.pos)
                .is_none()
            {
                stack.push_back(State::new(
                    current.pos,
                    next_blizzards.clone(),
                    current.time + 1,
                ));
            }

            // otherwise check all directions for possible moves
            for dir in Self::DIRECTIONS.iter() {
                let next_pos = current.pos + *dir;
                if self.get_tile(&next_pos) == Tile::Ground {
                    if next_blizzards
                        .iter()
                        .find(|&blizzard| current.pos + *dir == blizzard.pos)
                        .is_none()
                    {
                        stack.push_back(State::new(
                            current.pos + *dir,
                            next_blizzards.clone(),
                            current.time + 1,
                        ));
                    }
                }
            }
        }

        0
    }

    pub fn add_row(mut self, mut tiles: Vec<Tile>, mut blizzards: Vec<Blizzard>) -> Self {
        self.height += 1;
        self.width = tiles.len() as i32;
        self.tiles.append(&mut tiles);
        self.blizzards.append(&mut blizzards);
        self
    }

    /// Advances all blizzards
    fn advance(&self, blizzards: &[Blizzard]) -> Vec<Blizzard> {
        Vec::new()
    }

    fn get_tile(&self, pos: &Pos) -> Tile {
        if 0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height {
            return self.tiles[(pos.y * self.width + pos.x) as usize];
        }
        Tile::Wall
    }

    fn start(&self) -> Pos {
        Pos::new(1, 0)
    }

    fn end(&self) -> Pos {
        Pos::new(self.width - 1, self.height)
    }
}

fn part1(maze: Maze) -> i64 {
    0
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> Maze {
    let maze = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(Maze::new(), |maze, (row, line)| {
            let mut tiles = Vec::new();
            let mut blizzards = Vec::new();
            for (col, c) in line.chars().enumerate() {
                let pos = Pos::new(col as i32, row as i32);
                match c {
                    '#' => tiles.push(Tile::Wall),
                    _ => tiles.push(Tile::Ground),
                }
                match c {
                    '>' => blizzards.push(Blizzard::new(pos, Direction::East)),
                    '<' => blizzards.push(Blizzard::new(pos, Direction::West)),
                    '^' => blizzards.push(Blizzard::new(pos, Direction::North)),
                    'v' => blizzards.push(Blizzard::new(pos, Direction::South)),
                    _ => (),
                }
            }
            maze.add_row(tiles, blizzards)
        });

    println!("Hello");

    maze
}

fn main() {
    let maze = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    ";

    #[test]
    fn check_part1() {
        assert_eq!(18, part1(parse(INPUT)));
    }

    #[ignore]
    #[test]
    fn check_part2() {
        // assert_eq!(20, part2(parse(INPUT)));
    }
}
