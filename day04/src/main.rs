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

    fn covers(&self) -> bool {
        self.l.contains(&self.r) || self.r.contains(&self.l)
    }

    fn intersects(&self) -> bool {
        self.l.intersect(&self.r)
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

    /// Returns `true` if the given [`Range`] is completely contained
    pub fn contains(&self, rhs: &Self) -> bool {
        self.min <= rhs.min && self.max >= rhs.max
    }

    /// Returns `true` if both ranges intersect with each other.
    ///
    /// Both sections intersect when any bound is inside the bounds of the other.
    pub fn intersect(&self, rhs: &Self) -> bool {
        self.max >= rhs.min && rhs.max >= self.min
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

/// Returns the total number of overlapping assignments, where one part overlaps completely
fn part1(sections: &[Assignment]) -> usize {
    sections
        .iter()
        .filter(|&assignment| assignment.covers())
        .count()
}

/// Returns the total number of any overlapping assignments
fn part2(sections: &[Assignment]) -> usize {
    sections
        .iter()
        .filter(|&assignment| assignment.intersects())
        .count()
}

fn parse(input: &str) -> Vec<Assignment> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(','))
        .map(|(l, r)| Assignment::new(l.into(), r.into()))
        .collect::<Vec<_>>()
}

fn main() {
    let input = parse(include_str!("input.txt"));
    println!("Overlaps: {}", part1(&input));
    println!("Intersections: {}", part2(&input));
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
    fn check_range_bounds() {
        let l = Range::new(2, 8);
        let r = Range::new(3, 7);
        assert!(l.contains(&r));
        assert!(!r.contains(&l));
    }

    #[test]
    fn check_range_intersects() {
        let l = Range::new(5, 7);
        let r = Range::new(7, 9);
        assert!(l.intersect(&r));
        assert!(r.intersect(&l));
    }

    #[test]
    fn check_part1() {
        assert_eq!(2, part1(&parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(4, part2(&parse(INPUT)));
    }
}
