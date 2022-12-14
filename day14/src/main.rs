//! Day 14: Regolith Reservoir

use itertools::{Itertools, MinMaxResult};
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    sequence::tuple, IResult,
};

#[derive(Debug)]
struct Rect {
    min: Pos,
    max: Pos,
}

#[derive(Debug, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
    lines: Vec<PolyLine>,
    bounds: Rect,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            bounds: Rect {
                min: Pos::new(i32::MAX, i32::MAX),
                max: Pos::new(i32::MIN, i32::MIN),
            },
        }
    }

    pub fn add_line(mut self, line: PolyLine) -> Self {
        if let MinMaxResult::MinMax(min, max) = line.points.iter().map(|p| p.x).minmax() {
            self.bounds.min.x = self.bounds.min.x.min(min);
            self.bounds.max.x = self.bounds.max.x.max(max);
        }
        if let MinMaxResult::MinMax(min, max) = line.points.iter().map(|p| p.y).minmax() {
            self.bounds.min.y = self.bounds.min.y.min(min);
            self.bounds.max.y = self.bounds.max.y.max(max);
        }
        self.lines.push(line);
        self
    }
}

fn part1(grid: &Grid) -> u32 {
    0
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_lines)
        .fold(Grid::new(), |grid, line| grid.add_line(line))
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
