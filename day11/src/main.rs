//! Day 11: Monkey In the Middle

use itertools::Itertools;

peg::parser! {
    grammar monkey_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule items() -> Vec<u64>
            = items:(number() ** ", ") { items }

        rule op_add() -> Operation
            = "+ " n:number() { Operation::Add(n) }

        rule op_mul_number() -> Operation
            = "* " n:number() { Operation::Mul(n) }

        rule op_mul_self() -> Operation
            = "* old" { Operation::MulSelf }

        pub(crate) rule id() -> u64
            = "Monkey " id:number() ":" { id }

        pub(crate) rule starting_itmes() -> Vec<u64>
            = "  Starting items: " items:items() { items }

        pub(crate) rule operation() -> Operation
            = "  Operation: new = old " op:(op_add() / op_mul_number() / op_mul_self()) { op }

        pub(crate) rule test() -> Test
            = "  Test: divisible by " divisible:number() "\n"
              "    If true: throw to monkey " if_true:number() "\n"
              "    If false: throw to monkey " if_false:number()
              {
                Test {
                    divisible,
                    if_true: if_true as usize,
                    if_false: if_false as usize,
                }
              }

        pub(crate) rule monkey() -> Monkey
            = id() "\n"
              items:starting_itmes() "\n"
              operation:operation() "\n"
              test:test() "\n"?
            {
                Monkey {
                    items, test, operation, inspections: 0,
                }
            }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Operation {
    Mul(u64),
    Add(u64),
    MulSelf,
}

impl Operation {
    pub fn apply(&self, level: u64) -> u64 {
        match self {
            Operation::Mul(value) => level * value,
            Operation::Add(value) => level + value,
            Operation::MulSelf => level * level,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Test {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

fn play(mut monkeys: Vec<Monkey>, num_rounds: u32, divisible: Option<u64>) -> u64 {
    let num_monkeys = monkeys.len();

    let common_denominator: u64 = monkeys.iter().map(|monkey| monkey.test.divisible).product();

    // play a nmber of N rounds
    for _ in 0..num_rounds {
        for index in 0..num_monkeys {
            let items = monkeys[index].items.drain(..).collect::<Vec<_>>();
            let operation = monkeys[index].operation;
            let test = monkeys[index].test.clone();

            for worry_level in items {
                let worry_level = operation.apply(worry_level);

                let worry_level = if let Some(divisible) = divisible {
                    worry_level / divisible
                } else {
                    worry_level % common_denominator
                };

                if worry_level % test.divisible == 0 {
                    monkeys[test.if_true].items.push(worry_level);
                } else {
                    monkeys[test.if_false].items.push(worry_level);
                }

                monkeys[index].inspections += 1;
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

/// Play a number of rounds, note how often items are inspected by monkeys
fn part1(monkeys: Vec<Monkey>) -> u64 {
    play(monkeys, 20, Some(3))
}

fn part2(monkeys: Vec<Monkey>) -> u64 {
    play(monkeys, 10_000, None)
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|block| monkey_parser::monkey(block).ok())
        .collect::<Vec<Monkey>>()
}

fn main() -> anyhow::Result<()> {
    let monkeys = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(monkeys.clone()));
    println!("Part 2: {}", part2(monkeys));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_parser_rules() {
        assert_eq!(0, monkey_parser::id("Monkey 0:").unwrap());
        assert_eq!(
            Ok(vec![79, 98]),
            monkey_parser::starting_itmes("  Starting items: 79, 98"),
        );
        assert_eq!(
            Ok(Operation::Mul(19)),
            monkey_parser::operation("  Operation: new = old * 19")
        );
        assert_eq!(
            Ok(Operation::Add(6)),
            monkey_parser::operation("  Operation: new = old + 6")
        );
        assert_eq!(
            Ok(Operation::MulSelf),
            monkey_parser::operation("  Operation: new = old * old")
        );
        let input = "  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3";
        assert_eq!(
            Ok(Test {
                divisible: 23,
                if_true: 2,
                if_false: 3,
            }),
            monkey_parser::test(input),
        )
    }

    #[test]
    fn check_parse_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        assert!(monkey_parser::monkey(input).is_ok());
    }

    #[test]
    fn check_part1() {
        let monkeys = parse(&INPUT);
        assert_eq!(10605, part1(monkeys));
    }

    #[test]
    fn check_part2() {
        let monkeys = parse(&INPUT);
        assert_eq!(2_713_310_158, part2(monkeys));
    }
}
