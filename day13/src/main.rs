use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

/// Recursive structure to keep (nested) lists & digits
#[derive(Debug, PartialEq, Eq)]
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

fn part1(pairs: &[(Entry, Entry)]) -> u32 {
    let mut sum = 0u32;
    for (index, (left, right)) in pairs.iter().enumerate() {
        if left < right {
            println!("Index: {}", index);
            sum = sum + 1 + index as u32;
        }
    }

    sum
}

fn parse(input: &str) -> Vec<(Entry, Entry)> {
    input
        .split("\n\n")
        .into_iter()
        .filter_map(|block| block.split_once("\n"))
        .map(|(left, right)| (Entry::from(left), Entry::from(right)))
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
}
