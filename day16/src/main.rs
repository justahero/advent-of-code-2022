use std::collections::BTreeMap;

use array2d::Array2D;
use itertools::Itertools;

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

/// A network of connected tunnels & valves.
#[derive(Debug)]
struct Network {
    start: usize,
    end: usize,
    flow_rates: BTreeMap<usize, i32>,
    tunnels: Array2D<i32>,
}

fn total_flow(num_rounds: i32, start: &str, pipes: &[&&Pipe]) -> usize {
    0
}

/// Determine the best path to maximume the flow of all open valves.
///
/// Travelling salesman problem!
/// Issue is a greedy algorithm ignores the best path, only considers the next best position
/// but ignoring optimal path finding of all nodes.
fn open_valves(pipes: Vec<Pipe>, num_rounds: i32) -> Option<usize> {
    let current_valve = "AA".to_string();

    // Get the list of possible permutations of pipes that have a flow rate
    /*
    let with_flow_rates = pipes
        .iter()
        .filter(|&pipe| pipe.flow_rate > 0)
        .collect::<Vec<_>>();

    let length = with_flow_rates.len();
    println!("LENGTH: {}", length);
    */

    let indexes: BTreeMap<String, usize> = pipes
        .iter()
        .enumerate()
        .map(|(index, pipe)| (pipe.valve.to_string(), index))
        .collect::<BTreeMap<_, _>>();

    let mut distribution = Array2D::filled_with(i32::MAX, pipes.len(), pipes.len());

    // Fill in matrix all existing pipes
    for pipe in pipes.iter() {
        distribution[(indexes[&pipe.valve], indexes[&pipe.valve])] = 0;
        for tunnel in &pipe.tunnels {
            distribution[(indexes[&pipe.valve], indexes[tunnel])] = 1;
        }
    }

    // Apply Floyd-Marshall to calculate shortest path for each pair of nodes
    // See https://www.programiz.com/dsa/floyd-warshall-algorithm
    for k in 0..indexes.len() {
        for i in 0..indexes.len() {
            for j in 0..indexes.len() {
                distribution[(i, j)] = std::cmp::min(
                    distribution[(i, j)],
                    distribution[(i, k)] + distribution[(k, j)],
                );
            }
        }
    }

    Some(0)
}

fn part1(pipes: Vec<Pipe>) -> usize {
    open_valves(pipes, 30).expect("Failed to find max value")
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
