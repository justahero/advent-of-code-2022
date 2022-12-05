//! Day 05: Supply Stacks

use anyhow::anyhow;

#[derive(Debug)]
pub struct SupplyStack {
    pub stacks: Vec<Vec<char>>,
}

impl SupplyStack {
    pub fn new(rows: &[Vec<char>]) -> Self {
        let mut stacks = vec![Vec::with_capacity(rows.len()); rows[0].len()];
        for row in rows {
            for (index, c) in row.iter().enumerate() {
                if c.is_alphabetic() {
                    stacks[index].push(*c);
                }
            }
        }
        Self { stacks }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub num_crates: u32,
    pub from: u32,
    pub to: u32,
}

impl Move {
    pub fn new(num_crates: u32, from: u32, to: u32) -> Self {
        Self {
            num_crates,
            from,
            to,
        }
    }
}

peg::parser! {
    grammar stack_parser() for str {
        rule number_slot() -> char
            = " " n:['0'..='9'] " " { n }

        rule empty_slot() -> char
            = "   " { '.' }

        rule filled_slot() -> char
            = "[" c:['A'..='Z'] "]" { c }

        rule slot() -> char
            = filled_slot() / empty_slot() / number_slot()

        pub rule line() -> Vec<char>
            = s:(slot() ** " " ) { s }
    }
}

peg::parser! {
    grammar move_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) { n.parse::<u32>().unwrap() }

        rule digit() -> u32
            = n:['0'..='9'] { n.to_string().parse::<u32>().unwrap() }

        /// Parses the line "move 1 from 2 to 1"
        pub rule line() -> Move
            = "move " num:number() " from " f:digit() " to " t:digit() { Move::new(num, f, t) }
    }
}

fn parse(input: &str) -> anyhow::Result<(SupplyStack, Vec<Move>)> {
    let (stack, moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to split input"))?;

    let stacks = stack
        .lines()
        .filter_map(|line| stack_parser::line(line).ok())
        .collect::<Vec<_>>();

    let stack = SupplyStack::new(&stacks);
    let moves = moves
        .lines()
        .filter_map(|line| move_parser::line(line).ok())
        .collect::<Vec<_>>();

    Ok((stack, moves))
}

fn part1() {
    todo!()
}

fn main() -> anyhow::Result<()> {
    let input = parse(include_str!("input.txt"))?;
    // println!("Part 1: {}", part1(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    // This time the formating needs to match the input file
    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn check_stack_parser() -> anyhow::Result<()> {
        assert_eq!(vec!['.', 'D', '.'], stack_parser::line("    [D]    ")?);
        assert_eq!(vec!['1', '2', '3'], stack_parser::line(" 1   2   3 ")?);
        Ok(())
    }

    #[test]
    fn check_move_parser() -> anyhow::Result<()> {
        assert_eq!(
            Ok(Move::new(1, 2, 1)),
            move_parser::line("move 1 from 2 to 1")
        );
        assert_eq!(
            Ok(Move::new(10, 1, 3)),
            move_parser::line("move 10 from 1 to 3")
        );
        Ok(())
    }

    #[test]
    fn check_part1() -> anyhow::Result<()> {
        let _input = parse(INPUT)?;
        // assert_eq!(2, part1(&parse(INPUT)));
        Ok(())
    }
}
