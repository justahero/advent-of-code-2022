peg::parser! {
    grammar line_parser() for str {
        rule valve() -> String
            = s:$(['A'..='Z']+) { s.to_string() }

        rule rate() -> u32
            = n:$(['0'..='9']+) { n.parse::<u32>().unwrap() }

        rule tunnels() -> Vec<String>
            = valves:(valve() ** ", ") { valves }

        pub(crate) rule pipe() -> Pipe
            = "Valve " valve:valve() " has flow rate=" rate:rate() "; tunnels lead to valves " tunnels:tunnels()
            {
                Pipe {
                    valve,
                    flow_rate: rate,
                    tunnels,
                }
            }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Valve(String);

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    valve: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

fn part1(pipes: Vec<Pipe>) -> usize {
    0
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
            }),
            line_parser::pipe("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
        );
    }

    #[test]
    fn check_part1() {
        // assert_eq!(26, part1(parse(INPUT), 10));
    }

    #[test]
    fn check_part2() {
        // assert_eq!(56_000_011, part2(parse(INPUT), 20));
    }
}
