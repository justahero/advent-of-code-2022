//! Day 11: Monkey In the Middle

peg::parser! {
    grammar monkey_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule items() -> Vec<u32>
            = items:(number() ** ", ") { items }

        rule op_add() -> Operation
            = "+ " n:number() { Operation::Add(n) }

        rule op_mul_number() -> Operation
            = "* " n:number() { Operation::Mul(n) }

        rule op_mul_self() -> Operation
            = "* old" { Operation::MulSelf }

        pub(crate) rule id() -> u32
            = "Monkey " id:number() ":" { id }

        pub(crate) rule starting_itmes() -> Vec<u32>
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
                    if_true,
                    if_false,
                }
              }

        pub(crate) rule monkey() -> Monkey
            = id:id() "\n"
              items:starting_itmes() "\n"
              operation:operation() "\n"
              test:test() "\n"?
            {
                Monkey {
                    id, items, test, operation,
                }
            }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Mul(u32),
    Add(u32),
    MulSelf,
}

#[derive(Debug, PartialEq, Eq)]
struct Test {
    divisible: u32,
    if_true: u32,
    if_false: u32,
}

#[derive(Debug)]
struct Monkey {
    /// The monkey identifier
    id: u32,
    /// The starting items.
    items: Vec<u32>,
    operation: Operation,
    test: Test,
}

fn part1(monkeys: &[Monkey]) -> u64 {
    0
}

fn parse(input: &str) -> anyhow::Result<Vec<Monkey>> {
    let monkeys = input
        .split("\n\n")
        .filter_map(|block| {
            println!("BLOCK: '{}'", block);
            monkey_parser::monkey(block).ok()
        })
        .collect::<Vec<Monkey>>();

    println!("MONKEYS: {:?}", monkeys);

    todo!("")
}

fn main() -> anyhow::Result<()> {
    let monkeys = parse(include_str!("input.txt"))?;
    println!("Part 1: {}", part1(&monkeys));
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_part1() {
        let monkeys = parse(&INPUT).expect("Failed to parse input");
        assert_eq!(10605, part1(&monkeys));
    }

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

        let result = monkey_parser::monkey(input).expect("failed");
        // assert!(monkey_parser::monkey(input).is_ok());
    }
}
