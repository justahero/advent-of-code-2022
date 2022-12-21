//! Day 19: Not Enough Minerals

use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

use anyhow::anyhow;

use itertools::Itertools;

peg::parser! {
    /// Parses a blueprint in the following format. The format is the same for all.
    /// The input is a single line, the given example is split for readability only.
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

        rule number() -> i32
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule robot() -> i32
            = "Each " ("ore" / "clay") " robot costs " n:number() " ore." { n }

        rule obsidian() -> (i32, i32)
            = "Each obsidian robot costs " ore:number() " ore and " clay:number() " clay." { (ore, clay) }

        rule geode() -> (i32, i32)
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
    pub robots: [i32; 4],
    pub ores: [i32; 4],
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
    pub fn can_build(&self, costs: &[i32; 4]) -> bool {
        self.ores[0] >= costs[0]
            && self.ores[1] >= costs[1]
            && self.ores[2] >= costs[2]
            && self.ores[3] >= costs[3]
    }

    /// Advances the state
    ///
    /// * first the costs are spent
    /// * then all robots dig their resources, time increases
    /// * add the robot
    pub fn build(&self, costs: &[i32; 4], robot: Mineral) -> Self {
        let ores = [
            self.ores[0] - costs[0] + self.robots[0],
            self.ores[1] - costs[1] + self.robots[1],
            self.ores[2] - costs[2] + self.robots[2],
            self.ores[3] - costs[3] + self.robots[3],
        ];
        let mut robots = self.robots.clone();
        robots[robot as usize] += 1;
        Self {
            ores,
            robots,
            time: self.time + 1,
        }
    }

    /// Advances the state by one, all robots dig for their material
    pub fn dig(&mut self) {
        self.ores[0] += self.robots[0];
        self.ores[1] += self.robots[1];
        self.ores[2] += self.robots[2];
        self.ores[3] += self.robots[3];
        self.time += 1;
    }

    #[inline]
    pub fn ore(&self, mineral: Mineral) -> i32 {
        self.ores[mineral as usize]
    }

    #[inline]
    pub fn robot(&self, mineral: Mineral) -> i32 {
        self.robots[mineral as usize]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Blueprint {
    id: u32,
    robot_costs: [[i32; 4]; 4],
}

impl Blueprint {
    pub fn new(id: u32, ore: i32, clay: i32, obsidian: (i32, i32), geode: (i32, i32)) -> Self {
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
    pub fn geodes(&self, minutes: u16) -> i32 {
        println!("Blueprint: {}", self.id());

        let state = State::default();
        let mut best_geodes = 0;

        let mut states = VecDeque::new();
        let mut visited_states = HashSet::new();

        states.push_back(state.clone());

        let max_ores = self.robot_costs.iter().map(|c| c[0]).max().unwrap();

        while let Some(state) = states.pop_front() {
            best_geodes = max(best_geodes, state.ore(Mineral::Geode));

            // simple heuristc that works for limited number of minutes
            if state.ore(Mineral::Geode) < best_geodes - 2 || visited_states.contains(&state) {
                continue;
            }

            // This state ends
            if state.time >= minutes {
                continue;
            }

            visited_states.insert(state.clone());

            // Check if there is anything that can be built
            if state.can_build(self.costs(Mineral::Geode)) {
                states.push_back(state.build(self.costs(Mineral::Geode), Mineral::Geode));
            }

            if state.can_build(self.costs(Mineral::Obsidian)) {
                states.push_back(state.build(self.costs(Mineral::Obsidian), Mineral::Obsidian));
            }

            if state.can_build(self.costs(Mineral::Clay)) {
                states.push_back(state.build(self.costs(Mineral::Clay), Mineral::Clay));
            }

            if state.can_build(self.costs(Mineral::Ore)) && state.robot(Mineral::Ore) < max_ores {
                states.push_back(state.build(self.costs(Mineral::Ore), Mineral::Ore));
            }

            // only when we have limited resources and nothing can be built allow robots to dig
            if state.robot(Mineral::Ore) < max_ores {
                let mut next_state = state.clone();
                next_state.dig();
                states.push_back(next_state);
            }
        }

        best_geodes
    }

    /// Returns the mineral costs for a specific robot
    fn costs(&self, mineral: Mineral) -> &[i32; 4] {
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

fn part1(blueprints: Vec<Blueprint>) -> u32 {
    blueprints
        .iter()
        .map(|blueprint| blueprint.geodes(24) as u32 * blueprint.id() as u32)
        .sum::<u32>()
}

fn part2(blueprints: Vec<Blueprint>) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.geodes(32) as u32)
        .product()
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
    println!("Part 1: {}", part1(blueprints.clone()));
    println!("Part 2: {}", part2(blueprints.clone()));
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
        assert_eq!(56, blueprint.geodes(32));
    }

    #[test]
    fn check_blueprint_geodes_2nd() {
        let input = "Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.";
        let blueprint = Blueprint::try_from(input).expect("Failed to parse blueprint");
        assert_eq!(12, blueprint.geodes(24));
        assert_eq!(62, blueprint.geodes(32));
    }

    #[test]
    fn check_part1() {
        assert_eq!(33, part1(parse(INPUT)));
    }
}
