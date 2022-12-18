use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashMap},
};

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
    flow_rates: BTreeMap<usize, i32>,
    tunnels: Array2D<i32>,
}
impl Network {
    const INF: i32 = 9999;

    pub fn create(pipes: Vec<Pipe>) -> Self {
        let indexes: BTreeMap<String, usize> = pipes
            .iter()
            .enumerate()
            .map(|(index, pipe)| (pipe.valve.to_string(), index))
            .collect::<BTreeMap<_, _>>();

        let start = *indexes.get("AA").expect("Failed to get pipe");

        let mut distribution = Array2D::filled_with(Self::INF, pipes.len(), pipes.len());

        // Fill in matrix all existing pipes
        for pipe in pipes.iter() {
            distribution[(indexes[&pipe.valve], indexes[&pipe.valve])] = 0;
            for tunnel in &pipe.tunnels {
                distribution[(indexes[&pipe.valve], indexes[tunnel])] = 1;
            }
        }

        // Use Floyd-Marshall algorithm to calculate shortest path for each pair of nodes
        // See https://www.programiz.com/dsa/floyd-warshall-algorithm
        for k in 0..indexes.len() {
            for i in 0..indexes.len() {
                for j in 0..indexes.len() {
                    distribution[(i, j)] = min(
                        distribution[(i, j)],
                        distribution[(i, k)] + distribution[(k, j)],
                    );
                }
            }
        }

        // Get all valve indexes with positive flow rate & the starting position to move from.
        let open_valves = pipes
            .iter()
            .enumerate()
            .filter(|(index, pipe)| {
                (distribution[(*index, start)] < Self::INF && pipe.flow_rate > 0)
                    || (*index == start)
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let num_open_valves = open_valves.len();
        let mut interesting = Array2D::filled_with(Self::INF, num_open_valves, num_open_valves);
        let mut flow_rates: BTreeMap<usize, i32> = BTreeMap::new();
        let mut start = usize::MAX;

        // Copy the distribution of all open valves into a smaller matrix
        for y in 0..num_open_valves {
            for x in 0..num_open_valves {
                interesting[(x, y)] = distribution[(open_valves[x], open_valves[y])] + 1;
            }
            flow_rates.insert(y, pipes[open_valves[y]].flow_rate);
            // TODO check again why that is necessary
            if pipes[open_valves[y]].valve == "AA" {
                start = y;
            }
        }

        Self {
            start,
            flow_rates,
            tunnels: interesting,
        }
    }

    fn distance(&self, current: usize, next: usize) -> i32 {
        self.tunnels[(current, next)]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Worker {
    time_left: i32,
    pos: usize,
}

impl Worker {
    pub fn new(time_left: i32, pos: usize) -> Self {
        Self { time_left, pos }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    open_valves: Vec<usize>,
    workers: Vec<Worker>,
}

/// Find the best path to open valves
fn solve(current: State, network: &Network, cache: &mut HashMap<State, i32>) -> i32 {
    if let Some(value) = cache.get(&current) {
        return *value;
    }

    let mut best = 0;

    let worker = &current.workers[0];

    for next in current.open_valves.iter() {
        let remaining = current
            .open_valves
            .iter()
            .filter(|x| *x != next)
            .map(|x| *x)
            .collect_vec();

        // Get the time left with reaching the target pos.
        let time_left = worker.time_left - network.distance(worker.pos, *next);
        if time_left < 0 {
            continue;
        }

        let mut workers = current.workers.clone();
        workers[0] = Worker::new(time_left, *next);
        let next_state = State {
            open_valves: remaining,
            workers: workers,
        };

        let total = solve(next_state, network, cache) + (time_left * network.flow_rates[next]);
        best = max(best, total);
    }

    if current.workers.len() > 1 {
        let new_workers = current.workers[1..].to_vec();
        let total = solve(
            State {
                open_valves: current.open_valves.clone(),
                workers: new_workers,
            },
            network,
            cache,
        );
        best = max(best, total);
    }

    cache.insert(current, best);

    best
}

/// Determine the best path to maximume the flow of all open valves.
///
fn part1(pipes: Vec<Pipe>) -> i32 {
    let network = Network::create(pipes);
    let state = State {
        open_valves: Vec::from_iter(0..network.flow_rates.len()),
        workers: vec![Worker::new(30, network.start)],
    };
    solve(state, &network, &mut HashMap::new())
}

fn part2(pipes: Vec<Pipe>) -> i32 {
    let network = Network::create(pipes);
    let workers = std::iter::repeat_with(|| Worker::new(26, network.start))
        .take(2)
        .collect_vec();

    let state = State {
        open_valves: Vec::from_iter(0..network.flow_rates.len()),
        workers,
    };
    solve(state, &network, &mut HashMap::new())
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
    println!("Part 1: {}", part1(pipes.clone()));
    println!("Part 2: {}", part2(pipes));
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
        assert_eq!(1707, part2(parse(INPUT)));
    }
}
