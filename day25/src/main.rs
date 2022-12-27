//! Day 25: Full of Hot Air

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct SnafuNumber(pub i64);

/// The digits are 2, 1, 0, minus (written -), and double-minus (written =).
/// Minus is worth -1, and double-minus is worth -2.
impl From<&str> for SnafuNumber {
    fn from(value: &str) -> Self {
        let chars = value.chars().collect_vec();
        let length = chars.len();

        let mut number: i64 = 0;
        for index in (0..length).rev() {
            let c = chars[length - 1 - index];
            let digit: i64 = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unsupported char found"),
            };
            number += (5 as i64).pow(index as u32) * digit
        }
        SnafuNumber(number)
    }
}

impl SnafuNumber {
    pub fn to_snafu(&self) -> String {
        const DIGITS: [char; 5] = ['0', '1', '2', '=', '-'];

        let mut result = Vec::new();
        let mut decimal = self.0;

        loop {
            if decimal == 0 {
                break;
            }

            let digit = DIGITS[decimal as usize % 5];
            let new_decimal = (decimal + 2) / 5;
            result.push(digit);
            decimal = new_decimal;
        }

        result.iter().rev().collect()
    }
}

impl std::ops::Add for SnafuNumber {
    type Output = SnafuNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for SnafuNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

/// Find the shortest path in the maze
fn part1(numbers: &[SnafuNumber]) -> String {
    let result = SnafuNumber(numbers.iter().map(|n| n.0).sum::<i64>());
    result.to_snafu()
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> Vec<SnafuNumber> {
    input
        .lines()
        .map(|line| SnafuNumber::from(line))
        .collect_vec()
}

fn main() {
    let numbers = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_conversion() {
        assert_eq!(1747, SnafuNumber::from("1=-0-2").0);
        assert_eq!(906, SnafuNumber::from("12111").0);
        assert_eq!(198, SnafuNumber::from("2=0=").0);
        assert_eq!(11, SnafuNumber::from("21").0);
        assert_eq!(201, SnafuNumber::from("2=01").0);
        assert_eq!(31, SnafuNumber::from("111").0);
        assert_eq!(1257, SnafuNumber::from("20012").0);
        assert_eq!(32, SnafuNumber::from("112").0);
        assert_eq!(353, SnafuNumber::from("1=-1=").0);
        assert_eq!(107, SnafuNumber::from("1-12").0);
        assert_eq!(7, SnafuNumber::from("12").0);
        assert_eq!(3, SnafuNumber::from("1=").0);
        assert_eq!(37, SnafuNumber::from("122").0);
    }

    #[test]
    fn test_conversion_to_snafu() {
        let number = SnafuNumber(4890);
        assert_eq!("2=-1=0", &number.to_snafu());
    }

    #[test]
    fn check_part1() {
        let result = part1(&parse(INPUT));
        assert_eq!(SnafuNumber(4890).to_snafu(), result);
    }

    #[ignore]
    #[test]
    fn check_part2() {}
}
