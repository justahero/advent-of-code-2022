//! Day 05: Supply Stacks

use anyhow::anyhow;

#[derive(Debug)]
struct SupplyStack {
    stacks: Vec<Vec<char>>,
}

impl SupplyStack {
    pub fn new(rows: &[Vec<char>]) -> Self {
        // transpose rows into columns
        // following article: https://www.hackertouch.com/matrix-transposition-in-rust.html
        let mut stacks = vec![Vec::with_capacity(rows.len()); rows[0].len()];
        for row in rows {
            for index in 0..row.len() {
                let c = row[index];
                if c.is_alphabetic() {
                    stacks[index].push(c);
                }
            }
        }

        Self { stacks }
    }
}

// TODO change into Crate, using enum?
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

fn parse(input: &str) -> anyhow::Result<SupplyStack> {
    let (stack, _moves) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Failed to split input"))?;

    let stacks = stack
        .lines()
        .filter_map(|line| stack_parser::line(line).ok())
        .collect::<Vec<_>>();

    let stacks = SupplyStack::new(&stacks);

    println!("STACKS: {:?}", stacks);

    todo!("")
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
    fn check_parser() -> anyhow::Result<()> {
        assert_eq!(vec!['.', 'D', '.'], stack_parser::line("    [D]    ")?);
        assert_eq!(vec!['1', '2', '3'], stack_parser::line(" 1   2   3 ")?);
        Ok(())
    }

    #[test]
    fn check_part1() -> anyhow::Result<()> {
        let _input = parse(INPUT)?;
        // assert_eq!(2, part1(&parse(INPUT)));
        Ok(())
    }
}
