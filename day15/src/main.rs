//! Day 15: Beacon Exclusion Zone

use std::collections::HashSet;

use itertools::Itertools;

peg::parser! {
    grammar line_parser() for str {
        rule number() -> i64
            = n:$(['-' | '0'..='9']+) { n.parse::<i64>().unwrap() }

        rule pos() -> Pos
            = "x=" x:number() ", y=" y:number() { Pos::new(x, y) }

        pub(crate) rule signal() -> Signal
            = "Sensor at " sensor:pos() ": closest beacon is at " beacon:pos() { Signal::new(sensor, beacon) }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan(&self, rhs: &Pos) -> i64 {
        i64::abs(self.x - rhs.x) + i64::abs(self.y - rhs.y)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    pub start: i64,
    pub end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn overlap(&self, rhs: &Range) -> bool {
        self.end >= rhs.start && rhs.end >= self.start
    }

    #[inline]
    pub fn merge(&self, rhs: &Range) -> Range {
        let start = i64::min(self.start, rhs.start);
        let end = i64::max(self.end, rhs.end);
        Range::new(start, end)
    }
}

#[derive(Debug, Clone)]
struct Signal {
    pub sensor: Pos,
    pub beacon: Pos,
}

impl Signal {
    pub fn new(sensor: Pos, beacon: Pos) -> Self {
        Self { sensor, beacon }
    }

    pub fn manhattan(&self) -> i64 {
        self.sensor.manhattan(&self.beacon)
    }
}

fn part1(signals: Vec<Signal>, line_number: i64) -> usize {
    // For each signal check if it's signal reaches or crosses the 'y' row
    // in case it does, calculate the positions on 'y'
    let beacons = signals
        .iter()
        .map(|signal| signal.beacon)
        .collect::<HashSet<Pos>>();

    let x_positions = signals
        .iter()
        .map(|sensor| (sensor.manhattan(), sensor.sensor))
        .filter(|(distance, sensor)| {
            let sensor_range = (sensor.y - distance)..(sensor.y + distance);
            sensor_range.contains(&line_number)
        })
        .flat_map(|(max_distance, sensor)| {
            let distance = (sensor.y - line_number).abs();
            let max_distance = max_distance - distance;

            (sensor.x - max_distance)..=(sensor.x + max_distance)
        })
        .unique()
        .filter(|x| !beacons.contains(&Pos::new(*x, line_number)));

    x_positions.count()
}

fn find_missing_beacon(signals: Vec<Signal>, limit: i64) -> Option<Pos> {
    let signals = signals
        .iter()
        .map(|signal| (signal.sensor, signal.manhattan()))
        .collect::<Vec<_>>();

    // find all the ranges for this row, see if they are contiguous / overlap
    for line_number in 0..=limit {
        let mut ranges = signals
            .iter()
            .filter(|(sensor, distance)| (sensor.y - line_number).abs() <= *distance)
            .map(|(sensor, manhattan)| {
                let dx = (manhattan - (sensor.y - line_number).abs()).abs();

                let minx = i64::max(0, sensor.x - dx);
                let maxx = i64::min(limit, sensor.x + dx);

                Range::new(minx, maxx)
            })
            .collect::<Vec<_>>();

        ranges.sort();

        // check the ranges are contiguous
        let (_result, x) = ranges
            .iter()
            .fold((Range::new(0, 0), None), |mut acc, range| {
                if acc.0.overlap(range) {
                    acc.0 = acc.0.merge(range);
                } else {
                    acc.1 = Some(acc.0.end + 1);
                }
                acc
            });

        if let Some(x) = x {
            return Some(Pos::new(x, line_number));
        }
    }

    None
}

/// Find the single position that is not covered by signals, it's the missing beacon.
fn part2(signals: Vec<Signal>, limit: i64) -> usize {
    let pos = find_missing_beacon(signals, limit).expect("Failed to find missing beacon");
    (pos.x * 4_000_000 + pos.y) as usize
}

fn parse(input: &str) -> Vec<Signal> {
    input
        .lines()
        .filter_map(|line| line_parser::signal(line).ok())
        .collect::<Vec<_>>()
}

fn main() {
    let signals = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(signals.clone(), 2_000_000));
    let result = part2(signals, 4_000_000);
    assert!(result > 4079108237741);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_manhattan_distance() {
        assert_eq!(9, Pos::new(8, 7).manhattan(&Pos::new(2, 10)));
    }

    #[test]
    fn check_part1() {
        assert_eq!(26, part1(parse(INPUT), 10));
    }

    #[test]
    fn check_part2() {
        assert_eq!(56_000_011, part2(parse(INPUT), 20));
    }
}
