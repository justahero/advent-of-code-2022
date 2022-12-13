use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list1, sequence::delimited, IResult,
};

#[derive(Debug)]
struct Pair {
    pub left: String,
    pub right: String,
}

impl Pair {
    pub fn new(left: &str, right: &str) -> Self {
        Pair {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

/// Recursive structure to keep (nested) lists & digits
#[derive(Debug, PartialEq, Eq)]
enum Entry {
    List(Box<Vec<Entry>>),
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

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((
        map(
            delimited(tag("["), separated_list1(tag(","), parse_entry), tag("]")),
            |inner| Entry::List(Box::new(inner)),
        ),
        map(digit1, Entry::int),
    ))(input)
}

fn part1(pairs: &[Pair]) -> u32 {
    let pair = &pairs[0];
    let left = parse_entry(&pair.left);

    0
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .into_iter()
        .filter_map(|block| block.split_once("\n"))
        .map(|(left, right)| Pair::new(left, right))
        .collect()
}

fn main() {
    let pairs = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_parse_entry() {
        let (_, entry) = parse_entry("[1,1,3,1,1]").unwrap();
        assert_eq!(
            Entry::List(Box::new(vec![
                Entry::Int(1),
                Entry::Int(1),
                Entry::Int(3),
                Entry::Int(1),
                Entry::Int(1)
            ])),
            entry
        );
        let (_, entry) = parse_entry("[[1],[2,3,4]]").unwrap();
        assert_eq!(
            Entry::List(Box::new(vec![
                Entry::List(Box::new(vec![Entry::Int(1)])),
                Entry::List(Box::new(vec![Entry::Int(2), Entry::Int(3), Entry::Int(4)])),
            ])),
            entry
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(13, part1(&parse(INPUT)));
    }
}
