//! Day 15: Beacon Exclusion Zone

use std::collections::HashSet;

use itertools::Itertools;

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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash)]
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
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    /// Checks if this line intersects the other, both need to align either horizontally or vertically
    /// Returns the new line.
    pub fn intersects(&self, rhs: &Range) -> Option<Range> {
        if rhs.end < self.start || rhs.start < self.end {
            let minx = i32::min(self.start, rhs.start);
            let maxx = i32::max(self.end, rhs.end);
            Some(Range::new(minx, maxx))
        } else {
            None
        }
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

fn part1(signals: Vec<Signal>, line_number: i32) -> usize {
    // For each signal check if it's signal reaches or crosses the 'y' row
    // in case it does, calculate the positions on 'y'
    // let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();

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

    /*
    for signal in signals.iter() {
        let Pos { x, y } = &signal.sensor;

        // get signal distance to beacon
        let manhattan = signal.manhattan();

        // get sensor y
        let sensor_y = i32::abs(row - y);

        // if sensor is in range of row 'y'
        if manhattan >= sensor_y {
            let width = manhattan - sensor_y;
            let range = Range::new(x - width, x + width);
            ranges.push(range);

            if signal.beacon.y == row {
                beacons.insert(signal.beacon.y);
            }
        }
    }
    */

    // last count all numbers of row 'y'
    /*
    for range in ranges.into_iter() {
        for x in range {
            if beacons.get(&x).is_none() {
                positions.insert(x);
            }
        }
    }
    */
}

fn part2(signals: Vec<Signal>) -> u64 {
    for signal in signals.iter() {}

    todo!("")
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
    let result = part1(signals, 2_000_000);
    assert!(result < 5686057);
    println!("Part 1: {}", result);
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
        assert_eq!(26, part1(parse(INPUT), 10));
    }

    #[test]
    fn check_part2() {
        assert_eq!(56_000_011, part2(parse(INPUT)));
    }
}
