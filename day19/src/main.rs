//! Day 19: Not Enough Minerals

use std::collections::{HashSet, VecDeque};

use anyhow::anyhow;

use itertools::Itertools;

peg::parser! {
    /// Parses a blueprint in the following format. The format is the same for all.
    ///
    /// Blueprint 1:
    ///   Each ore robot costs 3 ore.
    ///   Each clay robot costs 4 ore.
    ///   Each obsidian robot costs 4 ore and 18 clay.
    ///   Each geode robot costs 3 ore and 8 obsidian.
    ///
    grammar line_parser() for str {
        rule ws()
            = " "

        rule number() -> u16
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule robot() -> u16
            = "Each " ("ore" / "clay") " robot costs " n:number() " ore." { n }

        rule obsidian() -> (u16, u16)
            = "Each obsidian robot costs " ore:number() " ore and " clay:number() " clay." { (ore, clay) }

        rule geode() -> (u16, u16)
            = "Each geode robot costs " ore:number() " ore and " obsidian:number() " obsidian." { (ore, obsidian) }

        rule id() -> u32
            = "Blueprint " id:number() ":" { id as u32 }

        pub(crate) rule blueprint() -> Blueprint
            = id:id() ws() ore:robot() ws() clay:robot() ws() obsidian:obsidian() ws() geode:geode()
            {
                Blueprint::new(id, ore, clay, obsidian, geode)
            };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Mineral {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}

/// The state representing a point in time.
///
/// The list of robots & minerals: ore, clay, obsidian, geodes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pub robots: [u16; 4],
    pub ores: [u16; 4],
    pub time: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            ores: [0, 0, 0, 0],
            time: 0,
        }
    }
}

impl State {
    pub fn new(time: u16, ores: [u16; 4], robots: [u16; 4]) -> Self {
        Self { robots, ores, time }
    }

    pub fn can_build(&self, costs: &[u16; 4]) -> bool {
        self.ores[0] >= costs[0]
            && self.ores[1] >= costs[1]
            && self.ores[2] >= costs[2]
            && self.ores[3] >= costs[3]
    }

    pub fn spend(&mut self, costs: &[u16; 4]) {
        self.ores[0] -= costs[0];
        self.ores[1] -= costs[1];
        self.ores[2] -= costs[2];
        self.ores[3] -= costs[3];
    }

    pub fn next_state(&self) -> Self {
        let ores = [
            self.ores[0] + self.robots[0],
            self.ores[1] + self.robots[1],
            self.ores[2] + self.robots[2],
            self.ores[3] + self.robots[3],
        ];
        Self {
            robots: self.robots.clone(),
            ores,
            time: self.time + 1,
        }
    }

    pub fn ore(&self, mineral: Mineral) -> u16 {
        let index = mineral as u8;
        self.ores[index as usize]
    }

