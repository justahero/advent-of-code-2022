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

    /*
    fn from(value: &str) -> Self {
        let s = value.chars().map(|c| match c {

        }).collect::<String>();
        SnafuNumber(i64::from_str_radix(&s, 5))
    }
    */
}

impl SnafuNumber {
    pub fn to_snafu(&self) -> String {
        todo!()
    }
}

impl std::ops::Add for SnafuNumber {
    type Output = SnafuNumber;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
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
    fn check_part1() {}

    #[ignore]
    #[test]
    fn check_part2() {}
}
