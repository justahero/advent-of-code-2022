//! Day 14: Regolith Reservoir

struct Rect {
    min: Pos,
    max: Pos,
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Pos,
    end: Pos,
}

fn parse_lines(input: &str) -> Vec<Line> {


    Vec::new()
}

#[derive(Debug)]
struct Grid {
    lines: Vec<Line>,
}

impl Grid {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }
}

fn part1(grid: &Grid) -> u32 {
    0
}

fn parse(input: &str) -> Grid {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| parse_lines(input))
        .collect::<Vec<Vec<Line>>>();

    Grid::new()
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
    fn check_part1() {
        assert_eq!(24, part1(&parse(INPUT)));
    }
}
