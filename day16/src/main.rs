use std::collections::BTreeMap;

use pathfinding::prelude::{dijkstra, dijkstra_all};

peg::parser! {
    grammar line_parser() for str {
        rule valve() -> String
            = s:$(['A'..='Z']+) { s.to_string() }

        rule rate() -> i32
            = n:$(['0'..='9']+) {? n.parse::<i32>().or(Err("Failed to parse number")) }

        rule tunnels() -> Vec<String>
            = valves:(valve() ** ", ") { valves }

        rule tunnel_leads()
            = "tunnels lead to valves" / "tunnel leads to valve"

        pub(crate) rule pipe() -> Pipe
            = "Valve " valve:valve() " has flow rate=" rate:rate() "; " tunnel_leads() " " tunnels:tunnels()
            {
                Pipe {
                    valve,
                    flow_rate: rate,
                    tunnels,
                    open: false,
                }
            }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Valve(String);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pipe {
    valve: String,
    flow_rate: i32,
    tunnels: Vec<String>,
    open: bool,
}

/// Determine the best path to maximume the flow of all open valves.
///
/// Travelling salesman problem!
/// Issue is a greedy algorithm ignores the best path, only considers the next best position
/// but ignoring optimal path finding of all nodes.
fn open_valves(pipes: Vec<Pipe>, num_rounds: i32) -> usize {
    // Look up table
    let mut pipes_by_label: BTreeMap<String, Pipe> = pipes
        .into_iter()
        .map(|pipe| (pipe.valve.to_string(), pipe))
        .collect::<BTreeMap<_, _>>();

    let mut current_valve = "AA".to_string();
    let mut total_flow: i32 = 0;

    // play all rounds
    for round in 0..num_rounds {
        println!("== MINUTE {} ==", round + 1);
        println!();
    }

    total_flow as usize
}

fn part1(pipes: Vec<Pipe>) -> usize {
    open_valves(pipes, 30)
}

fn parse(input: &str) -> Vec<Pipe> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line_parser::pipe(line))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse pipes")
}

fn main() {
    let pipes = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(pipes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "#;

    #[test]
    fn check_parser() {
        assert_eq!(
            Ok(Pipe {
                valve: "AA".into(),
                flow_rate: 0,
                tunnels: vec!["DD".into(), "II".into(), "BB".into()],
                open: false,
            }),
            line_parser::pipe("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
        );

        // one entry with 'tunnel' instead of 'tunnels'
        assert!(line_parser::pipe("Valve HH has flow rate=22; tunnel leads to valve GG").is_ok());
    }

    #[test]
    fn check_part1() {
        assert_eq!(1651, part1(parse(INPUT)));
    }

    #[test]
    fn check_part2() {
        // assert_eq!(56_000_011, part2(parse(INPUT), 20));
    }
}
