//! Day 19: Not Enough Minerals

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

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("Failed to parse number")) }

        rule robot() -> u32
            = "Each " ("ore" / "clay") " robot costs " n:number() " ore." { n }

        rule obsidian() -> (u32, u32)
            = "Each obsidian robot costs " ore:number() " ore and " clay:number() " clay." { (ore, clay) }

        rule geode() -> (u32, u32)
            = "Each geode robot costs " ore:number() " ore and " obsidian:number() " obsidian." { (ore, obsidian) }

        rule id() -> u32
            = "Blueprint " id:number() ":" { id }

        pub(crate) rule blueprint() -> Blueprint
            = id:id() ws() ore:robot() ws() clay:robot() ws() obsidian:obsidian() ws() geode:geode()
            {
                Blueprint::new(id, ore, clay, obsidian, geode)
            };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

impl Blueprint {
    pub fn new(id: u32, ore: u32, clay: u32, obsidian: (u32, u32), geode: (u32, u32)) -> Self {
        Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Calculates the optimal number of geodes that can be opened by given number of minutes
    pub fn geodes(&self, minutes: u32) -> u32 {
        0
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

fn part1(blueprints: Vec<Blueprint>) -> u32 {
    let minutes = 24;
    blueprints
        .iter()
        .map(|blueprint| blueprint.geodes(minutes) * blueprint.id())
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
    // println!("Part 1: {}", part1(cubes.clone()));
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
                ore: 4,
                clay: 2,
                obsidian: (3, 14),
                geode: (2, 7)
            }),
            line_parser::blueprint(input),
        );
    }

    #[test]
    fn check_part1() {
        // assert_eq!(64, part1(parse(INPUT)));
        assert_eq!(12, part1(parse(INPUT)));
    }

    #[ignore]
    #[test]
    fn check_part2() {
        // assert_eq!(58, part2(parse(INPUT)));
    }
}
