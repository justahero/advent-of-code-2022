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

impl Cell {
    pub fn is_blocked(&self) -> bool {
        matches!(self, Cell::Rock | Cell::Sand)
    }
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

#[derive(Debug, Clone)]
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

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
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

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Cell>,
    bounds: Rect,
}

impl Grid {
    pub fn new(cells: Vec<Cell>, bounds: Rect) -> Self {
        Self { cells, bounds }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.bounds.width()
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.bounds.height()
    }

    pub fn fill_sand(&mut self) -> usize {
        loop {
            let sand = Pos::new(500, 0);
            if !self.scan(sand) {
                break;
            }
        }

        // count all the sands
        self.cells
            .iter()
            .filter(|&&cell| cell == Cell::Sand)
            .count()
    }

    fn scan(&mut self, sand: Pos) -> bool {
        let directions = [Pos::new(0, 1), Pos::new(-1, 1), Pos::new(1, 1)];
        let mut sand = Pos::new(sand.x - self.bounds.min.x, sand.y);

        loop {
            // advance sand one step in any direction
            let next_pos = directions.iter().map(|dir| sand + *dir).find(|&next_pos| {
                match self.get(next_pos) {
                    Some(cell) => !cell.is_blocked(),
                    _ => false,
                }
            });

            match next_pos {
                Some(pos) => {
                    assert!(self.get(pos).unwrap() == Cell::Air);
                    sand = pos;
                }
                None => {
                    // There was no free spot, if it's inside boundaries set sand
                    if 0 <= sand.x
                        && sand.x <= self.width()
                        && 0 <= sand.y
                        && sand.y < self.height() - 1
                    {
                        self.set_cell(sand, Cell::Sand);
                        return true;
                    }
                    break;
                }
            }
        }

        false
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
        let bounds = Rect::new(min.x - 1, 0, max.x + 1, max.y + 1);
        let width = bounds.width();
        let height = bounds.height();

        let mut cells = vec![Cell::Air; (width * height) as usize];

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
                        cells[y * width as usize + x] = Cell::Rock;
                        pos += dir;
                    }
                }
            }
        }

        Self::new(cells, bounds)
    }

    /// Returns the cell at given coordinates.
    /// The x-coordinate has to be adjusted by the bounds to fit in.
    fn get(&self, Pos { x, y }: Pos) -> Option<Cell> {
        if 0 <= x && x < self.width() as i32 && 0 <= y && y < self.height() as i32 {
            Some(self.cells[(y * self.width() as i32 + x) as usize])
        } else {
            None
        }
    }

    fn set_cell(&mut self, Pos { x, y }: Pos, cell: Cell) {
        assert!(0 <= x && x < self.width() as i32);
        assert!(0 <= y && y < self.height() as i32);

        let index = (y * self.width() + x) as usize;
        self.cells[index] = cell;
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let index = y * self.width() + x;
                write!(f, "{}", self.cells[index as usize])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn part1(mut grid: Grid) -> usize {
    grid.fill_sand()
}

fn parse(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(parse_lines)
        .collect_vec();

    Grid::build(lines)
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(grid.clone()));
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
        assert_eq!(24, part1(parse(INPUT)));
    }
}
