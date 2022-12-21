//! Day 20: Monkey Math

use anyhow::anyhow;
use petgraph::prelude::DiGraphMap;

use std::{collections::HashMap, fmt::Display};

type MonkeyGraph = DiGraphMap<i32, ()>;

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

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    instruction: Instruction,
}

impl Monkey {
    pub fn new(name: String, instruction: Instruction) -> Self {
        Self { name, instruction }
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
}

fn part1(monkeys: Vec<Monkey>, graph: MonkeyGraph) -> anyhow::Result<i64> {
    // let root = graph..get("root").ok_or(anyhow!("Failed to find root"))?;
    // root.evaluate(&graph)

    todo!("")
}

fn part2(monkeys: Vec<Monkey>, graph: MonkeyGraph) -> anyhow::Result<i64> {
    // Find chain of instructions from root to "humn", determine the value of the other
    // branch in root, then invert all instructions down (with value) to "humn", then evaluate
    // ignore sub-branch below "humn"

    // "humn" as in human
    // Find the instructions chain from root to "humn"

    Ok(0)
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> (Vec<Monkey>, MonkeyGraph) {
    let monkeys = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line_parser::monkey(line).ok())
        .map(|(monkey, instruction)| Monkey::new(monkey, instruction))
        .collect::<Vec<_>>();

    // Generate all edges
    let edges = monkeys
        .iter()
        .enumerate()
        .flat_map(|(index, monkey)| match &monkey.instruction {
            Instruction::Operation(left, _op, right) => {
                let left = monkeys
                    .iter()
                    .position(|monkey| monkey.name == *left)
                    .unwrap() as i32;
                let right = monkeys
                    .iter()
                    .position(|monkey| monkey.name == *right)
                    .unwrap() as i32;
                vec![(left, index as i32), (right, index as i32)]
            }
            Instruction::Yell(_value) => vec![],
        })
        .collect::<Vec<_>>();

    (monkeys, MonkeyGraph::from_edges(edges))
}

fn main() -> anyhow::Result<()> {
    let (monkeys, graph) = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(monkeys.clone(), graph.clone())?);
    println!("Part 2: {}", part2(monkeys, graph)?);

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
        let (monkeys, graph) = parse(INPUT);
        assert_eq!(152, part1(monkeys, graph).unwrap());
    }

    #[test]
    fn check_part2() {
        let (monkeys, graph) = parse(INPUT);
        assert_eq!(301, part2(monkeys, graph).unwrap());
    }
}
