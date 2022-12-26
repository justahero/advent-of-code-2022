//! Day 24: Unstable Diffusion

use std::collections::HashSet;

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
        }
    }

    /// Returns all unique blizzard formations possible.
    pub fn get_blizzard_formations(&self, blizzards: Vec<Blizzard>) -> Vec<Vec<Blizzard>> {
        let lcm = num::integer::lcm(self.width - 2, self.height - 2);

        let mut current = blizzards.clone();
        let mut blizzards = Vec::new();

        for _ in 0..lcm {
            blizzards.push(current.clone());
            let next_blizzards = self.advance(&current);
            current = next_blizzards;
        }

        blizzards
    }

    /// Search the shortest path
    pub fn shortest(&self, blizzards: Vec<Blizzard>) -> Option<u32> {
        let directions = [
            Pos::new(0, 0),
            Pos::new(0, -1),
            Pos::new(1, 0),
            Pos::new(0, 1),
            Pos::new(-1, 0),
        ];

        let blizzards = self.get_blizzard_formations(blizzards);
        let blizzards_len = blizzards.len();

        let mut time = 0usize;
        let mut current_positions = HashSet::from([self.start()]);

        loop {
            let mut next_positions: HashSet<Pos> = HashSet::new();
            let next_blizzrds = &blizzards[(time + 1).rem_euclid(blizzards_len) as usize];

            for pos in current_positions.into_iter() {
                for dir in directions.iter() {
                    let next_pos = pos + *dir;
                    if self.get_tile(&next_pos) != Tile::Ground {
                        continue;
                    }

                    if next_pos == self.end() {
                        return Some(time as u32 + 1);
                    }

                    if next_blizzrds.iter().all(|b| b.pos != next_pos) {
                        next_positions.insert(next_pos);
                    }
                }
            }

            current_positions = next_positions;
            if current_positions.is_empty() {
                current_positions.insert(self.start());
            }

            time += 1;
        }
    }

    pub fn add_row(mut self, mut tiles: Vec<Tile>) -> Self {
        self.height += 1;
        self.width = tiles.len() as i32;
        self.tiles.append(&mut tiles);
        self
    }

    /// Advances all blizzards
    pub fn advance(&self, blizzards: &[Blizzard]) -> Vec<Blizzard> {
        let mut result = Vec::new();

        for Blizzard { pos, dir } in blizzards {
            let next_pos = *pos + Self::DIRECTIONS[*dir as usize];
            let x = (next_pos.x - 1).rem_euclid(self.width - 2) + 1;
            let y = (next_pos.y - 1).rem_euclid(self.height - 2) + 1;
            result.push(Blizzard::new(Pos::new(x, y), *dir));
        }

        result
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
        Pos::new(self.width - 2, self.height - 1)
    }
}

/// Find the shortest path in the maze
fn part1(maze: &Maze, blizzards: Vec<Blizzard>) -> u32 {
    maze.shortest(blizzards).expect("Failed to find path")
}

fn part2(maze: &Maze, blizzards: Vec<Blizzard>) -> u32 {
    maze.shortest(blizzards).expect("Failed to get path")
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> (Maze, Vec<Blizzard>) {
    let mut all_blizzards = Vec::new();

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
            all_blizzards.append(&mut blizzards);
            maze.add_row(tiles)
        });

    (maze, all_blizzards)
}

fn main() {
    let (maze, blizzards) = parse(include_str!("input.txt"));
    let result = part1(&maze, blizzards.clone());
    assert!(146 < result);
    println!("Part 1: {}", result);
    println!("Part 2: {}", part2(&maze, blizzards));
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
    fn check_advance_blizzards() {
        let input: &str = r#"
            #.#####
            #.....#
            #.>...#
            #.....#
            #.....#
            #...v.#
            #####.#
        "#;

        let (maze, blizzards) = parse(input);
        let blizzards = maze.advance(&blizzards);

        let expected = vec![
            Blizzard::new(Pos::new(3, 2), Direction::East),
            Blizzard::new(Pos::new(4, 1), Direction::South),
        ];

        assert_eq!(expected, blizzards,);
    }

    #[test]
    fn check_part1() {
        let (maze, blizzards) = parse(INPUT);
        assert_eq!(18, part1(&maze, blizzards));
    }

    #[test]
    fn check_part2() {
        let (maze, blizzards) = parse(INPUT);
        assert_eq!(54, part2(&maze, blizzards));
    }
}
