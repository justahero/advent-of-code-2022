//! Day 24: Unstable Diffusion

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
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

#[derive(Debug)]
#[repr(u8)]
enum Tile {
    Ground = 0,
    Wall,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
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

impl Maze {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            tiles: Vec::new(),
            blizzards: Vec::new(),
        }
    }

    pub fn add_row(mut self, mut tiles: Vec<Tile>, mut blizzards: Vec<Blizzard>) -> Self {
        self.height += 1;
        self.width = tiles.len() as i32;
        self.tiles.append(&mut tiles);
        self.blizzards.append(&mut blizzards);
        self
    }

    pub fn get(x: i32, y: i32) -> Option<Tile> {
        None
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
    let grid = parse(include_str!("input.txt"));
    // println!("Part 1: {}", part1(grid.clone()));
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
