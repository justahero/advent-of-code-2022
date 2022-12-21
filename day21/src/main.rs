//! Day 20: Monkey Math

use anyhow::anyhow;

use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Yell(i64),
    WhatToYell,
    Operation(String, Op, String),
}

impl Instruction {
    pub fn operation(op: Op, left: String, right: String) -> Self {
        Self::Operation(left, op, right)
    }

    pub fn evaluate(&self, monkeys: &HashMap<String, Instruction>) -> anyhow::Result<i64> {
        match self {
            Instruction::Yell(n) => Ok(*n),
            Instruction::WhatToYell => todo!(),
            Instruction::Operation(left, op, right) => {
                let left = monkeys.get(left).ok_or(anyhow!("Monkey not found"))?;
                let right = monkeys.get(right).ok_or(anyhow!("Monkey not found"))?;
                let result = match op {
                    Op::Add => left.evaluate(monkeys)? + right.evaluate(monkeys)?,
                    Op::Mul => left.evaluate(monkeys)? * right.evaluate(monkeys)?,
                    Op::Div => left.evaluate(monkeys)? / right.evaluate(monkeys)?,
                    Op::Sub => left.evaluate(monkeys)? - right.evaluate(monkeys)?,
                    Op::Equal => todo!(),
                };
                Ok(result)
            }
        }
    }
}

fn part1(monkeys: HashMap<String, Instruction>) -> anyhow::Result<i64> {
    let root = monkeys.get("root").ok_or(anyhow!("Failed to find root"))?;
    root.evaluate(&monkeys)
}

fn part2(mut monkeys: HashMap<String, Instruction>) -> anyhow::Result<i64> {
    let root = monkeys.get("root").ok_or(anyhow!("Failed to find root"))?;
    let second = monkeys
        .entry("humn".to_string())
        .and_modify(|entry| *entry = Instruction::WhatToYell);

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
