//! Day 24: Unstable Diffusion

peg::parser! {
    grammar line_parser() for str {
        pub(crate) rule line() -> (Vec<Tile>, Vec<Blizzard>)
            = { (Vec::new(), Vec::new()) }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
#[repr(u8)]
enum TileType {
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
        Pos::new(x, y)
    }
}

#[derive(Debug)]
struct Blizzard {
    pos: Pos,
    dir: Direction,
}

/// Describes a field with coordinates and tile types?
#[derive(Debug)]
struct Tile {
    pos: Pos,
    field: TileType,
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

    pub fn add_row(mut self, (mut tiles, mut blizzards): (Vec<Tile>, Vec<Blizzard>)) -> Self {
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
        .fold(Maze::new(), |maze, line| {
            maze.add_row(line_parser::line(line).unwrap())
        });

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
