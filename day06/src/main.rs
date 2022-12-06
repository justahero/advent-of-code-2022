//! Day 06: Tuning Trouble

use std::collections::HashSet;

use itertools::Itertools;

/// Detects the start of packet marker, a four letter sequence where all letters are different
///
/// The first position of such a sequence is returned.
fn marker_pos(datastream: &str) -> Option<usize> {
    const NUM_ELEMENTS: usize = 4;
    datastream
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .into_iter()
        .enumerate()
        .map(|(index, (c1, c2, c3, c4))| {
            (index, vec![c1, c2, c3, c4].iter().collect::<HashSet<_>>().len())
        })
        .find(|(_, count)| *count == NUM_ELEMENTS)
        .map(|(index, _)| index + 4)
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(str::trim).collect::<Vec<_>>()
}

fn part1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .filter_map(|line| marker_pos(line))
        .map(|v| v as u32 + 3)
        .sum()
}

fn main() {
    let lines = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_part1() {
        assert_eq!(Some(7), marker_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(Some(5), marker_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(Some(6), marker_pos("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(Some(10), marker_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(Some(11), marker_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
