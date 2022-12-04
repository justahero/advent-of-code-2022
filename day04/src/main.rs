//! Day 04: Camp Cleanup
use bitvec::prelude::*;

fn part1(sections: &[BitVec]) -> u32 {
    todo!()
}

fn parse(input: &str) -> Vec<BitVec> {
    let x = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.split_once(","))
        .collect::<Vec<_>>();

    Vec::new()
}

fn main() {
    let input = parse(include_str!("input.txt"));
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
    fn check_part1() {
        assert_eq!(2, part1(&parse(INPUT)));
    }
}
