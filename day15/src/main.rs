//! Day 15: Beacon Exclusion Zone

use std::{collections::HashSet, ops::RangeInclusive};

peg::parser! {
    grammar line_parser() for str {
        rule number() -> i32
            = n:$(['-' | '0'..='9']+) { n.parse::<i32>().unwrap() }

        rule pos() -> Pos
            = "x=" x:number() ", y=" y:number() { Pos::new(x, y) }

        pub(crate) rule signal() -> Signal
            = "Sensor at " sensor:pos() ": closest beacon is at " beacon:pos() { Signal::new(sensor, beacon) }
    }
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

    pub fn manhattan(&self, rhs: &Pos) -> i32 {
        i32::abs(self.x - rhs.x) + i32::abs(self.y - rhs.y)
    }
}

#[derive(Debug)]
struct Signal {
    pub sensor: Pos,
    pub beacon: Pos,
}

impl Signal {
    pub fn new(sensor: Pos, beacon: Pos) -> Self {
        Self { sensor, beacon }
    }

    pub fn manhattan(&self) -> i32 {
        self.sensor.manhattan(&self.beacon)
    }
}

fn part1(signals: Vec<Signal>) -> usize {
    let row = 10;

    // TODO for each signal check if it's signal reaches or crosses the 'y' row
    // in case it does, calculate the positions on 'y'
    let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();
    let mut beacons = HashSet::new();

    for signal in signals.iter() {
        let Pos { x, y } = &signal.sensor;

        // get signal distance to beacon
        let manhattan = signal.manhattan();

        // get sensor y
        let sensor_y = i32::abs(row - y);

        // if sensor is in range of row 'y'
        if manhattan >= sensor_y {
            let width = manhattan - sensor_y;
            let range = (x - width)..=(x + width);
            ranges.push(range);

            if signal.beacon.y == row {
                beacons.insert(signal.beacon.y);
            }
        }
    }

    // last count all numbers of row 'y'
    let mut positions: HashSet<i32> = HashSet::new();
    for range in ranges.into_iter() {
        for x in range {
            if beacons.get(&x).is_none() {
                positions.insert(x);
            }
        }
    }

    positions.len()
}

fn parse(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .map(|line| line_parser::signal(line).expect("Failed to parse line"))
        .collect::<Vec<_>>()
}

fn main() {
    let signals = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(signals));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#;

    #[test]
    fn check_manhattan_distance() {
        assert_eq!(9, Pos::new(8, 7).manhattan(&Pos::new(2, 10)));
    }

    #[test]
    fn check_part1() {
        assert_eq!(26, part1(parse(INPUT)));
    }
}
