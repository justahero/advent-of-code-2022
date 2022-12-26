//! Day 12: Hill Climbing Algorithm

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

/// Recursive structure to keep (nested) lists & digits
#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum Entry {
    List(Vec<Entry>),
    Int(u8),
}

impl Entry {
    pub fn int(input: &str) -> Self {
        Entry::Int(
            format!("{}", input)
                .parse::<u8>()
                .expect("Failed to parse number"),
        )
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Entry::Int(l), Entry::Int(r)) => l.partial_cmp(r),
            (Entry::Int(l), Entry::List(r)) => vec![Entry::Int(*l)].partial_cmp(r),
            (Entry::List(l), Entry::List(r)) => l.partial_cmp(r),
            (Entry::List(l), Entry::Int(r)) => l.partial_cmp(&vec![Entry::Int(*r)]),
        }
    }
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(
            delimited(tag("["), separated_list0(tag(","), parse_entry), tag("]")),
            |inner| Entry::List(inner),
        ),
        map(digit1, Entry::int),
    ))(input)
}

impl From<&str> for Entry {
    fn from(input: &str) -> Self {
        let (_, entry) = parse_entry(input).expect("Failed to parse list");
        entry
    }
}

fn part1(pairs: &[Entry]) -> u32 {
    pairs
        .iter()
        .as_slice()
        .chunks(2)
        .enumerate()
        .fold(0, |sum, (index, pair)| {
            if &pair[0] < &pair[1] {
                sum + 1 + index as u32
            } else {
                sum
            }
        })
}

fn part2(mut pairs: Vec<Entry>) -> usize {
    let left: Entry = Entry::from("[[2]]");
    let right: Entry = Entry::from("[[6]]");

    pairs.push(Entry::from("[[2]]"));
    pairs.push(Entry::from("[[6]]"));
    pairs.sort_unstable();

    let first = pairs.iter().position(|p| *p == left).unwrap() + 1;
    let second = pairs.iter().position(|p| *p == right).unwrap() + 1;

    first * second
}

fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| Entry::from(line))
        .collect()
}

fn main() {
    let pairs = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_parse_entry() {
        let (_, entry) = parse_entry("[1,1,3,1,1]").unwrap();
        assert_eq!(
            Entry::List(vec![
                Entry::Int(1),
                Entry::Int(1),
                Entry::Int(3),
                Entry::Int(1),
                Entry::Int(1)
            ]),
            entry
        );
        let (_, entry) = parse_entry("[[1],[2,3,4]]").unwrap();
        assert_eq!(
            Entry::List(vec![
                Entry::List(vec![Entry::Int(1)]),
                Entry::List(vec![Entry::Int(2), Entry::Int(3), Entry::Int(4)]),
            ]),
            entry
        );
        let (_, entry) = parse_entry("[]").unwrap();
        assert_eq!(Entry::List(vec![]), entry,);
    }

    #[test]
    fn check_part1() {
        assert_eq!(13, part1(&parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(140, part2(parse(INPUT)));
    }
}