    pub fn robot_mut(&mut self, mineral: Mineral) -> &mut u16 {
        let index = mineral as u8;
        &mut self.robots[index as usize]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    robot_costs: [[u16; 4]; 4],
}

impl Blueprint {
    pub fn new(id: u32, ore: u16, clay: u16, obsidian: (u16, u16), geode: (u16, u16)) -> Self {
        let costs = [
            [ore, 0, 0, 0],
            [clay, 0, 0, 0],
            [obsidian.0, obsidian.1, 0, 0],
            [geode.0, 0, geode.1, 0],
        ];

        Self {
            id,
            robot_costs: costs,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Calculates the optimal number of geodes that can be opened in a given number of minutes
    ///
    /// * get ores with "ore robot" -> can build "clay robot"
    /// * get clay with "clay robot"
    /// * get "obsidian robot" with ore & clay
    /// * get "geode robot" with ore & obsidian
    ///
    /// Blueprint 1 - Example:
    ///   Each ore robot costs 3 ore.
    ///   Each clay robot costs 4 ore.
    ///   Each obsidian robot costs 4 ore and 18 clay.
    ///   Each geode robot costs 3 ore and 8 obsidian.
    ///
    pub fn geodes(&self, minutes: u16) -> u16 {
        println!("-- Blueprint {}", self.id);

        // beginning state, one "ore robot", no ores
        let state = State::default();
        let mut best_geodes = 0;

        let mut states = VecDeque::new();
        let mut visited_states = HashSet::new();

        states.push_back(state.clone());

        while let Some(state) = states.pop_front() {
            // println!("Current state: {:?}", state);

            best_geodes = std::cmp::max(best_geodes, state.ore(Mineral::Geode));
            if state.ore(Mineral::Geode) < best_geodes || visited_states.contains(&state) {
                continue;
            }

            // This state ends
            if state.time >= minutes {
                continue;
            }

            visited_states.insert(state.clone());

            // Check if there is anything that can be built, from most expensive to cheapest
            if state.can_build(self.costs(Mineral::Geode)) {
                let mut next_state = state.next_state();
                next_state.spend(self.costs(Mineral::Geode));
                *next_state.robot_mut(Mineral::Geode) += 1;
                states.push_back(next_state);
            } else {
                if state.can_build(self.costs(Mineral::Obsidian)) {
                    let mut next_state = state.next_state();
                    next_state.spend(self.costs(Mineral::Obsidian));
                    *next_state.robot_mut(Mineral::Obsidian) += 1;
                    states.push_back(next_state);
                }

                if state.can_build(self.costs(Mineral::Clay)) {
                    let mut next_state = state.next_state();
                    next_state.spend(self.costs(Mineral::Clay));
                    *next_state.robot_mut(Mineral::Clay) += 1;
                    states.push_back(next_state);
                }

                if state.can_build(self.costs(Mineral::Ore)) {
                    let mut next_state = state.next_state();
                    next_state.spend(self.costs(Mineral::Ore));
                    *next_state.robot_mut(Mineral::Ore) += 1;
                    states.push_back(next_state);
                }
            }

            // otherwise continue to dig with current robots
            states.push_back(state.next_state());
        }

        best_geodes
    }

    /// Returns the mineral costs for a specific robot
    fn costs(&self, mineral: Mineral) -> &[u16; 4] {
        let index = mineral as usize;
        &self.robot_costs[index]
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        line_parser::blueprint(input).map_err(|_| anyhow!("Failed to parse '{}'", input))
    }
}

fn part1(blueprints: Vec<Blueprint>) -> u16 {
    let minutes = 24;
    blueprints
        .iter()
        .map(|blueprint| blueprint.geodes(minutes) * blueprint.id() as u16)
        .max()
        .expect("Failed to get max value")
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(str::trim)
        .filter_map(|line| Blueprint::try_from(line).ok())
        .collect_vec()
}

fn main() {
    let blueprints = parse(include_str!("input.txt"));
    println!("Part 1: {}", part1(blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test.txt");

    #[test]
    fn check_blueprint_parser() {
        let input = "Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.";

        assert_eq!(
            Ok(Blueprint {
                id: 1,
                robot_costs: [[4, 0, 0, 0], [2, 0, 0, 0], [3, 14, 0, 0], [2, 0, 7, 0]],
            }),
            line_parser::blueprint(input),
        );
    }

    #[test]
    fn check_blueprint_geodes() {
        let input = "Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.";
        let blueprint = Blueprint::try_from(input).expect("Failed to parse blueprint");
        assert_eq!(9, blueprint.geodes(24));
    }

    #[test]
    fn advance_next_state() {
        assert_eq!(
            State::new(1, [1, 0, 0, 0], [1, 0, 0, 0]),
            State::new(0, [0, 0, 0, 0], [1, 0, 0, 0]).next_state(),
        );
        assert_eq!(
            State::new(3, [9, 6, 4, 2], [4, 3, 2, 1]),
            State::new(1, [1, 0, 0, 0], [4, 3, 2, 1])
                .next_state()
                .next_state(),
        );
    }

    #[test]
    fn check_part1() {
        assert_eq!(12, part1(parse(INPUT)));
    }

    #[ignore]
    #[test]
    fn check_part2() {
        // assert_eq!(58, part2(parse(INPUT)));
    }
}
