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

#[derive(Debug)]
struct List(Vec<Entry>);

#[derive(Debug)]
enum Entry {
    List(List),
    Numbers(Vec<u8>),
}

impl Entry {
    pub fn int(input: &str) -> Self {
        Entry::Numbers(vec![format!("{}", input)
            .parse::<u8>()
            .expect("Failed to parse number")])
    }

    pub fn numbers(input: &[&str]) -> Self {
        let numbers = input
            .iter()
            .filter_map(|s| format!("{}", s).parse::<u8>().ok())
            .collect::<Vec<_>>();
        Entry::Numbers(numbers)
    }
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, entry) = alt((
        map(delimited(tag("["), parse_entry, tag("]")), |inner| {
            Entry::List(List(vec![inner]))
        }),
        map(separated_list1(tag(","), digit1), |n| Entry::numbers(&n)),
        map(digit1, Entry::int),
    ))(input)?;

    // delimited(tag("["), parse_entry, tag("]"))(input)?;

    todo!("")
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
    fn check_part1() {
        assert_eq!(13, part1(&parse(INPUT)));
    }
}
