use std::collections::BTreeMap;

use pathfinding::prelude::dijkstra_all;

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
fn open_valves(pipes: Vec<Pipe>, num_rounds: i32) -> usize {
    // Look up table
    let mut pipes_by_label: BTreeMap<String, Pipe> = pipes
        .into_iter()
        .map(|pipe| (pipe.valve.to_string(), pipe))
        .collect::<BTreeMap<_, _>>();

    let mut current_valve = "AA".to_string();
    let mut current_steps = 0;
    let mut total_flow: i32 = 0;

    // play all rounds
    let mut round = 0;
    while round < num_rounds {
        println!("ROUND: {}", round);

        // calculcate flow since last step
        total_flow += pipes_by_label
            .values()
            .filter(|pipe| pipe.open)
            .inspect(|pipe| println!("PIPE: {:?}", pipe))
            .map(|pipe| pipe.flow_rate)
            .inspect(|flow| println!("FLOW: {}", flow))
            .sum::<i32>();

        println!("TOTAL: {}", total_flow);

        current_steps -= 1;

        // TODO update the pipe!!!
        pipes_by_label.get_mut(&current_valve).unwrap().open = true;

        // currently on the way to new valve
        if current_steps == 0 {
            // evaluate current network of open valves
            let all_paths = dijkstra_all(&current_valve.clone(), |valve| {
                let mut valves: Vec<(String, i32)> = Vec::new();
                let current_pipe = pipes_by_label.get(valve).expect("Failed to get pipe");

                for tunnel in &current_pipe.tunnels {
                    let pipe = pipes_by_label.get(tunnel).expect("Failed to get successor");
                    valves.push((pipe.valve.to_string(), 1));
                }

                valves
            });

            // find the best possible candidate that provides the highest flow
            let next_valve = all_paths
                .iter()
                .filter_map(|(key, (_valve, steps))| {
                    let pipe = pipes_by_label.get(key).unwrap();
                    if !pipe.open {
                        let flow_rate = (num_rounds - round - steps).max(0) * pipe.flow_rate;
                        Some((key, flow_rate, steps))
                    } else {
                        None
                    }
                })
                .max_by(|left, right| left.1.cmp(&right.1));

            // when there is still one valve left, advance
            if let Some((next_valve, _, steps)) = next_valve {
                current_valve = next_valve.clone();
                current_steps = *steps;
            }
        }

        round += 1;
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
