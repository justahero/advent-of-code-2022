//! Day 23: Unstable Diffusion

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter},
};

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum Dir {
    N = 0,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub const DIRECTIONS: [Pos; 8] = [
        Pos::new(0, -1),  // N
        Pos::new(1, -1),  // NE
        Pos::new(1, 0),   // E
        Pos::new(1, 1),   // SE
        Pos::new(0, 1),   // S
        Pos::new(-1, 1),  // SW
        Pos::new(-1, 0),  // W
        Pos::new(-1, -1), // NW
    ];

    /// Returns true if this direction contains no neighbors.
    pub fn propose(&self, neighbors: &[bool; 8]) -> bool {
        let [n, ne, e, se, s, sw, w, nw] = neighbors;

        match self {
            Dir::N => !n && !ne && !nw,
            Dir::S => !s && !se && !sw,
            Dir::W => !w && !nw && !sw,
            Dir::E => !e && !ne && !se,
            _ => false,
        }
    }

    pub fn direction(&self) -> Pos {
        Self::DIRECTIONS[*self as usize]
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    elves: HashSet<Pos>,
}

impl Grid {
    pub fn new(elves: HashSet<Pos>) -> Self {
        Self { elves }
    }

    /// Returns the bounds as two coordinate pairs:
    /// upper left & lower right.
    pub fn bounds(&self) -> (Pos, Pos) {
        let mut minx = i32::MAX;
        let mut miny = i32::MAX;
        let mut maxx = i32::MIN;
        let mut maxy = i32::MIN;

        for elf in self.elves.iter() {
            minx = min(elf.x, minx);
            miny = min(elf.y, miny);
            maxx = max(elf.x, maxx);
            maxy = max(elf.y, maxy);
        }

        (Pos::new(minx, miny), Pos::new(maxx, maxy))
    }

    pub fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        Dir::DIRECTIONS.iter().cloned().map(move |dir| pos + dir)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (Pos { x: minx, y: miny }, Pos { x: maxx, y: maxy }) = self.bounds();

        for y in miny..=maxy {
            for x in minx..=maxx {
                if self.elves.contains(&Pos::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Advances all elves across the grid once!
///
/// ```text
/// .....
/// ..##.
/// ..#..
/// .....
/// ..##.
/// .....
/// ```
fn advance(grid: &mut Grid, directions: &[Dir]) {
    // Maps proposed position to elves, multiple elves may propose the same position
    let mut proposals: HashMap<Pos, Vec<Pos>> = HashMap::new();

    // first phase each elf proposes a new position
    for elf in &grid.elves {
        let neighbors: [bool; 8] = grid
            .neighbors(*elf)
            .map(|neighbor| grid.elves.contains(&neighbor))
            .collect_vec()
            .try_into()
            .unwrap();

        // If there no elves in the adjacent fields don't move
        if neighbors.iter().all(|occupied| !occupied) {
            continue;
        }

        if let Some(proposed_dir) = directions.iter().find(|&dir| dir.propose(&neighbors)) {
            proposals
                .entry(*elf + proposed_dir.direction())
                .or_default()
                .push(*elf);
        }
    }

    // movement phase
    for (next_pos, elves) in proposals {
        if elves.len() == 1 {
            grid.elves.remove(&elves[0]);
            grid.elves.insert(next_pos);
        }
    }
}

fn part1(mut grid: Grid) -> i64 {
    let mut directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];

    for _ in 0..10 {
        advance(&mut grid, &directions);
        directions.rotate_left(1);
    }

    // determine bounds of current arrangement
    let (min, max) = grid.bounds();

    let width = (max.x - min.x) as i64 + 1;
    let height = (max.y - min.y) as i64 + 1;
    width * height - grid.elves.len() as i64
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> Grid {
    let elves = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '#' => Some(Pos::new(col as i32, row as i32)),
                    _ => None,
                })
        })
        .collect();

    Grid::new(elves)
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    ";

    #[test]
    fn check_advance() {
        let input: &str = r#"
            .....
            ..##.
            ..#..
            .....
            ..##.
            .....
        "#;

        let expected: &str = r#"
            ..##.
            .....
            ..#..
            ...#.
            ..#..
            .....
        "#;

        let mut grid = parse(&input);
        let directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];
        advance(&mut grid, &directions);
        println!("{}", grid);

        let expected_grid = parse(&expected);
        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn check_advance_big() {
        let input: &str = r#"
            ..............
            .......#......
            .....#...#....
            ...#..#.#.....
            .......#..#...
            ....#.#.##....
            ..#..#.#......
            ..#.#.#.##....
            ..............
            ....#..#......
            ..............
            ..............
        "#;

        let expected: &str = r#"
            ..............
            .......#......
            ....#.....#...
            ...#..#.#.....
            .......#...#..
            ...#..#.#.....
            .#...#.#.#....
            ..............
            ..#.#.#.##....
            ....#..#......
            ..............
            ..............
        "#;

        // simulate 2nd round
        let directions = vec![Dir::S, Dir::W, Dir::E, Dir::N];

        let mut grid = parse(&input);
        advance(&mut grid, &directions);

        let expected_grid = parse(&expected);
        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn check_part1() {
        assert_eq!(110, part1(parse(INPUT)));
    }
}
