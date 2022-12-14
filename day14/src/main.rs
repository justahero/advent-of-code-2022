//! Day 14: Regolith Reservoir

use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::tuple, IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Rock = 0,
    Air,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Cell::Rock => "#",
            Cell::Air => ".",
            Cell::Sand => "o",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
struct Rect {
    min: Pos,
    max: Pos,
}

impl Rect {
    pub fn new(minx: i32, miny: i32, maxx: i32, maxy: i32) -> Self {
        Self {
            min: Pos::new(minx, miny),
            max: Pos::new(maxx, maxy),
        }
    }

    pub fn width(&self) -> usize {
        (self.max.x - self.min.x) as usize
    }

    pub fn height(&self) -> usize {
        (self.max.y - self.min.y) as usize
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns a directional Vector to advance one knot to the other
    pub fn get_dir(&self, target: &Pos) -> Pos {
        let x = match target.x.cmp(&self.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        let y = match target.y.cmp(&self.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        Pos::new(x, y)
    }

    /// This assumes one axis is the same, either x or y
    fn distance(&self, rhs: &Pos) -> i32 {
        if self.x == rhs.x {
            (rhs.y - self.y).abs()
        } else {
            (rhs.x - self.x).abs()
        }
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct PolyLine {
    points: Vec<Pos>,
}

impl PolyLine {
    pub fn new(points: Vec<Pos>) -> Self {
        Self { points }
    }
}

fn parse_pair(input: &str) -> IResult<&str, Pos> {
    let (input, (x, _, y)) = tuple((
        map_res(digit1, str::parse::<i32>),
        tag(","),
        map_res(digit1, str::parse::<i32>),
    ))(input)?;
    Ok((input, Pos::new(x, y)))
}

fn parse_lines(input: &str) -> PolyLine {
    let (_, points) = separated_list1(tag(" -> "), parse_pair)(input).unwrap();
    PolyLine::new(points)
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    bounds: Rect,
}

impl Grid {
    pub fn new(cells: Vec<Cell>, bounds: Rect) -> Self {
        Self { cells, bounds }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.bounds.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.bounds.height()
    }

    pub fn build(lines: Vec<PolyLine>) -> Self {
        let mut min = Pos::new(i32::MAX, i32::MAX);
        let mut max = Pos::new(i32::MIN, i32::MIN);

        // get dimensions
        for line in lines.iter() {
            for pos in &line.points {
                min.x = min.x.min(pos.x);
                max.x = max.x.max(pos.x);
                max.y = max.y.max(pos.y);
            }
        }
        let bounds = Rect::new(min.x, 0, max.x + 1, max.y + 1);
        let width = bounds.width();
        let height = bounds.height();

        let mut cells = vec![Cell::Air; width * height];

        // mark the grid with blocks
        for line in lines.iter() {
            for line in line.points.windows(2) {
                if let [l, r] = line {
                    let dir = r.get_dir(l);
                    let height = l.distance(r);
                    let mut pos = *l;
                    for _ in 0..=height {
                        let x = (pos.x - min.x) as usize;
                        let y = pos.y as usize;
                        cells[y * width + x] = Cell::Rock;
                        pos += dir;
                    }
                }
            }
        }

        Self::new(cells, bounds)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let index = y * self.width() + x;
                write!(f, "{}", self.cells[index])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn part1(grid: &Grid) -> u32 {
    0
}

fn parse(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(parse_lines)
        .collect_vec();

    let grid = Grid::build(lines);

    println!("GRID:\n{}", grid);

    grid
}

fn main() {
    let pairs = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "#;

    #[test]
    fn check_parse_lines() {
        assert_eq!(Pos::new(498, 4), parse_pair("498,4").unwrap().1);
        assert_eq!(
            PolyLine {
                points: vec![Pos::new(498, 4), Pos::new(498, 6), Pos::new(496, 6)]
            },
            parse_lines("498,4 -> 498,6 -> 496,6")
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(24, part1(&parse(INPUT)));
    }
}
