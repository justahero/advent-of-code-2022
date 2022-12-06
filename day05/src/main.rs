//! Day 05: Supply Stacks

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct SupplyStack {
    pub stacks: Vec<Vec<char>>,
}

impl SupplyStack {
    /// A slightly better handling might have been to put all stacks into a HashMap, due to indices starting with `1`
    /// Found this quite neat "transpose" implementation, slightly adjusted to skip non-alphabetical characters
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

    /// Applies a single move on the stack, each crate is picked one by one from
    /// source to target stack, thereby reversing the order of the crates.
    ///
    /// "from" and "to" indices start at `1`.
    pub fn single(&mut self, mv: &Move) -> anyhow::Result<()> {
        // Good example for ranges, classical for loop
        // Not the fastest way to "move" crates, but it's ok
        for _ in 0..mv.num_crates {
            if self.stacks[mv.from - 1].is_empty() {
                return Err(anyhow!("Cannot take crate from empty stack"));
            }
            let c = self.stacks[mv.from - 1].remove(0);
            self.stacks[mv.to - 1].insert(0, c);
        }
        Ok(())
    }

    /// Applies a single move on the stack, the pile of crates are picked at once
    /// and moved from source to target stack, the order of crates does not change.
    pub fn multi(&mut self, mv: &Move) -> anyhow::Result<()> {
        for index in (0..mv.num_crates as usize).rev() {
            let c = self.stacks[mv.from - 1].remove(index);
            self.stacks[mv.to - 1].insert(0, c);
        }
        Ok(())
    }

    /// Returns the top crates from the stacks, ignores any empty stacks
    pub fn top(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.first())
            .collect::<String>()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub num_crates: u32,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn new(num_crates: u32, from: usize, to: usize) -> Self {
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

        rule digit() -> usize
            = n:['0'..='9'] { n.to_string().parse::<usize>().unwrap() }

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

/// Run all moves for crane, re-arrange the stacks
fn part1(stack: &mut SupplyStack, moves: &[Move]) -> anyhow::Result<String> {
    // using `?` operator to return an `Err` works in a `for` loop, but not from a closure
    for m in moves {
        stack.single(m)?;
    }
    Ok(stack.top())
}

fn part2(stack: &mut SupplyStack, moves: &[Move]) -> anyhow::Result<String> {
    for m in moves {
        stack.multi(m)?;
    }
    Ok(stack.top())
}

fn main() -> anyhow::Result<()> {
    let (stack, moves) = parse(include_str!("input.txt"))?;
    println!("Part 1: {}", part1(&mut stack.clone(), &moves)?);
    println!("Part 2: {}", part2(&mut stack.clone(), &moves)?);

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
        let (mut stack, moves) = parse(INPUT)?;
        assert_eq!("CMZ", part1(&mut stack, &moves)?);
        Ok(())
    }

    #[test]
    fn check_part2() -> anyhow::Result<()> {
        let (mut stack, moves) = parse(INPUT)?;
        assert_eq!("MCD", part2(&mut stack, &moves)?);
        Ok(())
    }
}
