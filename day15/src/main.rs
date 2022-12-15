//! Day 15: Beacon Exclusion Zone

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

#[derive(Debug)]
struct Signal {
    pub sensor: Pos,
    pub beacon: Pos,
}

impl Signal {
    pub fn new(sensor: Pos, beacon: Pos) -> Self {
        Self { sensor, beacon }
    }
}

fn part1(signals: Vec<Signal>) -> u32 {
    0
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
    fn check_part1() {
        assert_eq!(26, part1(parse(INPUT)));
    }
}
