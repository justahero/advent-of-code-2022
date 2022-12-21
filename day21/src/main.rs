//! Day 20: Monkey Math

use std::collections::HashMap;

peg::parser! {
    /// Parses monkey operations
    grammar line_parser() for str {
        rule name() -> String
            = name:$(['a'..='z']+) { name.to_string() }

        rule number() -> i32
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule yell() -> Operation
            = n:number() { Operation::Yell(n) }

        rule add() -> Operation
            = l:name() " + " r:name() { Operation::Add(l.into(), r.into()) }

        rule div() -> Operation
            = l:name() " / " r:name() { Operation::Div(l.into(), r.into()) }

        rule mul() -> Operation
            = l:name() " * " r:name() { Operation::Mul(l.into(), r.into()) }

        rule sub() -> Operation
            = l:name() " - " r:name() { Operation::Sub(l.into(), r.into()) }

        rule operation() -> Operation
            = op:(yell() / add() / div() / mul() / sub()) { op }

        pub(crate) rule monkey() -> (String, Operation)
            = name:name() ": " op:operation() { (name, op) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Yell(i32),
    Add(String, String),
    Div(String, String),
    Mul(String, String),
    Sub(String, String),
}

fn part1(monkeys: HashMap<String, Operation>) -> i64 {
    0
}

/// Parses the string, returns a map of monkey id to operation
fn parse(input: &str) -> HashMap<String, Operation> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line_parser::monkey(line).ok())
        .collect::<HashMap<_, _>>()
}

fn main() {
    let encrypted_file = parse(include_str!("input.txt"));
    // println!("Part 1: {}", part1(encrypted_file.clone()));
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
            Ok(("root".into(), Operation::Add("pppw".into(), "sjmn".into()))),
            line_parser::monkey("root: pppw + sjmn"),
        );
    }

    #[test]
    fn check_part1() {
        // assert_eq!(3, part1(parse(INPUT)));
    }
}
