//! Day 21: Monkey Math

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
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Sub => "-",
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
            _ => panic!("Unsupported op '{}' found", input),
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

    pub fn evaluate(&self, monkeys: &HashMap<String, Instruction>) -> i64 {
        match self {
            Instruction::Yell(n) => *n,
            Instruction::Operation(left, op, right) => {
                let (left, right) = (&monkeys[left], &monkeys[right]);
                let result = match op {
                    Op::Add => left.evaluate(monkeys) + right.evaluate(monkeys),
                    Op::Mul => left.evaluate(monkeys) * right.evaluate(monkeys),
                    Op::Div => left.evaluate(monkeys) / right.evaluate(monkeys),
                    Op::Sub => left.evaluate(monkeys) - right.evaluate(monkeys),
                };
                result
            }
        }
    }
}

/// Graph traversal function
fn traverse(name: &str, value: i64, monkeys: &HashMap<String, Instruction>) -> i64 {
    if name == "humn" {
        return value;
    }

    match &monkeys[name] {
        Instruction::Yell(n) => *n,
        Instruction::Operation(left, op, right) => {
            let (next_monkey, new_value) = if is_human_branch(left, monkeys) {
                // invert calculation for human branch
                let right_total = monkeys[right].evaluate(&monkeys);
                let new_value = match op {
                    Op::Add => value - right_total,
                    Op::Mul => value / right_total,
                    Op::Div => value * right_total,
                    Op::Sub => value + right_total,
                };
                (left, new_value)
            } else {
                // normal evalulation
                let left_total = monkeys[left].evaluate(&monkeys);
                let new_value = match op {
                    Op::Add => value - left_total,
                    Op::Sub => left_total - value,
                    Op::Mul => value / left_total,
                    Op::Div => left_total / value,
                };
                (right, new_value)
            };

            traverse(next_monkey, new_value, monkeys)
        }
    }
}

/// Helper function to check if this node is part of the "human" branch
fn is_human_branch(name: &str, monkeys: &HashMap<String, Instruction>) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkeys[name] {
        Instruction::Yell(_) => false,
        Instruction::Operation(left, _, right) => {
            is_human_branch(left, monkeys) || is_human_branch(right, monkeys)
        }
    }
}

fn part1(monkeys: &HashMap<String, Instruction>) -> i64 {
    monkeys["root"].evaluate(&monkeys)
}

/// Find chain of instructions from root to "humn", determine the value of the other
/// branch in root, then invert all instructions down (with value) to "humn", then evaluate
/// ignore sub-branch below "humn"
fn part2(monkeys: &HashMap<String, Instruction>) -> i64 {
    if let Instruction::Operation(left, _, right) = &monkeys["root"] {
        // check which branch needs to get calculated for root
        let (name, root_total) = if is_human_branch(left, monkeys) {
            (left, monkeys[right].evaluate(&monkeys))
        } else {
            (right, monkeys[left].evaluate(&monkeys))
        };

        traverse(name, root_total, monkeys)
    } else {
        panic!("Root is not an infix operation.");
    }
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
    println!("Part 1: {}", part1(&monkeys));
    println!("Part 2: {}", part2(&monkeys));

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
        assert_eq!(152, part1(&parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        assert_eq!(301, part2(&parse(INPUT)));
    }
}
