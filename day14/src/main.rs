//! Day 14: Regolith Reservoir

use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::tuple, IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Rock = 0,
    Sand,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
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

fn parse_pos(input: &str) -> IResult<&str, Pos> {
    let (input, (x, _, y)) = tuple((
        map_res(digit1, str::parse::<i32>),
        tag(","),
        map_res(digit1, str::parse::<i32>),
    ))(input)?;
    Ok((input, Pos::new(x, y)))
}

fn parse_lines(input: &str) -> Vec<Pos> {
    let (_, points) = separated_list1(tag(" -> "), parse_pos)(input).unwrap();
    points
}

#[derive(Debug, Clone)]
struct Grid {
    cells: BTreeMap<Pos, Cell>,
    /// The max depth of the lowest rock
    depth: i32,
    /// The minimum x
    min_x: i32,
    /// The maximum x
    max_x: i32,
}

impl Grid {
    pub fn new(cells: BTreeMap<Pos, Cell>) -> Self {
        let mut depth = i32::MIN;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;

        // get dimensions
        for (pos, _) in cells.iter() {
            min_x = i32::min(min_x, pos.x);
            max_x = i32::max(max_x, pos.x);
            depth = i32::max(depth, pos.y);
        }

        Self {
            cells,
            depth,
            min_x,
            max_x,
        }
    }

    pub fn fill_sand(&mut self) -> usize {
        while self.scan(Pos::new(500, 0)) {}

        // count all the sands
        self.cells
            .iter()
            .filter(|(_pos, &cell)| cell == Cell::Sand)
            .count()
    }

    fn scan(&mut self, start: Pos) -> bool {
        // down, left-down, right-down
        let directions = [Pos::new(0, 1), Pos::new(-1, 1), Pos::new(1, 1)];
        let mut sand = start;

        loop {
            // advance one step in any direction
            let next_pos = directions
                .iter()
                .map(|dir| sand + *dir)
                .find(|&next_pos| self.get(&next_pos).is_none());

            match next_pos {
                Some(pos) => {
                    if pos.y >= self.depth {
                        break;
                    }
                    sand = pos.clone();
                }
                None => {
                    self.cells.insert(sand, Cell::Sand);
                    if sand == start {
                        return false;
                    }
                    return true;
                }
            }
        }

        false
    }

    pub fn build(lines: Vec<Vec<Pos>>) -> Self {
        let mut cells = BTreeMap::new();

        // mark the grid with blocks
        for line in lines.iter() {
            for line in line.windows(2) {
                if let [l, r] = line {
                    // vertical line
                    if l.x == r.x {
                        let x = l.x;
                        let y_start = i32::min(l.y, r.y);
                        let y_end = i32::max(l.y, r.y);
                        for y in y_start..=y_end {
                            cells.insert(Pos::new(x, y), Cell::Rock);
                        }
                    } else {
                        // horizontal line
                        let y = l.y;
                        let x_start = i32::min(l.x, r.x);
                        let x_end = i32::max(l.x, r.x);
                        for x in x_start..=x_end {
                            cells.insert(Pos::new(x, y), Cell::Rock);
                        }
                    }
                }
            }
        }

        Self::new(cells)
    }

    /// Returns the cell at given coordinates.
    /// The x-coordinate has to be adjusted by the bounds to fit in.
    fn get(&self, pos: &Pos) -> Option<&Cell> {
        self.cells.get(pos)
    }

    /// Sets an Air cell to Sand
    fn set_cell(&mut self, pos: Pos, cell: Cell) {
        self.cells.insert(pos, cell);
    }
}

fn part1(mut grid: Grid) -> usize {
    grid.fill_sand()
}

fn part2(mut grid: Grid) -> usize {
    // update bounds of grid
    grid.depth += 2;
    grid.min_x = 500 - grid.depth - 1;
    grid.max_x = 500 + grid.depth + 1;

    // add bottom rock row to grid
    let y = grid.depth;
    for x in grid.min_x..=grid.max_x {
        grid.set_cell(Pos::new(x, y), Cell::Rock);
    }

    grid.fill_sand()
}

fn parse(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(parse_lines)
        .collect::<Vec<_>>();

    Grid::build(lines)
}

fn main() {
    let grid = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid));
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
        assert_eq!(Pos::new(498, 4), parse_pos("498,4").unwrap().1);
        assert_eq!(
            vec![Pos::new(498, 4), Pos::new(498, 6), Pos::new(496, 6)],
            parse_lines("498,4 -> 498,6 -> 496,6")
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(24, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(93, part2(parse(INPUT)));
    }
}
