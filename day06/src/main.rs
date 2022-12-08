//! Day 06: Tuning Trouble

use std::collections::HashSet;

/// Detects the start of packet marker, a four letter sequence where all letters are different
///
/// The first position of such a sequence is returned.
fn marker_pos(datastream: &str, distinct: usize) -> Option<usize> {
    let chars = datastream.chars().collect::<Vec<_>>();
    chars
        .windows(distinct)
        .map(|seq| seq.iter().collect::<HashSet<_>>())
        .position(|seq| seq.len() == distinct)
        .map(|pos| pos + distinct)
}

fn part1(line: &str) -> usize {
    marker_pos(line, 4).expect("Failed to find start of packet")
}

fn part2(line: &str) -> usize {
    marker_pos(line, 14).expect("Failed to find start of packet")
}

fn main() {
    let lines = include_str!("input.txt");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_part1() {
        assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn check_part2() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
