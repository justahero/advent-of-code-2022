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

#[derive(Debug, Clone)]
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

fn part2(signals: Vec<Signal>, limit: i32) -> u64 {
    let signals = signals
        .iter()
        .map(|signal| (signal.sensor, signal.manhattan()))
        .collect::<Vec<_>>();

    for line_number in 0..=limit {
        // find all the ranges for this row, see if they are contiguous / overlap
        let mut ranges = signals
            .iter()
            .map(|(sensor, distance)| {
                let dx = (distance - (sensor.y - line_number).abs()).abs();
                let minx = i32::max(0, sensor.x - dx);
                let maxx = i32::min(limit, sensor.x + dx);

                (minx, maxx)
            })
            .collect::<Vec<_>>();

        // check the ranges are contiguous
        ranges.sort();

        if let Some(x) =
            ranges
                .iter()
                .tuple_windows()
                .find_map(|((l_start, l_end), (r_start, r_end))| {
                    if !(l_end >= r_start && r_end >= l_start) {
                        Some(l_end + 1)
                    } else {
                        None
                    }
                })
        {
            println!("Pos: {},{}", x, line_number);
            return (x * 4_000_000 + line_number) as u64;
        };
    }

    panic!("Nothing found")
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
    println!("Part 1: {}", part1(signals.clone(), 2_000_000));
    println!("Part 2: {}", part2(signals, 4_000_000));
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
        assert_eq!(56_000_011, part2(parse(INPUT), 20));
    }
}
