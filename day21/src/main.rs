//! Day 20: Monkey Math

use anyhow::anyhow;

use std::{collections::HashMap, fmt::Display};

peg::parser! {
    /// Parses monkey instructions
    grammar line_parser() for str {
        rule name() -> String
            = name:$(['a'..='z']+) { name.to_string() }

        rule number() -> i64
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule yell() -> Instruction
            = n:number() { Instruction::Yell(n) }

        rule op() -> Op
            = op:$("+" / "*" / "/" / "-") { op.into() }

        rule operation() -> Instruction
            = l:name() " " op:op() " " r:name() { Instruction::operation(op, l, r) }

        rule instruction() -> Instruction
            = op:(yell() / operation()) { op }

        pub(crate) rule monkey() -> (String, Instruction)
            = name:name() ": " op:instruction() { (name, op) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
    Div,
    Sub,
    Equal,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Sub => "-",
            Op::Equal => "=",
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        match input {
            "+" => Op::Add,
            "*" => Op::Mul,
            "/" => Op::Div,
            "-" => Op::Sub,
            "=" => Op::Equal,
            _ => panic!("Unsupported op '{}' found", input),
        }
    }
}

impl Op {
    pub fn invert(&self) -> Self {
        match self {
            Op::Add => Op::Sub,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
            Op::Sub => Op::Add,
            Op::Equal => Op::Equal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Yell(i64),
    Operation(String, Op, String),
}

impl Instruction {
    pub fn operation(op: Op, left: String, right: String) -> Self {
        Self::Operation(left, op, right)
    }

    pub fn evaluate(&self, monkeys: &HashMap<String, Instruction>) -> anyhow::Result<i64> {
        match self {
            Instruction::Yell(n) => Ok(*n),
            Instruction::Operation(left, op, right) => {
                let left = monkeys.get(left).ok_or(anyhow!("Monkey not found"))?;
                let right = monkeys.get(right).ok_or(anyhow!("Monkey not found"))?;
                let result = match op {
                    Op::Add => left.evaluate(monkeys)? + right.evaluate(monkeys)?,
                    Op::Mul => left.evaluate(monkeys)? * right.evaluate(monkeys)?,
                    Op::Div => left.evaluate(monkeys)? / right.evaluate(monkeys)?,
                    Op::Sub => left.evaluate(monkeys)? - right.evaluate(monkeys)?,
                    Op::Equal => 0,
                };
                Ok(result)
            }
        }
    }

    pub fn invert(&self) -> Instruction {
        match self {
            Instruction::Yell(value) => Instruction::Yell(-value),
            Instruction::Operation(left, op, right) => {
                Instruction::Operation(left.to_string(), op.invert(), right.to_string())
            }
        }
    }

    /// Determine the chain of this instruction to the root
    pub fn chain(
        start: &str,
        monkeys: &HashMap<String, Instruction>,
    ) -> anyhow::Result<Vec<(String, Instruction)>> {
        let mut start = start.to_string();
        let mut chain = vec![];

        loop {
            let result = monkeys
                .iter()
                .find(|&(_name, instruction)| match instruction {
                    Instruction::Operation(left, _, right) => left == &start || right == &start,
                    _ => false,
                });

            if let Some((parent, instruction)) = result {
                start = parent.clone();
                chain.push((parent.clone(), instruction.clone()));
            } else {
                break;
            }
        }

        Ok(chain)
    }
}

fn part1(monkeys: HashMap<String, Instruction>) -> anyhow::Result<i64> {
    let root = monkeys.get("root").ok_or(anyhow!("Failed to find root"))?;
    root.evaluate(&monkeys)
}

fn part2(mut monkeys: HashMap<String, Instruction>) -> anyhow::Result<i64> {
    // Find chain of instructions from root to "humn", determine the value of the other
    // branch in root, then invert all instructions down (with value) to "humn", then evaluate
    // ignore sub-branch below "humn"

    // "humn" as in human
    // Find the instructions chain from root to "humn"
    let chain = Instruction::chain("humn", &monkeys)?;
    println!("CHAIN: {:?}", chain);

    let (child, _) = chain
        .iter()
        .rev()
        .nth(1)
        .ok_or(anyhow!("Could not find root child"))?;
    println!("CHILD: {}", child);

    // Get the alternative branch
    let other_child = match monkeys.get("root").ok_or(anyhow!("No root"))? {
        Instruction::Operation(left, _, right) => {
            if left == child {
                println!("  Right: {}", right);
                monkeys
                    .get(right)
                    .ok_or(anyhow!("Failed to get instruction"))?
            } else {
                println!("  Left: {}", left);
                monkeys
                    .get(left)
                    .ok_or(anyhow!("Failed to get instruction"))?
            }
        }
        _ => return Err(anyhow!("Root is not a tuple operation")),
    };

    let total = other_child.evaluate(&monkeys)?;
    println!("Expected total: {}", total);

    // Invert the chain of instructions from root to "humn"
    for (name, instruction) in chain.iter() {
        match instruction.invert() {
            Instruction::Operation(left, op, right) => {
                if left == *name {
                    println!(":: {} {} {}", left, op, right);
                } else {
                    println!(":: {} {} {}", left, op, right);
                }
            }
            Instruction::Yell(_) => todo!(),
        }
    }

    Ok(0)
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> HashMap<String, Instruction> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line_parser::monkey(line).ok())
        .collect::<HashMap<_, _>>()
}

fn main() -> anyhow::Result<()> {
    let monkeys = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(monkeys.clone())?);
    println!("Part 2: {}", part2(monkeys)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    ";

    #[test]
    fn check_parser() {
        assert_eq!(
            Ok((
                "root".into(),
                Instruction::Operation("pppw".into(), Op::Add, "sjmn".into())
            )),
            line_parser::monkey("root: pppw + sjmn"),
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(152, part1(parse(INPUT)).unwrap());
    }

    #[test]
    fn check_part2() {
        assert_eq!(301, part2(parse(INPUT)).unwrap());
    }
}
