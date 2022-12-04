//! Day 04: Camp Cleanup

struct Assignment {
    /// First or left sections
    pub l: Range,
    /// Second or right sections
    pub r: Range,
}

impl Assignment {
    pub fn new(l: Range, r: Range) -> Self {
        Self { l, r }
    }

    fn overlaps(&self) -> bool {
        self.l.contains(&self.r) || self.r.contains(&self.l)
    }
}

struct Range {
    /// The lower or minimum bound
    min: u32,
    /// The upper or maximum bound
    max: u32,
}

impl Range {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, rhs: &Self) -> bool {
        self.min <= rhs.min && self.max >= rhs.max
    }
}

impl From<&str> for Range {
    fn from(range: &str) -> Self {
        let (min, max) = range.split_once('-').expect("Failed to parse range.");
        let min = min.parse::<u32>().expect("Not a number");
        let max = max.parse::<u32>().expect("Not a number");
        Range::new(min, max)
    }
}

fn part1(sections: &[Assignment]) -> usize {
    sections
        .iter()
        .filter(|&assignment| assignment.overlaps())
        .count()
}

fn parse(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(','))
        .map(|(l, r)| (l.into(), r.into()))
        .map(|(l, r)| Assignment::new(l, r))
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input.txt"));
    println!("Overlaps: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#;

    #[test]
    fn check_flat_map() {
        let values = [true, true, false, false, false];
        let count = values.iter().filter_map(|v| v.into()).count();
        println!("Count: {}", count);
    }

    #[test]
    fn check_part1() {
        assert_eq!(2, part1(&parse(INPUT)));
    }
}
